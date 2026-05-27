use std::cmp::{max, min};
use std::fmt::Debug;
use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::beam::Beam;
use crate::enemy::{BeingDefeated, Dead, Enemy, EnemyContext, Kinds, Live};
use crate::engine::{Audio, Cell, Point, Rect, Renderer, Sound, SpriteSheet};
use crate::player::{Player, PlayerStateMachine};

//--------------------------------------------
// 衝突すると、ゲームオーバーになるロケットを定義する
//--------------------------------------------


const ENEMY_0: &str = "onigirya_0.png";
const ENEMY_1: &str = "onigirya_1.png";
const ENEMY_DEFEATED_0: &str = "defeated_0.png";

const ENEMY_DEFEATED_FRAME_NAME: &str = "defeated";
const ENEMY_DEFEATED_FRAMES: i16 = 5;

#[derive(Debug, Clone)]
pub struct EnemyRocket {
	pub state_machine: EnemyStateMachine
}

impl Enemy for EnemyRocket {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Rocket
	}

	fn pos_x(&self) -> f32 {
		self.state_machine.context().position.x
	}

	fn pos_y(&self) -> f32 {
		self.state_machine.context().position.y
	}

	fn update(&mut self, player: &mut Player, beams: &Vec<Beam>) {
		self.state_machine = self.state_machine.clone().update(player, beams);
	}

	fn draw(&self, renderer: &Renderer) {
		self.state_machine.draw(renderer);
	}

	fn be_defeated(&mut self) {
		self.state_machine.be_defeated();
	}
}

impl EnemyRocket {
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

		let audio_context = AudioContext {audio, sound};

		EnemyRocket {
			state_machine: EnemyStateMachine::Live(
				EnemyState {
					context,
					audio_context,
					_state: Live
				}
			),
		}

	}

}
#[derive(Debug, Clone)]
pub enum EnemyStateMachine {
	Live(EnemyState<Live>),
	BeingDefeated(EnemyState<BeingDefeated>),
	Dead(EnemyState<Dead>)
}

impl EnemyStateMachine {

	pub fn update(self, player: &mut Player, beams: &Vec<Beam>) -> Self {
		match self.clone() {
			EnemyStateMachine::Live(state) => state.update(player, beams).into(),
			EnemyStateMachine::BeingDefeated(state) => state.update().into(),
			EnemyStateMachine::Dead(state) => state.update().into(),
		}
	}

	pub fn context(&self) -> &EnemyContext {
		match self {
			EnemyStateMachine::Live(state) => state.context(),
			EnemyStateMachine::BeingDefeated(state) => state.context(),
			EnemyStateMachine::Dead(state) => state.context(),
		}
	}

	pub fn draw(&self, renderer: &Renderer) {
		match self {
			EnemyStateMachine::Live(state) => state.draw(renderer),
			EnemyStateMachine::BeingDefeated(state) => state.draw(renderer),
			EnemyStateMachine::Dead(state) => state.draw(renderer),
		}
	}

	pub fn be_defeated(&mut self) {
		match self {
			EnemyStateMachine::Live(state) => state.context.defeated = true,
			EnemyStateMachine::BeingDefeated(_state) => {}
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
			log!("Error playing defeated_sound: {}", err);
		}
	}

}

impl EnemyState<Live> {

	fn check_intersection(mut self, player: &mut Player, beams: &Vec<Beam>) -> LiveEndState {

		// intersects()メソッドで、playerとEnemyが衝突しているかチェックする、
		let enemy_x = self.context.center.x as i32;
		let enemy_y = self.context.center.y as i32;
		
		if beams.iter().any(|beam| {
			let beam_x = ( beam.bounding_box.position.x + beam.center.x - beam.bounding_box.width / 2.0 ) as i32;
			let beam_y = ( beam.bounding_box.position.y + beam.center.y - beam.bounding_box.height / 2.0 ) as i32;
			let beam_w = beam.bounding_box.width as i32;
			let beam_h = beam.bounding_box.height as i32;
			
			let enemy_radius = self.context.defeated_radius as i32;

			let closest_x = max(beam_x, min(enemy_x, beam_x + beam_w));
			let closest_y = max(beam_y, min(enemy_y, beam_y + beam_h));

			let distance_x = enemy_x - closest_x;
			let distance_y = enemy_y - closest_y;

			let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

			if distance_squared < (enemy_radius * enemy_radius) {

				true
			} else {

				false
			}
		}) {
			LiveEndState::Complete(self.be_defeated(player))
		} else {
			let player_center_x = player.center_x() as i32;
			let player_center_y = player.center_y() as i32;

			let player_radius = player.radius();

			let distance_x = enemy_x - player_center_x;
			let distance_y = enemy_y - player_center_y;

			let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

			match player.state_machine {

				PlayerStateMachine::Slashing(_) => {

					let radius_sum = (80.0 + self.context.defeated_radius) as i32;
					let radius_sum_squared = radius_sum * radius_sum;

					if distance_squared <= radius_sum_squared {
						// エネミー消滅処理

						LiveEndState::Complete(self.be_defeated(player))

					} else {
						LiveEndState::Continue(self)
					}
				}

				PlayerStateMachine::Shielding(_) => LiveEndState::Continue(self),

				_ => {

					let radius_sum = (player_radius + self.context.attack_radius) as i32;
					let radius_sum_squared = radius_sum * radius_sum;

					if distance_squared <= radius_sum_squared && !self.context.defeated {

						player.knock_out();
					}

					LiveEndState::Continue(self)
				}
			}
		}
	}
	
