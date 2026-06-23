use std::cmp::{max, min};
use std::rc::Rc;
use crate::beam::Beam;
use crate::bonus::{BeingDefeated, BonusContext, Dead, Live};
use crate::engine::{Audio, Cell, Point, Rect, Renderer, Sound, SpriteSheet};
use crate::obstacle::{Obstacle, ObstacleKinds};
use crate::player::Player;

#[derive(Debug, Clone)]
pub struct BonusWoodBox {
	pub state_machine: BonusStateMachine,
}

impl Obstacle for BonusWoodBox {
	
	fn get_kinds(&self) -> ObstacleKinds {
		ObstacleKinds::WoodBox
	}
	
	fn check_intersection(&self, player: &mut Player) {
		self.state_machine.check_intersection(player);
	}

	fn draw(&self, renderer: &Renderer) {
		self.state_machine.draw(renderer);
	}

	fn move_horizontally(&mut self, distance: f32) {
		self.state_machine.move_horizontally(distance);
	}

	fn right(&self) -> i16 {
		self.state_machine.context().bounding_box.right() as i16
	}

	fn update(&mut self, x: f32, player: &mut Player, beams: &mut Vec<Beam>) {
		self.move_horizontally(x);

		self.check_intersection(player);

		self.state_machine = self.state_machine.clone().update(player, beams);
	}
	
	fn be_defeated(&mut self) {
		self.state_machine.be_defeated();
	}
}

impl BonusWoodBox {
	pub fn new(
		gravity: f32,
		sprite_sheet: Rc<SpriteSheet>,
		position: Point,
		audio: [Rc<Audio>; 1],
		sound: [Rc<Sound>; 1],
	) -> BonusWoodBox {

		BonusWoodBox {

			state_machine: BonusStateMachine::Live(

				BonusState {
					context: BonusContext {
						sprite_sheet,
						position: Point { x: position.x - 200.0, y: position.y - 200.0 },
						bounding_box: Rect::new(
							Point { x: position.x + 5.0, y: position.y + 5.0 },
							90.0,
							90.0
						),
						velocity: Point { x: 0.0, y: 0.0 },
						frame: 0,
						defeated: false,
					},
					wood_box_context: WoodBoxContext {
						gravity: gravity * 5.0,
						audio,
						sound,
					},
					_state: Live,
				},
			),
		}
	}
}

#[derive(Debug, Clone)]
pub enum BonusStateMachine {
	Live(BonusState<Live>),
	BeingDefeated(BonusState<BeingDefeated>),
	Dead(BonusState<Dead>)
}

impl BonusStateMachine {
	pub fn update(self, player: &mut Player, beams: &mut Vec<Beam>) -> Self {
		match self.clone() {
			BonusStateMachine::Live(state) => state.update(player, beams).into(),
			BonusStateMachine::BeingDefeated(state) => state.update().into(),
			BonusStateMachine::Dead(state) => state.update().into(),
		}
	}

	pub fn context(&self) -> &BonusContext {
		match self {
			BonusStateMachine::Live(state) => state.context(),
			BonusStateMachine::BeingDefeated(state) => state.context(),
			BonusStateMachine::Dead(state) => state.context()
		}
	}
	pub fn draw(&self, renderer: &Renderer) {
		match self{
			BonusStateMachine::Live(state) => state.draw(renderer),
			BonusStateMachine::BeingDefeated(state) => state.draw(renderer),
			BonusStateMachine::Dead(_state) => {},
		}
	}
	pub fn check_intersection(&self, player: &mut Player) {
		match self {
			BonusStateMachine::Live(state) => state.check_intersection(player),
			BonusStateMachine::BeingDefeated(_state) => {}
			BonusStateMachine::Dead(_state) => {}
		}
	}
	pub fn move_horizontally(&mut self, distance: f32) {
		match self{
			BonusStateMachine::Live(state) => state.move_horizontally(distance).into(),
			BonusStateMachine::BeingDefeated(state) => state.move_horizontally(distance).into(),
			BonusStateMachine::Dead(state) => state.move_horizontally(distance).into(),
		}
	}
	pub fn be_defeated(&mut self) {
		match self {
			BonusStateMachine::Live(state) => state.context.defeated = true,
			BonusStateMachine::BeingDefeated(_state) => {}
			BonusStateMachine::Dead(_state) => {},
		}
	}
}

#[derive(Debug, Clone)]
pub enum LiveEndState {
	Continue(BonusState<Live>),
	Complete(BonusState<BeingDefeated>)
}

#[derive(Debug, Clone)]
pub enum BeingDefeatedEndState {
	Continue(BonusState<BeingDefeated>),
	Complete(BonusState<Dead>)
}

impl From<BonusState<Live>> for BonusStateMachine {
	fn from(state: BonusState<Live>) -> Self {
		BonusStateMachine::Live(state)
	}
}
impl From<BonusState<BeingDefeated>> for BonusStateMachine {
	fn from(state: BonusState<BeingDefeated>) -> Self {
		BonusStateMachine::BeingDefeated(state)
	}
}
impl From<BonusState<Dead>> for BonusStateMachine {
	fn from(state: BonusState<Dead>) -> Self {
		BonusStateMachine::Dead(state)
	}
}

