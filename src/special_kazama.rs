use std::rc::Rc;
use web_sys::HtmlImageElement;
use crate::engine::{draw_modal_background, Audio, Cell, Image, Point, Rect, Renderer, Sheet, Sound};
use crate::special::{Special, SpecialContext};

#[derive(Debug)]
pub struct KazamaSounds {
	koto: Rc<Sound>,
	sword_0: Rc<Sound>,
	sword_1: Rc<Sound>,
}

impl KazamaSounds {
	pub fn new(koto: Rc<Sound>, sword_0: Rc<Sound>, sword_1: Rc<Sound>) -> KazamaSounds {
		KazamaSounds {
			koto, sword_0, sword_1
		}
	}
}

#[derive(Debug)]
pub struct SpecialKazama {
	velocity_x: f32,
	context: SpecialContext,
	images: [Image; 5],
	sheet: Sheet,
	sprite_sheet_image: HtmlImageElement,
	sounds: KazamaSounds,
	def: bool,
}

impl Special for SpecialKazama {

	fn is_end(&self) -> bool {
		self.context.end
	}

	fn update(&mut self) {

		match self.context.frame {
			60..120 =>
				self.context.position.x
					+= 6.0 - ( ( self.context.frame - 60 ) as f32 / 10.0 ),

			244..254 =>
				self.context.position.x
					+= self.velocity_x,

			_ => {}
		}

		match self.context.frame {
			0 => self.play_special_koto_sound(),
			180 => self.play_special_sword_0_sound(),
			240 => self.play_special_sword_1_sound(),
			430 => self.context.end = true,
			_ => {}
		}

		self.context.frame += 1;
	}

	fn draw(&self, renderer: &Renderer) {

		if self.context.frame < 400 {
			draw_modal_background("rgba(0, 0, 0, 0.6)");
			self.images[4].draw(renderer);

		}

		let sprite_name: &str = if self.def {
			"kazama_def_"
		} else {
			"kazama_"
		};

		let kazama_sprite_name: String;

		match self.context.frame {

			0..124 => kazama_sprite_name = format!("{}0.png", sprite_name),
			124..127 => kazama_sprite_name = format!("{}1.png", sprite_name),
			127..130 => kazama_sprite_name = format!("{}2.png", sprite_name),
			130..180 => kazama_sprite_name = format!("{}3.png", sprite_name),
			180..240 => kazama_sprite_name = format!("{}4.png", sprite_name),
			240..400 => kazama_sprite_name = format!("{}5.png", sprite_name),
			_ => kazama_sprite_name = format!("{}0.png", sprite_name)

		};

		match self.context.frame {
			0..400 => {
				let sprite = self
					.current_sprite(&kazama_sprite_name)
					.expect("Cell not found");

				self.render_image(renderer, sprite);
			}
			_ => {}
		}


		match self.context.frame {
			0..60 => {
				let str_color = format!(
					"rgba({0:?}, 255, {0:?}, 1.0)",
					self.context.frame + 195
				);
				draw_modal_background(&str_color);
			}
			60..90 => {
				let str_color = format!(
					"rgba(255, 255, 255, {})",
					( 90 - self.context.frame ) as f32 / 30.0
				);
				draw_modal_background(&str_color);
			}
			180..210 => {
				let str_color = format!(
					"rgba(0, 0, 0, {})",
					( 210 - self.context.frame ) as f32 / 30.0
				);
				draw_modal_background(&str_color);
			}
			240..247 => {
				let str_color = "rgba(0, 0, 0, 1.0)";
				draw_modal_background(&str_color);
			}
			247..277 => {
				let str_color = format!(
					"rgba(0, 0, 0, {})",
					( 277 - self.context.frame ) as f32 / 30.0
				);
				draw_modal_background(&str_color);
			}
			300..400 => {

				let mut a = ( self.context.frame - 300 ) as f32 / 60.0;
				if a > 1.0 {
					a = 1.0;
				}
				let mut f = self.context.frame - 75;
				if f > 255 {
					f = 255;
				}
				let str_color = format!("rgba({0:?}, 255, {1:?}, {2:?})", f, f, a );

				draw_modal_background(&str_color);
			}
			_ => {}
		}
		match self.context.frame {
			240 => self.images[0].draw_bg(renderer),
			241 => self.images[1].draw_bg(renderer),
			242 => self.images[2].draw_bg(renderer),
			243..247 => self.images[3].draw_bg(renderer),
			_ => {}
		}
	}
}

impl SpecialKazama {

	pub fn new (
		images: [Image; 5],
		sheet: Sheet,
		sprite_sheet_image: HtmlImageElement,
		center: Point,
		audio: Rc<Audio>,
		sounds: KazamaSounds,
		def: bool
	) -> Self {

		let position: Point;
		if def {
			position = Point { x: -160.0, y: 170.0 };
		} else {
			position = Point { x: -250.0, y: 10.0 };
		}

		SpecialKazama {
			context: SpecialContext {
				frame: 0,
				position,
				audio,
				center,
				end: false
			},
			images,
			sheet,
			sprite_sheet_image,
			sounds,
			def,
			velocity_x: 80.0
		}
	}

	fn current_sprite(&self, frame_name: &str) -> Option<&Cell> {
		self.sheet.frames.get(frame_name)
	}

	pub fn render_image(&self, renderer: &Renderer, sprite: &Cell) {
		renderer.draw_image(
			&self.sprite_sheet_image,
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

	fn play_special_koto_sound(&self) {
		if let Err(err) = self.context.audio.play_sound(&self.sounds.koto) {
			log!("Error playing special_koto_sound: {}", err);
		}
	}
	fn play_special_sword_0_sound(&self) {
		if let Err(err) = self.context.audio.play_sound(&self.sounds.sword_0) {
			log!("Error playing special_sword_0_sound: {}", err);
		}
	}
	fn play_special_sword_1_sound(&self) {
		if let Err(err) = self.context.audio.play_sound(&self.sounds.sword_1) {
			log!("Error playing special_sword_1_sound: {}", err);
		}
	}
}