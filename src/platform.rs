use std::{
	rc::Rc,
	cmp::{
		max, min
	}
};
use crate::beam::Beam;
use crate::engine::{Cell, Point, Rect, Renderer, SpriteSheet};
use crate::obstacle::{Obstacle, ObstacleKinds};
use crate::player::{Player};

//----------------------------------------
// 地形に関する処理を定義する
//-----------------------------------------


#[derive(Debug)]
pub struct Platform {
	kinds_machine: PlatformKindsMachine,
}

impl Obstacle for Platform {
	fn get_kinds(&self) -> ObstacleKinds {
		ObstacleKinds::Platform
	}

	fn check_intersection(&self, player: &mut Player) {

		// intersects()メソッドで、playerとObstacleが衝突しているかチェックする、
		if let Some(platform) = self
			.bounding_boxes()
			.iter()
			.find(| &bounding_box | player.bounding_box().intersects(bounding_box))
		{
			let rect_x = platform.position.x as i32;
			let rect_y = platform.position.y as i32;
			let rect_width = platform.width as i32;
			let rect_height = platform.height as i32;

			let player_center_x = player.center_x() as i32;
			let player_center_y = player.center_y() as i32;

			let player_radius = player.radius() as i32;

			let closest_x = max(rect_x, min(player_center_x, rect_x + rect_width));
			let closest_y = max(rect_y, min(player_center_y, rect_y + rect_height));

			let distance_x = player_center_x - closest_x;
			let distance_y = player_center_y - closest_y;

			let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

			if distance_squared < (player_radius * player_radius) {
				
				player.knock_out();

			}
		}
	}

	fn draw(&self, renderer: &Renderer) {

		self.kinds_machine.draw(renderer);
	}

	fn move_horizontally(&mut self, distance: f32) {
		self.kinds_machine.move_horizontally(distance)
	}

	fn right(&self) -> i16 {
		self.bounding_boxes()
			.last()
			.unwrap_or(&Rect::default())
			.right() as i16
	}

	fn update(&mut self, x: f32, player: &mut Player, _beams: &mut Vec<Beam>) {

		self.move_horizontally(x);

		if self.kinds_machine.context().frame % 2 == 0 {
			self.check_intersection(player);
		}

		self.kinds_machine = self.kinds_machine.clone().update();
	}

	fn be_defeated(&mut self) {
		todo!();
	}
}

pub enum Kinds {
	Normal,
	MovingUp1, // 下から登ってくる地形
	MovingDown1, // 上から降りてくる地形
	MovingUpDown1, // 一定間隔で上下移動をする
	MovingUp2, // 下から上ってきて降りていく地形
	MovingDown2, // 上から降りてきて上る地形
	MovingDown3, // 上から降りてきて上り、再度降りる
	MovingUp3, // 下から登ってきており、再度上る
}

impl Platform {
	pub fn new(
		kinds: Kinds,
		sheet: Rc<SpriteSheet>,
		position: Point,
		sprite_names: &Vec<&str>,
		bounding_boxes: &Vec<Rect>,
		velocity_y: f32
	) -> Self {

		// cloned()は、Option<&T> をクローンして Option<T> にして返す
		let sprites = sprite_names
			.iter()
			.filter_map(|sprite_name| sheet.cell(sprite_name).cloned())
			.collect();

		let bounding_boxes = bounding_boxes
			.iter()
			.map(|bounding_box| {
				Rect::new_from_x_y(
					bounding_box.x() + position.x,
					bounding_box.y() + position.y,
					bounding_box.width,
					bounding_box.height,
				)
			})
			.collect();

		let context = PlatformContext {
			sheet,
			bounding_boxes,
			sprites,
			frame: 0,
			position,
			velocity_y
		};

		let kinds_machine = match kinds {
			Kinds::Normal => PlatformKindsMachine::Normal(
				PlatformKinds {
					context,
					_state: Normal
				}),
			Kinds::MovingUp1 => PlatformKindsMachine::MovingUp1(
				PlatformKinds {
					context,
					_state: MovingUp1
				}),
			Kinds::MovingDown1 => PlatformKindsMachine::MovingDown1(
				PlatformKinds {
					context,
					_state: MovingDown1
				}),
			Kinds::MovingUpDown1 => PlatformKindsMachine::MovingUpDown1(
				PlatformKinds {
					context,
					_state: MovingUpDown1
				}),
			Kinds::MovingUp2 => PlatformKindsMachine::MovingUp2(
				PlatformKinds {
					context,
					_state: MovingUp2
				}),
			Kinds::MovingDown2 => PlatformKindsMachine::MovingDown2(
				PlatformKinds {
					context,
					_state: MovingDown2
				}),
			Kinds::MovingUp3 => PlatformKindsMachine::MovingUp3(
				PlatformKinds {
					context,
					_state: MovingUp3
				}),
			Kinds::MovingDown3 => PlatformKindsMachine::MovingDown3(
				PlatformKinds {
					context,
					_state: MovingDown3
				}),
		};

		Platform {
			kinds_machine,
		}
	}

