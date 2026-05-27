use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::beam::Beam;
use crate::enemy::{Enemy, EnemyContext, Kinds};
use crate::engine::{Cell, Point, Rect, Renderer, SpriteSheet};
use crate::player::Player;

const POKOBE_NAME: &str = "pokobe.png";

#[derive(Debug, Clone)]
pub struct EnemyPokobe {
	pub state: EnemyState
}

impl Enemy for EnemyPokobe {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Pokobe
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

impl EnemyPokobe {
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

		EnemyPokobe {
			state: EnemyState {
				context
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct EnemyState {
	pub context: EnemyContext,
}

impl EnemyState {

	pub fn update(mut self) -> Self {

		match self.context.frame {
			0..30 => self.context.position.y += self.context.velocity.y,
			90..120 => self.context.position.y -= self.context.velocity.y,
			120 => self.context.position.x = -2000.0,
			_ => {}
		}

		self.context.frame += 1;

		self
	}

	pub fn draw(&self, renderer: &Renderer) {

		match self.context.frame {
			0..120 => {
				let sprite = self
					.current_sprite(POKOBE_NAME)
					.expect("Cell not found");

				self.render_image(renderer, sprite);
			}
			_ => {}
		}
	}

	fn current_sprite(&self, frame_name: &str) -> Option<&Cell> {
		self.context.sheet.sheet.frames.get(frame_name)
	}

	pub fn render_image(&self, renderer: &Renderer, sprite: &Cell) {
		renderer.draw_image(
			&self.context.sheet.image,
			&Rect::new(
				Point {
					x: sprite.frame.x.into(),
					y: sprite.frame.y.into(),
				},
				sprite.frame.w,
				sprite.frame.h
			),
			&self.destination_box(sprite)
		);
	}

	// 描画時に呼び出すメソッド
	fn destination_box(&self, sprite: &Cell) -> Rect {

		Rect::new(
			Point {
				x: (self.context.position.x + sprite.sprite_source_size.x).into(),
				y: (self.context.position.y + sprite.sprite_source_size.y).into(),
			},
			sprite.frame.w,
			sprite.frame.h
		)
	}
}

pub fn create_enemy_pokobe(
	sprite_sheet: Rc<SpriteSheet>,
	width: f32,
	height: f32
) -> EnemyPokobe {

	let mut rng = thread_rng();
	let next = rng.gen_range(0..500);

	EnemyPokobe::new(
		sprite_sheet,
		POKOBE_NAME,
		Point {
			x: width - next as f32,
			y: height
		},
		Point {
			x: 0.0,
			y: 0.0
		},
		Point {
			x: 0.0,
			y: -height / 200.0,
		},
		0.0,
		0.0,
		0
	)
}