use std::fmt::Debug;
use std::rc::Rc;
use crate::engine::{Audio, Point, Renderer};

pub trait Special: Debug {

	fn is_end(&self) -> bool;
	fn update(&mut self);

	fn draw(&self, renderer: &Renderer);
}

#[derive(Debug)]
pub struct SpecialContext {
	pub position: Point,
	pub center: Point,
	pub audio: Rc<Audio>,
	pub frame: u16,
	pub end: bool
}