	// 衝突判定時に呼び出すメソッド
	fn bounding_boxes(&self) -> &Vec<Rect> {
		self.kinds_machine.context().bounding_boxes.as_ref()
	}

}

#[derive(Debug, Clone)]
pub enum PlatformKindsMachine {
	Normal(PlatformKinds<Normal>),
	MovingUp1(PlatformKinds<MovingUp1>),
	MovingDown1(PlatformKinds<MovingDown1>),
	MovingUpDown1(PlatformKinds<MovingUpDown1>),
	MovingUp2(PlatformKinds<MovingUp2>),
	MovingDown2(PlatformKinds<MovingDown2>),
	MovingUp3(PlatformKinds<MovingUp3>),
	MovingDown3(PlatformKinds<MovingDown3>),
}

impl PlatformKindsMachine {
	pub fn update(self) -> Self {
		match self.clone() {
			PlatformKindsMachine::Normal(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingUp1(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingDown1(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingUpDown1(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingUp2(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingDown2(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingUp3(kinds) => kinds.update().into(),
			PlatformKindsMachine::MovingDown3(kinds) => kinds.update().into(),
		}
	}
	pub fn draw(&self, renderer: &Renderer) {
		match self {
			PlatformKindsMachine::Normal(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingUp1(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingDown1(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingUpDown1(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingUp2(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingDown2(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingUp3(kinds) => kinds.draw(renderer),
			PlatformKindsMachine::MovingDown3(kinds) => kinds.draw(renderer),
		}
	}
	pub fn context(&self) -> &PlatformContext {
		match self {
			PlatformKindsMachine::Normal(kinds) => kinds.context(),
			PlatformKindsMachine::MovingUp1(kinds) => kinds.context(),
			PlatformKindsMachine::MovingDown1(kinds) => kinds.context(),
			PlatformKindsMachine::MovingUpDown1(kinds) => kinds.context(),
			PlatformKindsMachine::MovingUp2(kinds) => kinds.context(),
			PlatformKindsMachine::MovingDown2(kinds) => kinds.context(),
			PlatformKindsMachine::MovingUp3(kinds) => kinds.context(),
			PlatformKindsMachine::MovingDown3(kinds) => kinds.context(),
		}
	}
	pub fn move_horizontally(&mut self, distance: f32) {
		match self {
			PlatformKindsMachine::Normal(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingUp1(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingDown1(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingUpDown1(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingUp2(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingDown2(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingUp3(kinds) => kinds.move_horizontally(distance),
			PlatformKindsMachine::MovingDown3(kinds) => kinds.move_horizontally(distance),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Normal;

#[derive(Debug, Clone)]
pub struct MovingUp1;

#[derive(Debug, Clone)]
pub struct MovingDown1;

#[derive(Debug, Clone)]
pub struct MovingUpDown1;

#[derive(Debug, Clone)]
pub struct MovingUp2;

#[derive(Debug, Clone)]
pub struct MovingDown2;

#[derive(Debug, Clone)]
pub struct MovingUp3;

#[derive(Debug, Clone)]
pub struct MovingDown3;


impl From<PlatformKinds<Normal>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<Normal>) -> Self {
		PlatformKindsMachine::Normal(state)
	}
}

impl From<PlatformKinds<MovingUp1>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingUp1>) -> Self {
		PlatformKindsMachine::MovingUp1(state)
	}
}

impl From<PlatformKinds<MovingDown1>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingDown1>) -> Self {
		PlatformKindsMachine::MovingDown1(state)
	}
}

impl From<PlatformKinds<MovingUpDown1>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingUpDown1>) -> Self {
		PlatformKindsMachine::MovingUpDown1(state)
	}
}

impl From<PlatformKinds<MovingUp2>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingUp2>) -> Self {
		PlatformKindsMachine::MovingUp2(state)
	}
}

impl From<PlatformKinds<MovingDown2>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingDown2>) -> Self {
		PlatformKindsMachine::MovingDown2(state)
	}
}

impl From<PlatformKinds<MovingUp3>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingUp3>) -> Self {
		PlatformKindsMachine::MovingUp3(state)
	}
}

impl From<PlatformKinds<MovingDown3>> for PlatformKindsMachine {
	fn from(state: PlatformKinds<MovingDown3>) -> Self {
		PlatformKindsMachine::MovingDown3(state)
	}
}

#[derive(Debug, Clone)]
pub struct PlatformKinds<S> {
	context: PlatformContext,
	_state: S
}

impl<S> PlatformKinds<S> {

	fn draw(&self, renderer: &Renderer) {

		let mut x: f32 = 0.0;
		self.context.sprites.iter().for_each(|sprite| {
			self.context.sheet.draw(
				renderer,
				&Rect::new_from_x_y(
					sprite.frame.x,
					sprite.frame.y,
					sprite.frame.w,
					sprite.frame.h
				),
				&Rect::new_from_x_y(
					self.context.position.x + x,
					self.context.position.y,
					sprite.frame.w,
					sprite.frame.h
				)
			);
			x += sprite.frame.w;
		});
	}

	fn move_horizontally(&mut self, x: f32) {
		self.context.position.x += x;
		self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
			bounding_box.set_x(bounding_box.position.x + x);
		});
	}

	fn context(&self) -> &PlatformContext {
		&self.context
	}

}

impl PlatformKinds<Normal> {

	fn update(self) -> Self {

		self
	}
}

impl PlatformKinds<MovingUp1> {

	fn update(mut self) -> Self {
		
		match self.context.frame {
			1..20 => {
				self.context.position.y -= self.context.velocity_y;
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y);
				});
			},
			_ => {}
		}

		if self.context.position.x < 600.0 {
			self.context.frame += 1;
		}

		self
	}
}

impl PlatformKinds<MovingDown1> {

	fn update(mut self) -> Self {
		
		match self.context.frame {
			
			1..20 => {
				self.context.position.y += self.context.velocity_y;
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y);
				});
			},
			_ => {}
		}