	pub fn be_defeated(&mut self, player: &mut Player) -> EnemyState<BeingDefeated> {
		self.context.frame = 0;
		self.play_sound();
		self.context.position.x -= 30.0;
		self.context.position.y -= 30.0;
		self.context.center.x -= 15.0;
		self.context.center.y -= 15.0;
		self.context.sprite = self.clone().set_sprite(ENEMY_DEFEATED_0.to_string());
		player.set_add_points(-50);

		EnemyState {
			context: self.context.clone(),
			audio_context: self.audio_context.clone(),
			_state: BeingDefeated
		}
	}

	pub fn update(mut self, player: &mut Player, beams: &Vec<Beam>) -> LiveEndState {

		if self.clone().context.defeated {

			LiveEndState::Complete(self.be_defeated(player))

		} else {

			self.context.frame += 1;

			self.context.position.x += self.context.velocity.x;
			self.context.position.y += self.context.velocity.y;
			self.context.center.x += self.context.velocity.x;
			self.context.center.y += self.context.velocity.y;

			self.check_intersection(player, beams)
		}
	}
}

impl EnemyState<BeingDefeated> {

	pub fn update(mut self) -> BeingDefeatedEndState {

		self.context.frame += 1;

		if self.context.frame > ENEMY_DEFEATED_FRAMES {

			self.context.position.x = -1000.0;
			self.context.center.x = -1000.0;

			BeingDefeatedEndState::Complete(self.die())

		} else {

			self.context.sprite = self.clone().set_sprite(
				format!(
					"{0}_{1}.png",
					ENEMY_DEFEATED_FRAME_NAME,
					self.context.frame
				)
			);

			BeingDefeatedEndState::Continue(self)
		}

	}

	fn die(self) -> EnemyState<Dead> {
		EnemyState {
			context: self.context,
			audio_context: self.audio_context,
			_state: Dead
		}
	}
}

impl EnemyState<Dead> {
	fn update(mut self) -> Self {

		self.context.position.x = -1000.0;
		self.context.center.x = -1000.0;

		self
	}
}

#[derive(Debug, Clone)]
pub struct AudioContext {
	pub audio: [Rc<Audio>; 1],
	pub sound: [Rc<Sound>; 1]
}

impl From<EnemyState<Live>> for EnemyStateMachine {
	fn from(state: EnemyState<Live>) -> Self {
		EnemyStateMachine::Live(state)
	}
}

impl From<EnemyState<BeingDefeated>> for EnemyStateMachine {
	fn from(state: EnemyState<BeingDefeated>) -> Self {
		EnemyStateMachine::BeingDefeated(state)
	}
}

impl From<EnemyState<Dead>> for EnemyStateMachine {
	fn from(state: EnemyState<Dead>) -> Self {
		EnemyStateMachine::Dead(state)
	}
}

#[derive(Debug, Clone)]
pub enum LiveEndState {
	Complete(EnemyState<BeingDefeated>),
	Continue(EnemyState<Live>),
}

#[derive(Debug, Clone)]
pub enum BeingDefeatedEndState {
	Complete(EnemyState<Dead>),
	Continue(EnemyState<BeingDefeated>),
}

impl From<LiveEndState> for EnemyStateMachine {
	fn from(end_state: LiveEndState) -> Self {
		match end_state {
			LiveEndState::Complete(state) => state.into(),
			LiveEndState::Continue(state) => state.into(),
		}
	}
}

impl From<BeingDefeatedEndState> for EnemyStateMachine {
	fn from(end_state: BeingDefeatedEndState) -> Self {
		match end_state {
			BeingDefeatedEndState::Complete(state) => state.into(),
			BeingDefeatedEndState::Continue(state) => state.into(),
		}
	}
}

pub fn create_rocket_0(
	sprite_sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	velocity_x: f32,
	width: f32,
	height: f32
) -> EnemyRocket {

	let rng_x = (width / 5.6) as i16;
	let mut rng = thread_rng();
	let position_x = rng.gen_range(0..rng_x);
	let speed_y = rng.gen_range(2..5);

	EnemyRocket::new(
		sprite_sheet.clone(),
		ENEMY_0,
		Point {
			x: (position_x + 500) as f32,
			y: height
		},
		Point {
			x: (position_x + 533) as f32,
			y: height + 28.0
		},
		Point {
			x: -velocity_x,
			y: -speed_y as f32,
		}, 20.0,
		30.0,
		audio,
		sound,
		0
	)
}

pub fn create_rocket_1(
	sprite_sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	velocity_x: f32,
	width: f32,
	height: f32
) -> EnemyRocket {
	let rng_y = (height - 160.0) as i16;
	let mut rng = thread_rng();
	let position_y = rng.gen_range(0..rng_y);
	let speed_x = rng.gen_range(1..5);

	EnemyRocket::new(
		sprite_sheet,
		ENEMY_1,
		Point {
			x: width,
			y: (position_y + 100) as f32
		},
		Point {
			x: width + 28.0,
			y: (position_y + 133) as f32
		},
		Point {
			x: -velocity_x - speed_x as f32,
			y: 0.0,
		},
		20.0,
		30.0,
		audio,
		sound,
		0
	)
}
