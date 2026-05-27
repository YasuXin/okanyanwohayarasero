use std::fmt::Debug;
use crate::beam::Beam;
use crate::engine::Renderer;
use crate::player::{Player};

// -----------------------------------------
// フィールド中のオブジェクトのトレイトを定義する
// -----------------------------------------

pub trait Obstacle: Debug {
	
	fn get_kinds(&self) -> ObstacleKinds;
	fn check_intersection(&self, player: &mut Player);
	fn draw(&self, renderer: &Renderer);
	fn move_horizontally(&mut self, distance: f32);
	fn right(&self) -> i16;
	
	fn update(&mut self, x: f32, player: &mut Player, beams: &mut Vec<Beam>);
	
	fn be_defeated(&mut self);
}

// Obstacleの中で、最も右側のものを取得
pub fn right_most(obstacle_list: &Vec<Box<dyn Obstacle>>) -> i16 {
	obstacle_list
		.iter()
		.map(|obstacle| obstacle.right())
		.max_by(|x, y| x.cmp(&y))
		.unwrap_or(0)
}

#[derive(Debug)]
pub enum ObstacleKinds {
	Platform, WoodBox
}