		if self.context.position.x < 600.0 {
			self.context.frame += 1;
		}

		self
	}
}

impl PlatformKinds<MovingUpDown1> {

	fn update(mut self) -> Self {
		
		match self.context.frame {
			0..16 => {
				
				self.context.position.y += self.context.velocity_y;
				
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y);
				});
			},
			60..76 => {

				self.context.position.y -= self.context.velocity_y;
				
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y);
				});
			},
			120 => self.context.frame = -1,
			_ => {}
		}

		self.context.frame += 1;

		self
	}
}

impl PlatformKinds<MovingUp2> {

	fn update(mut self) -> Self {

		match self.context.frame {
			1..80 => {
				
				self.context.position.y -= self.context.velocity_y;
				
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y);
				});
			},
			160..600 => {

				self.context.position.y += self.context.velocity_y / 2.0;
				
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y / 2.0);
				});
			},
			_ => {}
		}
		
		if self.context.position.x < 200.0 {
			self.context.frame += 1;
		}

		self
	}
}

impl PlatformKinds<MovingDown2> {

	fn update(mut self) -> Self {
		
		match self.context.frame {
			1..80 => {
				
				self.context.position.y += self.context.velocity_y;
				
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y);
				});
			},
			160..600 => {
				
				self.context.position.y -= self.context.velocity_y / 2.0;
				
				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {
					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y - 2.0);
				});
			},
			_ => {}
		}

		if self.context.position.x < 200.0 {
			self.context.frame += 1;
		}

		self
	}
}

impl PlatformKinds<MovingUp3> {

	fn update(mut self) -> Self {

		match self.context.frame {
			1..20 => {
				self.context.position.y -= self.context.velocity_y / 2.0;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y / 2.0);
				});
			},
			60..100 => {
				self.context.position.y += self.context.velocity_y / 4.0;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y / 4.0);
				});
			},
			100..130 => {
				self.context.position.y -= self.context.velocity_y;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y);
				});
			},
			150..600 => {
				self.context.position.y += self.context.velocity_y / 4.0;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y / 4.0);
				});
			}
			_ => {}
		}

		if self.context.position.x < 600.0 {
			self.context.frame += 1;
		}

		self
	}
}

impl PlatformKinds<MovingDown3> {

	fn update(mut self) -> Self {
		
		match self.context.frame {
			1..20 => {
				self.context.position.y += self.context.velocity_y / 2.0;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y / 2.0);
				});
			},
			60..100 => {
				self.context.position.y -= self.context.velocity_y / 4.0;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y / 4.0);
				});
			},
			100..130 => {
				self.context.position.y += self.context.velocity_y;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y + self.context.velocity_y);
				});
			},
			150..600 => {
				self.context.position.y -= self.context.velocity_y / 4.0;

				self.context.bounding_boxes.iter_mut().for_each(|bounding_box| {

					bounding_box.set_y(bounding_box.position.y - self.context.velocity_y / 4.0);
				});
			},
			_ => {}
		}
		
		if self.context.position.x < 600.0 {
			self.context.frame += 1;
		}

		self
	}
}

#[derive(Debug, Clone)]
pub struct PlatformContext {
	velocity_y: f32,
	sheet: Rc<SpriteSheet>,
	bounding_boxes: Vec<Rect>,
	sprites: Vec<Cell>,
	position: Point,
	frame: i16,
}
