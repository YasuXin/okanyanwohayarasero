use std::cmp::{max, min};
use std::fmt::Debug;
use std::rc::Rc;
use crate::beam::Beam;
use crate::enemy::{Dead, Enemy, EnemyContext, Kinds, Live};
use crate::engine::{Audio, Cell, Point, Rect, Renderer, Sound, SpriteSheet};
use crate::player::{Player, PlayerStateMachine};

//--------------------------------------------
// 衝突すると、ゲームオーバーになるビームを定義する
//--------------------------------------------

const ENEMY_BEAM: &str = "beam_0.png";

#[derive(Debug, Clone)]
pub struct EnemyBeam {
	pub state_machine: EnemyStateMachine
}

impl Enemy for EnemyBeam {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Beam
	}

	fn pos_x(&self) -> f32 {
		self.state_machine.context().position.x
	}

	fn pos_y(&self) -> f32 {
		self.state_machine.context().position.y
	}

	fn update(&mut self, player: &mut Player, _beams: &Vec<Beam>) {
		self.state_machine = self.state_machine.clone().update(player);
	}

	fn draw(&self, renderer: &Renderer) {
		self.state_machine.draw(renderer);
	}

	fn be_defeated(&mut self) {
		self.state_machine.be_defeated();
	}
}

impl EnemyBeam {
	fn new(
		sheet: Rc<SpriteSheet>,
		sprite_name: &str,
		position: Point,
		center: Point,
		velocity: Point,
		attack_radius: f32,
		defeated_radius: f32,
		audio: [Rc<Audio>; 1],
		sound: [Rc<Sound>; 1],
		frame: i16
	) -> Self {

		// cloned()は、Option<&T> をクローンして Option<T> にして返す
		let sprite = sheet.cell(sprite_name).cloned().expect("Cell is not exists");

		let context = EnemyContext {
			sheet,
			sprite,
			position,
			center,
			velocity,
			attack_radius,
			defeated_radius,
			frame,
			defeated: false
		};

		EnemyBeam {
			state_machine: EnemyStateMachine::Live(
				EnemyState {
					context,
					audio_context: AudioContext {audio, sound},
					_state: Live
				}
			),
		}

	}
}

#[derive(Debug, Clone)]
pub enum EnemyStateMachine {
	Live(EnemyState<Live>),
	Dead(EnemyState<Dead>)
}

impl EnemyStateMachine {

	pub fn update(self, player: &mut Player) -> Self {
		match self.clone() {
			EnemyStateMachine::Live(state) => state.update(player).into(),
			EnemyStateMachine::Dead(state) => state.update().into(),
		}
	}

	pub fn context(&self) -> &EnemyContext {
		match self {
			EnemyStateMachine::Live(state) => state.context(),
			EnemyStateMachine::Dead(state) => state.context(),
		}
	}

	pub fn draw(&self, renderer: &Renderer) {
		match self {
			EnemyStateMachine::Live(state) => state.draw(renderer),
			EnemyStateMachine::Dead(state) => state.draw(renderer),
		}
	}

	pub fn be_defeated(&mut self) {
		match self {
			EnemyStateMachine::Live(state) => state.context.defeated = true,
			EnemyStateMachine::Dead(_state) => {},
		}
	}
}

#[derive(Debug, Clone)]
pub struct EnemyState<S> {
	pub context: EnemyContext,
	pub audio_context: AudioContext,
	pub _state: S
}

impl<S> EnemyState<S> {

	fn context(&self) -> &EnemyContext {
		&self.context
	}

	pub fn draw(&self, renderer: &Renderer) {

		let sprite = &self.context.sprite;

		let mut size: f32 = 1.5;
		if self.context.defeated {
			size = 1.0;
		}
		self.context.sheet.draw(
			renderer,
			&Rect::new_from_x_y(
				sprite.frame.x,
				sprite.frame.y,
				sprite.frame.w,
				sprite.frame.h
			),
			&Rect::new_from_x_y(
				self.context.position.x,
				self.context.position.y,
				sprite.frame.w / size,
				sprite.frame.h / size
			)
		);
	}

	pub fn set_sprite(self, sprite_name: String) -> Cell {
		self.context
			.sheet
			.cell(&sprite_name)
			.cloned()
			.expect("Cell is not exists")
	}

	pub fn play_sound(&self) {
		let audio = &self.audio_context.audio[0];
		let sound = &self.audio_context.sound[0];
		
		if let Err(err) = audio.play_sound(&sound) {
			log!("Error playing beam_sound: {}", err);
		}
	}

}

impl EnemyState<Live> {

