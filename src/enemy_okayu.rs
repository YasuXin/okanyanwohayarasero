use std::fmt::Debug;
use std::rc::Rc;
use crate::beam::Beam;
use crate::enemy::{Enemy, EnemyContext, Kinds};
use crate::engine::{Point, Rect, Renderer, SpriteSheet};
use crate::player::{Player};

//--------------------------------------------
// おかゆを定義する
//--------------------------------------------

const ENEMY_2: &str = "okayu_0.png";

#[derive(Debug, Clone)]
pub struct EnemyOkayu {
	pub state: EnemyState
}

impl Enemy for EnemyOkayu {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Okayu
	}
	
	fn pos_x(&self) -> f32 {
		self.state.context.position.x
	}

	fn pos_y(&self) -> f32 {
		self.state.context.position.y
	}

	fn update(&mut self, _player: &mut Player, _beams: &Vec<Beam>) {
		self.state = self.state.clone().update();
	}

	fn draw(&self, renderer: &Renderer) {
		self.state.draw(renderer);
	}

	fn be_defeated(&mut self) {
		self.state.context.defeated = true;
	}
}

impl EnemyOkayu {
	fn new(
		sheet: Rc<SpriteSheet>,
		sprite_name: &str,
		position: Point,
		center: Point,
		velocity: Point,
		attack_radius: f32,
		defeated_radius: f32,
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

		EnemyOkayu {
			state: EnemyState {
				context,
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct EnemyState {
	pub context: EnemyContext,
}

impl EnemyState{

	pub fn update(mut self) -> Self {

		match self.context.frame {
			0..20 => self.context.position.x -= self.context.velocity.x,
			20..140 => {} // 静止する
			140..200 => self.context.position.x += self.context.velocity.x,
			200..240 => self.context.position.x -= self.context.velocity.x * 3.5,
			240..360 => {} // ビーム発射する
			360.. 600 => self.context.position.x += self.context.velocity.x * 2.5,
			600 => self.context.position.x = -2000.0,
			_ => {}
		}

		self.context.frame += 1;

		self
	}

	pub fn draw(&self, renderer: &Renderer) {

		let sprite = &self.context.sprite;

		let size: f32 = 1.5;

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
}

pub fn create_enemy_okayu(
	sprite_sheet: Rc<SpriteSheet>,
	position_y: i16,
	width: f32
) -> EnemyOkayu {

	EnemyOkayu::new(
		sprite_sheet,
		ENEMY_2,
		Point {
			x: width,
			y: (position_y - 50) as f32
		},
		Point {
			x: width + 30.0,
			y: (position_y + 175) as f32
		},
		Point {
			x: 2.0,
			y: 0.0,
		},
		0.0,
		0.0,
		0
	)
}