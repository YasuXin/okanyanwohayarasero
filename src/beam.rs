use web_sys::HtmlImageElement;
use crate::engine::{Point, Rect, Renderer};

//------------------------------
// Player の撃ったビームを定義する
//------------------------------

#[derive(Debug, Clone)]
pub struct Beam {
	pub bounding_box: Rect,
	pub center: Point
}

impl Beam {
	pub fn new(position: Point) -> Beam {
		Beam {
			bounding_box: Rect::new(position, 180.0, 50.0),
			center: Point { x: 100.0, y: 50.0 }
		}
	}
	
	pub fn update(&mut self) {
		self.bounding_box.position.x = self.bounding_box.position.x + 40.0;
	}
	
	pub fn draw(&self, image: &HtmlImageElement, renderer: &Renderer) {
		renderer.draw_entire_image(image, &self.bounding_box.position);
	}
}