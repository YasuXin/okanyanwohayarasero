use std::fmt::Debug;
use std::rc::Rc;
use crate::beam::Beam;
use crate::enemy::{Dead, Enemy, EnemyContext, Kinds, Live};
use crate::engine::{Audio, Point, Rect, Renderer, Sound, SpriteSheet};
use crate::player::{Player, PlayerStateMachine};

//--------------------------------------------
// 衝突すると、ゲームオーバーになるエネルギー弾を定義する
//--------------------------------------------

const ENEMY_ENERGY: &str = "beam_10.png";

#[derive(Debug, Clone)]
pub struct EnemyEnergy {
	pub state_machine: EnemyStateMachine
}

impl Enemy for EnemyEnergy {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Energy
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

impl EnemyEnergy {
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

		EnemyEnergy {
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

			if self.context.frame == 250 {

				self.clone().play_sound();

				self.context.position.x = 970.0;
				self.context.center.x = 1002.0

			} else if self.context.frame > 250 {

				self.context.position.x += self.context.velocity.x;
				self.context.position.y += self.context.velocity.y;
				self.context.center.x += self.context.velocity.x;
				self.context.center.y += self.context.velocity.y;
				
				if self.context.frame% 2 == 0 {
					self.check_intersection(player);
				}
			}
			
			LiveEndState::Continue(self)
		}
	}

	pub fn check_intersection(&self, player: &mut Player) {
		// intersects()メソッドで、playerとObstacleが衝突しているかチェックする、

		match player.state_machine {
			PlayerStateMachine::Shielding(_) => {}
			PlayerStateMachine::KnockedOut(_) => {}
			_ => {
				// intersects()メソッドで、playerとEnemyが衝突しているかチェックする、
				let enemy_x = self.context.center.x as i32;
				let enemy_y = self.context.center.y as i32;

				let player_center_x = player.center_x() as i32;
				let player_center_y = player.center_y() as i32;

				let player_radius = player.radius();

				let distance_x = enemy_x - player_center_x;
				let distance_y = enemy_y - player_center_y;

				let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

				let radius_sum = (player_radius + self.context.attack_radius) as i32;
				let radius_sum_squared = radius_sum * radius_sum;

				if distance_squared <= radius_sum_squared {

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

pub fn create_enemy_energy(
	sprite_sheet: Rc<SpriteSheet>,
	position_y: i16,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	velocity: Point,
	frame: i16,
	width: f32,
) -> EnemyEnergy {

	EnemyEnergy::new(
		sprite_sheet,
		ENEMY_ENERGY,
		Point {
			x: width + 400.0,
			y: ( position_y ) as f32
		},
		Point {
			x: width + 433.0,
			y: ( position_y + 33 ) as f32
		},
		velocity,
		20.0,
		0.0,
		audio,
		sound,
		frame
	)
}