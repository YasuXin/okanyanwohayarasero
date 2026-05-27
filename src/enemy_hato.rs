use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::beam::Beam;
use crate::enemy::{Enemy, EnemyContext, Kinds};
use crate::engine::{Audio, Cell, Point, Rect, Renderer, Sound, SpriteSheet};
use crate::player::Player;

const HATO_NAME: &str = "hato";

#[derive(Debug, Clone)]
pub struct EnemyHato {
	pub state: EnemyState
}

impl Enemy for EnemyHato {
	fn get_struct_kinds(&self) -> Kinds {
		Kinds::Hato
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

impl EnemyHato {
	fn new(
		sheet: Rc<SpriteSheet>,
		sprite_name: &str,
		position: Point,
		center: Point,
		velocity: Point,
		attack_radius: f32,
		defeated_radius: f32,
		audio: [Rc<Audio>; 1],
		sound: [Rc<Sound>; 2],
		frame: i16,
		fixed_position_x: f32
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

		EnemyHato {
			state: EnemyState {
				context,
				audio_context: AudioContext{audio, sound},
				fixed_position_x
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct EnemyState {
	pub context: EnemyContext,
	pub audio_context: AudioContext,
	pub fixed_position_x: f32
}

impl EnemyState {

	pub fn update(mut self) -> Self {

		if self.context.defeated {
			self.context.position.x = -2000.0;
		} else {
			match self.context.frame {
				0 => self.play_thunder_sound(),
				300 => self.play_eye_sound(),
				390 => self.play_thunder_sound(),
				_ => {}
			}

			match self.context.frame {
				0..300 => {
					let mut rng = thread_rng();
					let next = rng.gen_range(0..10);
					self.context.position.x = self.fixed_position_x + next as f32;
				}
				300 => self.context.velocity.y = 0.0,
				389 => self.context.velocity.y = 1.8,
				390..700 => {
					let mut rng = thread_rng();
					let next = rng.gen_range(0..10);
					self.context.position.x = self.fixed_position_x + next as f32;
				}
				700 => self.context.position.x = -2000.0,
				_ => {}
			}

			self.context.position.x += self.context.velocity.x;
			self.context.position.y += self.context.velocity.y;

			self.context.frame += 1;
		}

		self
	}

	pub fn draw(&self, renderer: &Renderer) {

		let sprite_name: String;

		match self.context.frame {
			0..300 => sprite_name = format!("{}_0.png", HATO_NAME),
			300..303 => sprite_name = format!("{}_1.png", HATO_NAME),
			303..306 => sprite_name = format!("{}_2.png", HATO_NAME),
			306..309 => sprite_name = format!("{}_3.png", HATO_NAME),
			309..312 => sprite_name = format!("{}_4.png", HATO_NAME),
			312..315 => sprite_name = format!("{}_5.png", HATO_NAME),
			_ => sprite_name = format!("{}_5.png", HATO_NAME)
		}

		match self.context.frame {
			0..860 => {
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

	pub fn play_thunder_sound(&mut self) {
		let audio = &self.audio_context.audio[0];
		let sound = &self.audio_context.sound[0];

		if let Err(err) = audio.play_sound(&sound) {
			log!("Error playing sound: {}", err);
		}
	}

	pub fn play_eye_sound(&mut self) {
		let audio = &self.audio_context.audio[0];
		let sound = &self.audio_context.sound[1];

		if let Err(err) = audio.play_sound(&sound) {
			log!("Error playing sound: {}", err);
		}
	}
}

#[derive(Debug, Clone)]
pub struct AudioContext {
	pub audio: [Rc<Audio>; 1],
	pub sound: [Rc<Sound>; 2],
}

pub fn create_enemy_hato(
	sprite_sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sounds: [Rc<Sound>; 2],
	width: f32,
	height: f32,
) -> EnemyHato {

	let sprite_name = format!("{}_0.png", HATO_NAME);

	EnemyHato::new(
		sprite_sheet,
		&sprite_name,
		Point {
			x: width - 300.0,
			y: height
		},
		Point {
			x: 0.0,
			y: 0.0
		},
		Point {
			x: 0.0,
			y: -1.8,
		},
		0.0,
		0.0,
		audio,
		sounds,
		0,
		width - 300.0
	)
}