impl From<LiveEndState> for BonusStateMachine {
	fn from(end_state: LiveEndState) -> Self {
		match end_state {
			LiveEndState::Complete(state) => state.into(),
			LiveEndState::Continue(state) => state.into(),
		}
	}
}
impl From<BeingDefeatedEndState> for BonusStateMachine {
	fn from(end_state: BeingDefeatedEndState) -> Self {
		match end_state {
			BeingDefeatedEndState::Complete(state) => state.into(),
			BeingDefeatedEndState::Continue(state) => state.into(),
		}
	}
}


#[derive(Debug, Clone)]
pub struct BonusState<S> {
	context: BonusContext,
	wood_box_context: WoodBoxContext,
	_state: S
}

impl<S> BonusState<S> {

	fn move_horizontally(&mut self, x: f32) {

		self.context.position.x += x;
		self.context.bounding_box.position.x += x;
	}

	fn current_sprite(&self, frame_name: &str) -> Option<&Cell> {
		self.context.sprite_sheet.sheet.frames.get(frame_name)
	}

	pub fn render_image(&self, renderer: &Renderer, sprite: &Cell) {

		renderer.draw_image(
			&self.context.sprite_sheet.image,
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

	fn context(&self) -> &BonusContext {
		&self.context
	}

	pub fn play_sound(&mut self) {
		let audio = &self.wood_box_context.audio[0];
		let sound = &self.wood_box_context.sound[0];

		if let Err(err) = audio.play_sound(&sound) {
			log!("Error playing beam_sound: {}", err);
		}
	}
}

impl BonusState<Live> {

	fn check_intersection(&self, player: &mut Player) {

		let rect_x = self.context.bounding_box.position.x as i32;
		let rect_y = self.context.bounding_box.position.y as i32;
		let rect_width = self.context.bounding_box.width as i32;
		let rect_height = self.context.bounding_box.height as i32;

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

	fn check_defeat(&self, beams: &mut Vec<Beam>) -> bool {

		beams.iter().any(|beam| {
			if self.context.bounding_box.intersects(&beam.bounding_box) {
				let beam_x = ( beam.bounding_box.position.x + beam.center.x - beam.bounding_box.width / 2.0 ) as i32;
				let beam_y = ( beam.bounding_box.position.y + beam.center.y - beam.bounding_box.height / 2.0 ) as i32;
				let beam_w = beam.bounding_box.width as i32;
				let beam_h = beam.bounding_box.height as i32;

				let box_x = self.context.bounding_box.position.x as i32;
				let box_y = self.context.bounding_box.position.y as i32;
				let box_w = self.context.bounding_box.width as i32;
				let box_h = self.context.bounding_box.height as i32;

				let conflict_x = if ( beam_x + beam_w ) >= box_x && ( box_x + box_w ) >= beam_x {
					true
				} else {
					false
				};

				let conflict_y = if ( beam_y + beam_h ) >= box_y && ( box_y + box_h ) >= beam_y {
					true
				} else {
					false
				};

				if conflict_x && conflict_y {
					true
				} else {
					false
				}
			} else {
				false
			}
		})
	}

	fn update(mut self, player: &mut Player, beams: &mut Vec<Beam>) -> LiveEndState {

		if self.context.defeated {

			LiveEndState::Complete(self.be_defeated(player))

		} else {
			self.context.frame += 1;

			if self.context.frame % 2 == 0 && self.check_defeat(beams) {
				self.context.defeated = true;
				LiveEndState::Complete(self.be_defeated(player))
			} else {
				LiveEndState::Continue(self)
			}
		}
	}

	fn draw(&self, renderer: &Renderer) {
		let sprite = self
			.current_sprite("wood_box_0.png")
			.expect("Cell not found");

		self.render_image(renderer, sprite);
	}

	fn be_defeated(&mut self, player: &mut Player) -> BonusState<BeingDefeated> {

		player.state_machine.set_add_points(500);

		self.play_sound();

		BonusState {
			_state: BeingDefeated,
			context: self.context.clone().reset_frame(),
			wood_box_context: self.wood_box_context.clone(),
		}
	}
}

impl BonusState<BeingDefeated> {

	fn update(mut self) -> BeingDefeatedEndState {

		self.context.velocity.y += self.wood_box_context.gravity;
		self.context.position.y += self.context.velocity.y;

		self.context.frame += 1;

		if self.context.frame > 18 {
			self.context.frame = 18;
		}

		if self.context.position.y >= 560.0 {
			BeingDefeatedEndState::Complete(self.die())

		} else {
			BeingDefeatedEndState::Continue(self)
		}
	}

	fn draw(&self, renderer: &Renderer) {

		let f = self.context.frame / 3;
		match f {
			0..=6 => {
				let sprite_name = format!("wood_box_{}.png", f);

				let sprite = self
					.current_sprite(&sprite_name)
					.expect("Cell not found");

				self.render_image(renderer, sprite);
			}
			_ => {}
		}

	}

	fn die(self) -> BonusState<Dead> {
		BonusState {
			_state: Dead,
			context: self.context,
			wood_box_context: self.wood_box_context,
		}
	}
}

impl BonusState<Dead> {
	fn update(mut self) -> Self {
		self.context.bounding_box.position.x = -2000.0;
		self
	}
}

#[derive(Debug, Clone)]
struct WoodBoxContext {
	pub gravity: f32,
	pub audio: [Rc<Audio>; 1],
	pub sound: [Rc<Sound>; 1],
}