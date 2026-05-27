use std::rc::Rc;
use crate::beam::Beam;
use crate::enemy::{Enemy, EnemyContext, Kinds};
use crate::engine::{Cell, Point, Rect, Renderer, SpriteSheet};
use crate::player::Player;

const KEDAMA_NAME: &str = "kedama";

#[derive(Debug, Clone)]
pub struct EnemyKedama {
	pub state: EnemyState
}

impl Enemy for EnemyKedama {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Kedama
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

impl EnemyKedama {
	fn new(
		sheet: Rc<SpriteSheet>,
		sprite_name: &str,
		position: Point,
		center: Point,
		velocity: Point,
		attack_radius: f32,
		defeated_radius: f32,
		frame: i16,
		kinds: KedamaKinds
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

		EnemyKedama {
			state: EnemyState {
				context,
				kinds
			}
		}
	}
}

#[derive(Debug, Clone)]
enum KedamaKinds {
	Normal, Reverse
}
#[derive(Debug, Clone)]
pub struct EnemyState {
	pub context: EnemyContext,
	kinds: KedamaKinds
}

impl EnemyState {

	pub fn update(mut self) -> Self {

		if self.context.defeated {
			self.context.position.x = -2000.0;
		}

		match self.context.frame {
			0..30 => self.context.position.y += self.context.velocity.y,
			300..400 => self.context.position.y -= self.context.velocity.y,
			400 => self.context.position.x = -2000.0,
			_ => {}
		}

		self.context.frame += 1;

		self
	}

	pub fn draw(&self, renderer: &Renderer) {

		let sprite_name: String;

		match self.kinds {
			KedamaKinds::Normal => {
				if self.context.frame >= 60 {
					let f = self.context.frame % 33;
					match f {
						0..3 => sprite_name = format!("{}_0.png", KEDAMA_NAME),
						3..6 => sprite_name = format!("{}_1.png", KEDAMA_NAME),
						6..9 => sprite_name = format!("{}_2.png", KEDAMA_NAME),
						9..12 => sprite_name = format!("{}_3.png", KEDAMA_NAME),
						12..15 => sprite_name = format!("{}_4.png", KEDAMA_NAME),
						15..21 => sprite_name = format!("{}_5.png", KEDAMA_NAME),
						21..24 => sprite_name = format!("{}_4.png", KEDAMA_NAME),
						24..26 => sprite_name = format!("{}_3.png", KEDAMA_NAME),
						26..28 => sprite_name = format!("{}_2.png", KEDAMA_NAME),
						28..30 => sprite_name = format!("{}_1.png", KEDAMA_NAME),
						30..33 => sprite_name = format!("{}_0.png", KEDAMA_NAME),
						_ => sprite_name = format!("{}_0.png", KEDAMA_NAME)
					}
				} else {
					sprite_name = format!("{}_0.png", KEDAMA_NAME)
				}
			}
			KedamaKinds::Reverse => {
				if self.context.frame >= 60 {
					let f = self.context.frame % 33;
					match f {
						0..3 => sprite_name = format!("{}_6.png", KEDAMA_NAME),
						3..6 => sprite_name = format!("{}_7.png", KEDAMA_NAME),
						6..9 => sprite_name = format!("{}_8.png", KEDAMA_NAME),
						9..12 => sprite_name = format!("{}_9.png", KEDAMA_NAME),
						12..15 => sprite_name = format!("{}_10.png", KEDAMA_NAME),
						15..21 => sprite_name = format!("{}_11.png", KEDAMA_NAME),
						21..24 => sprite_name = format!("{}_10.png", KEDAMA_NAME),
						24..26 => sprite_name = format!("{}_9.png", KEDAMA_NAME),
						26..28 => sprite_name = format!("{}_8.png", KEDAMA_NAME),
						28..30 => sprite_name = format!("{}_7.png", KEDAMA_NAME),
						30..33 => sprite_name = format!("{}_6.png", KEDAMA_NAME),
						_ => sprite_name = format!("{}_6.png", KEDAMA_NAME)
					}
				} else {
					sprite_name = format!("{}_6.png", KEDAMA_NAME)
				}
			}
		}

		match self.context.frame {
			0..400 => {
				let sprite = self
					.current_sprite(&sprite_name)
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

pub fn create_enemy_kedama(
	sprite_sheet: Rc<SpriteSheet>,
	height: f32
) -> EnemyKedama {

	let kedama_str = format!(
		"{}_{}.png",
		KEDAMA_NAME, 0
	);

	EnemyKedama::new(
		sprite_sheet,
		&kedama_str,
		Point {
			x: 0.0,
			y: height
		},
		Point {
			x: 0.0,
			y: 0.0
		},
		Point {
			x: 0.0,
			y: -( height / 56.0),
		},
		0.0,
		0.0,
		0,
		KedamaKinds::Normal
	)
}
pub fn create_enemy_kedama_r(
	sprite_sheet: Rc<SpriteSheet>,
	height: f32
) -> EnemyKedama {

	let kedama_str = format!(
		"{}_{}.png",
		KEDAMA_NAME, 6
	);

	EnemyKedama::new(
		sprite_sheet,
		&kedama_str,
		Point {
			x: 0.0,
			y: -height
		},
		Point {
			x: 0.0,
			y: 0.0
		},
		Point {
			x: 0.0,
			y: height / 56.0 * 1.2,
		},
		0.0,
		0.0,
		0,
		KedamaKinds::Reverse
	)
}