	pub fn be_defeated(&mut self, player: &mut Player) -> EnemyState<Dead> {
		self.context.frame = 0;
		self.context.position.x = 2000.0;
		self.context.position.y = 2000.0;
		self.context.center.x = 2015.0;
		self.context.center.y = 2015.0;
		player.set_add_points(-50);

		EnemyState {
			context: self.context.clone(),
			audio_context: self.audio_context.clone(),
			_state: Dead
		}
	}

	pub fn update(mut self, player: &mut Player) -> LiveEndState {

		if self.context.defeated {

			LiveEndState::Complete(self.be_defeated(player))

		} else {

			self.context.frame += 1;

			match self.context.frame {
				0..250 => {}, // 何もしない
				250 => {
					self.clone().play_sound();
					self.context.position.x = -50.0
				},
				252 => self.context.sprite = self.clone().set_sprite("beam_1.png".to_string()),
				255 => self.context.sprite = self.clone().set_sprite("beam_2.png".to_string()),
				258 => self.context.sprite = self.clone().set_sprite("beam_3.png".to_string()),
				261 => self.context.sprite = self.clone().set_sprite("beam_4.png".to_string()),
				264 => self.context.sprite = self.clone().set_sprite("beam_5.png".to_string()),
				335 => self.context.sprite = self.clone().set_sprite("beam_4.png".to_string()),
				338 => self.context.sprite = self.clone().set_sprite("beam_3.png".to_string()),
				341 => self.context.sprite = self.clone().set_sprite("beam_2.png".to_string()),
				344 => self.context.sprite = self.clone().set_sprite("beam_1.png".to_string()),
				347 => self.context.sprite = self.clone().set_sprite("beam_0.png".to_string()),
				351 => self.context.position.x = -2000.0,
				_ => {}
			}

			match self.context.frame {
				261..340 => self.check_intersection(player),
				_ => {}
			}

			if self.context.frame >= 351 {
				LiveEndState::Complete(self.be_defeated(player))
			} else {
				LiveEndState::Continue(self)
			}
		}
	}

	pub fn check_intersection(&self, player: &mut Player) {
		// intersects()メソッドで、playerとObstacleが衝突しているかチェックする、

		match player.state_machine {
			PlayerStateMachine::Shielding(_) => {}
			PlayerStateMachine::KnockedOut(_) => {}
			_ => {
				let rect_x = self.context.position.x as i32;
				let rect_y = self.context.position.y as i32;
				let rect_width = ( 1600.0 / 1.5 ) as i32;
				let rect_height = ( 66.0 / 1.5 ) as i32;

				let player_center_x = player.center_x() as i32;
				let player_center_y = player.center_y() as i32;

				let player_radius = player.radius() as i32;

				let closest_x = max(rect_x, min(player_center_x, rect_x + rect_width));
				let closest_y = max(rect_y, min(player_center_y, rect_y + rect_height));

				let distance_x = player_center_x - closest_x;
				let distance_y = player_center_y - closest_y;

				let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

				if distance_squared < (player_radius * player_radius) {

					player.knock_out();

				}
			}
		}
	}

}

impl EnemyState<Dead> {
	fn update(self) -> Self {

		self
	}
}

#[derive(Debug, Clone)]
pub struct AudioContext {
	pub audio: [Rc<Audio>; 1],
	pub sound: [Rc<Sound>; 1],
}

impl From<EnemyState<Live>> for EnemyStateMachine {
	fn from(state: EnemyState<Live>) -> Self {
		EnemyStateMachine::Live(state)
	}
}

impl From<EnemyState<Dead>> for EnemyStateMachine {
	fn from(state: EnemyState<Dead>) -> Self {
		EnemyStateMachine::Dead(state)
	}
}

#[derive(Debug, Clone)]
pub enum LiveEndState {
	Complete(EnemyState<Dead>),
	Continue(EnemyState<Live>),
}


impl From<LiveEndState> for EnemyStateMachine {
	fn from(end_state: LiveEndState) -> Self {
		match end_state {
			LiveEndState::Complete(state) => state.into(),
			LiveEndState::Continue(state) => state.into(),
		}
	}
}


pub fn create_enemy_beam(
	sprite_sheet: Rc<SpriteSheet>,
	position_y: i16,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	width: f32
) -> EnemyBeam {

	EnemyBeam::new(
		sprite_sheet,
		ENEMY_BEAM,
		Point {
			x: width + 400.0,
			y: ( position_y + 27 ) as f32
		},
		Point {
			x: width + 850.0,
			y: ( position_y + 65 ) as f32
		},
		Point {
			x: 0.0,
			y: 0.0,
		},
		0.0,
		0.0,
		audio,
		sound,
		0
	)
}
