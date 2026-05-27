use std::rc::Rc;
use crate::engine::{draw_circle, draw_modal_background, Audio, Image, Point, Rect, Renderer, Sound};
use crate::special::{Special, SpecialContext};

#[derive(Debug)]
pub struct OkanyanSounds {
	special: Rc<Sound>,
	energy: Rc<Sound>
}

impl OkanyanSounds {
	pub fn new(special: Rc<Sound>, energy: Rc<Sound>) -> OkanyanSounds {
		OkanyanSounds {
			special, energy
		}
	}
}

#[derive(Debug)]
pub struct SpecialOkanyan{
	context: SpecialContext,
	images: [Image; 3],
	sounds: OkanyanSounds,
}

impl Special for SpecialOkanyan {

	fn is_end(&self) -> bool {
		self.context.end
	}

	fn update(&mut self) {

		match self.context.frame {
			150..360 => {
				self.images[1].reset_bounding_box();
				self.images[1].shake_bounding_box();
			}
			_ => {}
		}

		match self.context.frame {
			0 => self.play_special_sound(),
			90 => self.play_special_sound(),
			150 => self.play_special_energy_sound(),
			360 => self.context.end = true,
			_ => {}
		}

		self.context.frame += 1;
	}

	fn draw(&self, renderer: &Renderer) {

		if self.context.frame < 330 {

			draw_modal_background("rgba(0, 0, 0, 0.9)");
		}

		match self.context.frame {
			0..91 => self.images[0].draw_bg(renderer),
			91..330 => self.images[1].draw_bg(renderer),
			_ => {}

		}

		match self.context.frame {
			0..30 => {
				let str_color = format!(
					"rgba(0, 0, 0, {})",
					( 30 - self.context.frame ) as f32 / 30.0
				);
				draw_modal_background(&str_color);
			}
			90..120 => {
				let str_color = format!(
					"rgba(0, 0, 0, {})",
					( 120 - self.context.frame ) as f32 / 30.0
				);
				draw_modal_background(&str_color);
			}
			_ => {}
		}

		match self.context.frame {
			120..330 => {
				let f = ( self.context.frame - 120 ) as f32;
				renderer.draw_image(
					&self.images[2].element,
					&self.images[2].bounding_box,
					&Rect::new(
						Point {
							x: self.context.center.x - f * 10.0,
							y: self.context.center.y - f * 10.0
						},
						f * 20.0,
						f * 20.0,
					)
				)
			},
			_ => {}
		}

		match self.context.frame {

			180..330 => {
				draw_circle(
					"rgba(255, 255, 255, 0.5)",
					&self.context.center,
					(( self.context.frame - 180 ) * 10 ) as f32
				)
			}
			_ => {}
		}

		match self.context.frame {

			210..330 => {
				draw_circle(
					"rgba(100, 255, 100, 0.5)",
					&self.context.center,
					(( self.context.frame - 210 ) * 15 ) as f32
				)
			}
			_ => {}
		}

		match self.context.frame {

			240..330 => {
				draw_circle(
					"rgba(255, 255, 255, 0.8)",
					&self.context.center,
					(( self.context.frame - 240 ) * 20 ) as f32
				)
			}
			_ => {}
		}
	}
}

impl SpecialOkanyan {

	pub fn new (
		images: [Image; 3],
		center: Point,
		audio: Rc<Audio>,
		sounds: OkanyanSounds,
	) -> Self {

		SpecialOkanyan {
			context: SpecialContext {
				frame: 0,
				position: Point { x: 0.0, y : 0.0 },
				center,
				audio,
				end: false
			},
			images,
			sounds,
		}
	}

	fn play_special_sound(&self) {
		if let Err(err) = self.context.audio.play_sound(&self.sounds.special) {
			log!("Error playing special_sound: {}", err);
		}
	}

	fn play_special_energy_sound(&self) {
		if let Err(err) = self.context.audio.play_sound(&self.sounds.energy) {
			log!("Error playing special_energy_sound: {}", err);
		}
	}
}