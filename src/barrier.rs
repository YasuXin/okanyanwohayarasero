/*use crate::engine::{Image, Renderer};
use crate::obstacle::Obstacle;
use crate::player::Player;

//-----------------------------------
// 動かないオブジェクトを定義
//-----------------------------------


#[derive(Debug)]
pub struct Barrier {
	image: Image
}

impl Obstacle for Barrier {
	fn check_intersection(&self, player: &mut Player) {
		if player.bounding_box().intersects(self.image.bounding_box()) {
			player.knock_out()
		}
	}
	fn draw(&self, renderer: &Renderer) {
		self.image.draw(renderer);
	}

	fn move_horizontally(&mut self, _distance: f32) {
		todo!()
	}

	fn right(&self) -> i16 {
		self.image.right() as i16
	}

	fn update(&mut self, x: f32) {
		//
	}
}

impl Barrier {
	pub fn new(image: Image) -> Self {
		Self { image }
	}
}
*/