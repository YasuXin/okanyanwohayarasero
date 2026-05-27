use std::fmt::Debug;
use std::rc::Rc;
use crate::beam::Beam;
use crate::engine::{Cell, Point, Renderer, SpriteSheet};
use crate::player::{Player};

//--------------------------------------------
// 衝突すると、ゲームオーバーになる障害物を定義する
//--------------------------------------------

pub trait Enemy: Debug {
	
	fn get_struct_kinds(&self) -> Kinds;
	fn pos_x(&self) -> f32;

	fn pos_y(&self) -> f32;

	fn update(&mut self, player: &mut Player, beams: &Vec<Beam>);

	fn draw(&self, renderer: &Renderer);

	fn be_defeated(&mut self);
}

pub enum Kinds {
	Rocket, Okayu, Beam, Energy, Fubura, Kedama, Hato, Pokobe
}

#[derive(Debug, Clone)]
pub struct Live;

#[derive(Debug, Clone)]
pub struct BeingDefeated;

#[derive(Debug, Clone)]
pub struct Dead;


#[derive(Debug, Clone)]
pub struct EnemyContext {
	pub attack_radius: f32, // Playerがゲームオーバーになる判定の半径
	pub defeated_radius: f32, // Playerの攻撃でエネミーが消滅する判定の半径
	pub frame: i16,
	pub defeated: bool,
	pub position: Point, // 左上の座標
	pub center: Point, // 中心点
	pub velocity: Point, // 移動
	pub sheet: Rc<SpriteSheet>,
	pub sprite: Cell,
}
