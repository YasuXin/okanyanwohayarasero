use std::fmt::Debug;
use std::rc::Rc;
use crate::engine::{Point, Rect, SpriteSheet};

#[derive(Debug, Clone)]
pub struct Live;

#[derive(Debug, Clone)]
pub struct BeingDefeated;

#[derive(Debug, Clone)]
pub struct Dead;

#[derive(Debug, Clone)]
pub struct BonusContext {
	pub sprite_sheet: Rc<SpriteSheet>,
	pub position: Point,
	pub bounding_box: Rect,
	pub velocity: Point,
	pub frame: i16,
	pub defeated: bool
}

impl BonusContext {
	
	pub fn reset_frame(mut self) -> Self {
		self.frame = 0;
		self
	}
}