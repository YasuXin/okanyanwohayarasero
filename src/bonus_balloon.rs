use std::cmp::{max, min};
use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::beam::Beam;
use crate::bonus::{BeingDefeated,  BonusContext, Dead, Live};
use crate::engine::{Audio, Cell, Point, Rect, Renderer, Sound, SpriteSheet};
use crate::player::{Player};

const VELOCITY: Point = Point { x: -0.5, y: -2.0 };

const BALLOON_RADIUS: f32 = 50.0;

#[derive(Debug, Clone)]
pub struct Balloon {
	pub state_machine: BonusStateMachine,
}

impl Balloon {
	pub fn new(
		sprite_sheet: Rc<SpriteSheet>,
		audio: [Rc<Audio>; 1],
		sound: [Rc<Sound>; 1],
		width: f32,
		height: f32
	) -> Balloon {

		let mut rng = thread_rng();
		let next_y = rng.gen_range(0..10);

		Balloon {
			state_machine: BonusStateMachine::Live(
				BonusState {
					context: BonusContext {
						sprite_sheet,
						position: Point {x: width - 200.0, y: height},
						bounding_box: Rect::new(
							Point {x: width - 200.0, y: height},
							200.0,
							250.0
						),
						velocity: Point { x: VELOCITY.x, y: VELOCITY.y - ( next_y as f32 / 5.0 ) },
						frame: 0,
						defeated: false
					},
					balloon_context: BalloonContext {
						center: Point { x: 100.0, y: 100.0},
						radius: BALLOON_RADIUS, // Playerの攻撃でボーナスが消滅する判定の半径
						audio,
						sound,
					},
					_state: Live
				}
			)
		}
	}

	pub fn update(&mut self, player: &mut Player, beams: &mut Vec<Beam>) {

		self.state_machine = self.state_machine.clone().update(player, beams);
	}

	pub fn draw(&self, renderer: &Renderer) {
		let _ = &self.state_machine.draw(renderer);
	}

	pub fn top(&self) -> i16 {
		self.state_machine.context().position.y as i16
	}
	
	pub fn be_defeated(&mut self) {
		self.state_machine.be_defeated();
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
	pub fn be_defeated(&mut self) {
		match self{
			BonusStateMachine::Live(state) => state.context.defeated = true,
			BonusStateMachine::BeingDefeated(_state) => {},
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
	balloon_context: BalloonContext,
	_state: S
}

impl<S> BonusState<S> {

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
		let audio = &self.balloon_context.audio[0];
		let sound = &self.balloon_context.sound[0];

		if let Err(err) = audio.play_sound(&sound) {
			log!("Error playing beam_sound: {}", err);
		}
	}
}

impl BonusState<Live> {
	
	fn check_defeat(&self, beams: &mut Vec<Beam>) -> bool {

		beams.iter().any(|beam| {

			let beam_x = ( beam.bounding_box.position.x + beam.center.x - beam.bounding_box.width / 2.0 ) as i32;
			let beam_y = ( beam.bounding_box.position.y + beam.center.y - beam.bounding_box.height / 2.0 ) as i32;
			let beam_w = beam.bounding_box.width as i32;
			let beam_h = beam.bounding_box.height as i32;

			let balloon_center_x = ( self.context.bounding_box.position.x + self.balloon_context.center.x ) as i32;
			let balloon_center_y = ( self.context.bounding_box.position.y + self.balloon_context.center.y ) as i32;

			let balloon_radius = self.balloon_context.radius as i32;

			let closest_x = max(beam_x, min(balloon_center_x, beam_x + beam_w));
			let closest_y = max(beam_y, min(balloon_center_y, beam_y + beam_h));

			let distance_x = balloon_center_x - closest_x;
			let distance_y = balloon_center_y - closest_y;

			let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

			if distance_squared < (balloon_radius * balloon_radius) {

				true
			} else {

				false
			}
		})
	}

	fn update(mut self, player: &mut Player, beams: &mut Vec<Beam>) -> LiveEndState {

		if self.context.defeated {

			LiveEndState::Complete(self.be_defeated(player))

		} else {
			let rounded_frame = self.context.frame % 240;

			// x軸の移動
			match rounded_frame {
				0 => self.context.velocity.x = -VELOCITY.x,
				60 | 180 => self.context.velocity.x = 0.0,
				120 => self.context.velocity.x = VELOCITY.x,
				_ => {}
			}
			match rounded_frame {
				0..120 => self.context.velocity.x += VELOCITY.x / 60.0,
				120..240 => self.context.velocity.x -= VELOCITY.x / 60.0,
				_ => {}
			}
			self.context.position.x += self.context.velocity.x;
			self.context.bounding_box.position.x += self.context.velocity.x;

			// y軸の移動
			let mut rng = thread_rng();
			let next_velocity = rng.gen_range(0..10);

			self.context.position.y += self.context.velocity.y + ( next_velocity as f32 / 5.0 );
			self.context.bounding_box.position.y += self.context.velocity.y + ( next_velocity as f32 / 5.0 );

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
			.current_sprite("balloon_0.png")
			.expect("Cell not found");

		self.render_image(renderer, sprite);
	}

	fn be_defeated(&mut self, player: &mut Player) -> BonusState<BeingDefeated> {

		player.state_machine.set_add_points(500);

		self.play_sound();

		BonusState {
			_state: BeingDefeated,
			context: self.context.clone().reset_frame(),
			balloon_context: self.balloon_context.clone(),
		}
	}
}

impl BonusState<BeingDefeated> {

	fn update(mut self) -> BeingDefeatedEndState {

		self.context.frame += 1;

		match self.context.frame {
			12 => BeingDefeatedEndState::Complete(self.die()),
			_ =>  BeingDefeatedEndState::Continue(self),
		}
	}

	fn draw(&self, renderer: &Renderer) {
		
		let f = self.context.frame / 3;

		match f {
			0..=3 => {
				let sprite_name = format!("balloon_{}.png", f);

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
			balloon_context: self.balloon_context,
		}
	}
}

impl BonusState<Dead> {
	fn update(mut self) -> Self {
		self.context.position.x = -2000.0;
		self
	}
}

#[derive(Debug, Clone)]
struct BalloonContext {
	pub center: Point,
	pub radius: f32, // Playerの攻撃でボーナスが消滅する判定の半径
	pub audio: [Rc<Audio>; 1],
	pub sound: [Rc<Sound>; 1],
}