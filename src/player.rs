use std::rc::Rc;
use web_sys::HtmlImageElement;
use crate::engine::{Audio, Cell, Point, Rect, Renderer, Sheet, Sound};


const PLAYER_WIDTH: f32 = 175.0 / 1.5;
const PLAYER_HEIGHT: f32 = 100.0 / 1.5;
const PLAYER_SLASHING_WIDTH: f32 = 350.0 / 1.5;
const PLAYER_SLASHING_HEIGHT: f32 = 350.0 / 1.5;


const IDLE_FRAME_NAME: &str = "idle";
const FALLING_FRAME_NAME: &str = "falling";
const FLYING_FRAME_NAME: &str = "flying";
const SLASHING_FRAME_NAME: &str = "slash";
const BEAMING_FRAME_NAME: &str = "beam";
const SHIELDING_FRAME_NAME: &str = "shield";
const KNOCKED_OUT_FRAME_NAME: &str = "knocked_out";


// FRAMES は cell数 * 3 - 1
const FALLING_FRAMES: u8 = 17;
const FLYING_FRAMES: u8 = 29;
const SLASHING_FRAMES: u8 = 17;
const BEAMING_FRAMES: u8 = 8;
const SHIELDING_FRAMES: u8 = 2;
const KNOCKED_OUT_FRAMES: u8 = 2;

const FLY_SPEED: f32 = -9.0;
const TERMINAL_VELOCITY: f32 = 20.0;


#[derive(Debug)]
pub struct Player {
	pub state_machine: PlayerStateMachine,
	pub sprite_sheet: Sheet,
	pub sprite_sheet_image: HtmlImageElement,
}

impl Player {

	pub fn new(
		velocity_x: f32,
		gravity: f32,
		starting_point: f32,
		sheet: Sheet,
		image: HtmlImageElement,
		audio: Rc<Audio>,
		sounds: PlayerSounds
	) -> Self {
		Player {
			state_machine: PlayerStateMachine::Idle(
				PlayerState::new(
					velocity_x,
					gravity,
					starting_point,
					audio, 
					sounds
				)
			),
			sprite_sheet: sheet,
			sprite_sheet_image: image,
		}
	}

	pub fn go_back_to_title(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::GoBackToTitle);
	}

	pub fn moving_speed(&self) -> f32 {
		self.state_machine.context().velocity.x
	}

	pub fn add_moving_speed(&mut self, velocity_x: f32) {
		self.state_machine.set_moving_speed(self.moving_speed() + velocity_x);
	}

	pub fn start_right(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::Fall);
	}

	pub fn fly(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::Fly);
	}

	pub fn slash(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::Slash);
	}

	pub fn beam(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::Beam);
	}

	pub fn shield(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::Shield);
	}

	pub fn update(&mut self) {
		self.state_machine = self.state_machine.clone().update();
	}

	pub fn draw(&self, renderer: &Renderer) {

		let sprite = self
			.current_sprite()
			.expect("Cell not found");

		self.render_image(renderer, sprite);
	}

	pub fn render_image(&self, renderer: &Renderer, sprite: &Cell) {
		renderer.draw_image(
			&self.sprite_sheet_image,
			&Rect::new(
				Point {
					x: sprite.frame.x.into(),
					y: sprite.frame.y.into(),
				},
				sprite.frame.w,
				sprite.frame.h
			),
			&self.destination_box()
		);
	}

	// 描画時に呼び出すメソッド
	fn destination_box(&self) -> Rect {

		let sprite = self
			.current_sprite()
			.expect("Cell not found");

		Rect::new(
			Point {
				x: (self.state_machine.context().position.x + sprite.sprite_source_size.x).into(),
				y: (self.state_machine.context().position.y + sprite.sprite_source_size.y).into(),
			},
			sprite.frame.w / 1.5,
			sprite.frame.h / 1.5
		)
	}

	// 衝突判定時に呼び出すメソッド
	pub fn bounding_box(&self) -> Rect {

		self.destination_box()
	}

	fn frame_name(&self) -> String {
		format!(
			"{}_{}.png",
			self.state_machine.frame_name(),
			(self.state_machine.context().status.frame / 3)
		)
	}

	fn current_sprite(&self) -> Option<&Cell> {
		self.sprite_sheet.frames.get(&self.frame_name())
	}

	pub fn knock_out(&mut self) {
		self.state_machine = self.state_machine.clone().transition(Event::KnockOut);
	}

	pub fn pos_y(&self) -> f32 {
		self.state_machine.context().position.y
	}

	pub fn center_y(&self) -> f32 {
		self.state_machine.context().center.y
	}

	pub fn center_x(&self) -> f32 {
		self.state_machine.context().center.x
	}

	pub fn radius(&self) -> f32 {self.state_machine.context().status.radius}

	pub fn set_add_points(&mut self, point: i32) {
		self.state_machine.set_add_points(point);
	}

	pub fn get_add_points(&self) -> Vec<i32> {
		self.state_machine.get_add_points()
	}
	
	pub fn remove_add_points(&mut self) {
		self.state_machine.remove_add_points();
	}

	pub fn reset(self, velocity_x: f32) -> Self {
		Player::new(
			velocity_x,
			self.state_machine.context().status.gravity,
			self.state_machine.context().status.starting_point,
			self.sprite_sheet,
			self.sprite_sheet_image,
			self.state_machine.context().audio.clone(),
			self.state_machine.context().sounds.clone()
		)
	}

}

#[derive(Clone, Debug)]
pub enum PlayerStateMachine {
	Idle(PlayerState<Idle>), // 何もしないとき
	Falling(PlayerState<Falling>), // 落下中
	Flying(PlayerState<Flying>), // 飛ぶ
	Slashing(PlayerState<Slashing>), // 剣で攻撃
	Beaming(PlayerState<Beaming>), // ビームを打つ
	Shielding(PlayerState<Shielding>), // シールド状態
	KnockedOut(PlayerState<KnockedOut>) // ゲームオーバー
}

#[derive(Debug)]
pub enum Event {
	GoBackToTitle,
	Fall,
	Fly,
	Slash,
	Beam,
	Shield,
	KnockOut,
	Update
}

impl PlayerStateMachine {
	fn transition(self, event: Event) -> Self {
		match (self.clone(), event) {
			(PlayerStateMachine::Idle(state), Event::Fall) => state.start().into(),
			
			(PlayerStateMachine::Falling(state), Event::Fly) => state.fly().into(),
			(PlayerStateMachine::Falling(state), Event::Slash) => state.slash().into(),
			(PlayerStateMachine::Falling(state), Event::Beam) => state.beam().into(),
			(PlayerStateMachine::Falling(state), Event::Shield) => state.shield().into(),
			(PlayerStateMachine::Falling(state), Event::KnockOut) => state.knock_out().into(),
			
			(PlayerStateMachine::Flying(state), Event::Fall) => state.fall().into(),
			(PlayerStateMachine::Flying(state), Event::Slash) => state.slash().into(),
			(PlayerStateMachine::Flying(state), Event::Beam) => state.beam().into(),
			(PlayerStateMachine::Flying(state), Event::Shield) => state.shield().into(),
			(PlayerStateMachine::Flying(state), Event::KnockOut) => state.knock_out().into(),
			
			(PlayerStateMachine::Slashing(state), Event::KnockOut) => state.knock_out().into(),
			
			(PlayerStateMachine::Beaming(state), Event::KnockOut) => state.knock_out().into(),

			(PlayerStateMachine::Shielding(state), Event::Fall) => state.fall().into(),
			(PlayerStateMachine::Shielding(state), Event::Fly) => state.fly().into(),
			(PlayerStateMachine::Shielding(state), Event::Slash) => state.slash().into(),
			(PlayerStateMachine::Shielding(state), Event::Beam) => state.beam().into(),
			(PlayerStateMachine::Shielding(state), Event::KnockOut) => state.knock_out().into(),

			(PlayerStateMachine::KnockedOut(state), Event::GoBackToTitle) => state.go_back_to_title().into(),

			(PlayerStateMachine::Idle(state), Event::Update) => state.update().into(),
			(PlayerStateMachine::Falling(state), Event::Update) => state.update().into(),
			(PlayerStateMachine::Flying(state), Event::Update) => state.update().into(),
			(PlayerStateMachine::Slashing(state), Event::Update) => state.update().into(),
			(PlayerStateMachine::Beaming(state), Event::Update) => state.update().into(),
			(PlayerStateMachine::Shielding(state), Event::Update) => state.update().into(),
			(PlayerStateMachine::KnockedOut(state), Event::Update) => state.update().into(),

			_ => self,
		}
	}
	fn frame_name(&self) -> &str {
		match self {
			PlayerStateMachine::Idle(state) => state.frame_name(),
			PlayerStateMachine::Falling(state) => state.frame_name(),
			PlayerStateMachine::Flying(state) => state.frame_name(),
			PlayerStateMachine::Slashing(state) => state.frame_name(),
			PlayerStateMachine::Beaming(state) => state.frame_name(),
			PlayerStateMachine::Shielding(state) => state.frame_name(),
			PlayerStateMachine::KnockedOut(state) => state.frame_name(),
		}
	}

	pub fn context(&self) -> &PlayerContext {
		match self {
			PlayerStateMachine::Idle(state) => state.context(),
			PlayerStateMachine::Falling(state) => state.context(),
			PlayerStateMachine::Flying(state) => state.context(),
			PlayerStateMachine::Slashing(state) => state.context(),
			PlayerStateMachine::Beaming(state) => state.context(),
			PlayerStateMachine::Shielding(state) => state.context(),
			PlayerStateMachine::KnockedOut(state) => state.context()
		}
	}

	pub fn set_add_points(&mut self, point: i32) {
		match self {
			PlayerStateMachine::Idle(state) => state.set_add_points(point),
			PlayerStateMachine::Falling(state) => state.set_add_points(point),
			PlayerStateMachine::Flying(state) => state.set_add_points(point),
			PlayerStateMachine::Slashing(state) => state.set_add_points(point),
			PlayerStateMachine::Beaming(state) => state.set_add_points(point),
			PlayerStateMachine::Shielding(state) => state.set_add_points(point),
			PlayerStateMachine::KnockedOut(state) => state.set_add_points(point)
		}
	}

	pub fn get_add_points(& self) -> Vec<i32> {
		match self {
			PlayerStateMachine::Idle(state) => state.get_add_points(),
			PlayerStateMachine::Falling(state) => state.get_add_points(),
			PlayerStateMachine::Flying(state) => state.get_add_points(),
			PlayerStateMachine::Slashing(state) => state.get_add_points(),
			PlayerStateMachine::Beaming(state) => state.get_add_points(),
			PlayerStateMachine::Shielding(state) => state.get_add_points(),
			PlayerStateMachine::KnockedOut(state) => state.get_add_points(),
		}
	}

	pub fn remove_add_points(&mut self) {
		match self {
			PlayerStateMachine::Idle(state) => state.remove_add_points(),
			PlayerStateMachine::Falling(state) => state.remove_add_points(),
			PlayerStateMachine::Flying(state) => state.remove_add_points(),
			PlayerStateMachine::Slashing(state) => state.remove_add_points(),
			PlayerStateMachine::Beaming(state) => state.remove_add_points(),
			PlayerStateMachine::Shielding(state) => state.remove_add_points(),
			PlayerStateMachine::KnockedOut(state) => state.remove_add_points(),
		}
	}

	pub fn set_velocity_y(&mut self, y: f32) {
		match self {
			PlayerStateMachine::Idle(state) => state.set_velocity_y(y),
			PlayerStateMachine::Falling(state) => state.set_velocity_y(y),
			PlayerStateMachine::Flying(state) => state.set_velocity_y(y),
			PlayerStateMachine::Slashing(state) => state.set_velocity_y(y),
			PlayerStateMachine::Beaming(state) => state.set_velocity_y(y),
			PlayerStateMachine::Shielding(state) => state.set_velocity_y(y),
			PlayerStateMachine::KnockedOut(state) => state.set_velocity_y(y),
		}
	}

	pub fn set_moving_speed(&mut self, velocity_x: f32) {
		match self {
			PlayerStateMachine::Idle(state) => state.set_velocity_x(velocity_x),
			PlayerStateMachine::Falling(state) => state.set_velocity_x(velocity_x),
			PlayerStateMachine::Flying(state) => state.set_velocity_x(velocity_x),
			PlayerStateMachine::Slashing(state) => state.set_velocity_x(velocity_x),
			PlayerStateMachine::Beaming(state) => state.set_velocity_x(velocity_x),
			PlayerStateMachine::Shielding(state) => state.set_velocity_x(velocity_x),
			PlayerStateMachine::KnockedOut(state) => state.set_velocity_x(velocity_x),
		}
	}

	fn update(self) -> Self {
		self.transition(Event::Update)
	}
}

impl From<PlayerState<Idle>> for PlayerStateMachine {
	fn from(state: PlayerState<Idle>) -> Self {
		PlayerStateMachine::Idle(state)
	}
}

impl From<PlayerState<Falling>> for PlayerStateMachine {
	fn from(state: PlayerState<Falling>) -> Self {
		PlayerStateMachine::Falling(state)
	}
}

impl From<PlayerState<Flying>> for PlayerStateMachine {
	fn from(state: PlayerState<Flying>) -> Self {
		PlayerStateMachine::Flying(state)
	}
}

impl From<FlyingEndState> for PlayerStateMachine {
	fn from(end_state: FlyingEndState) -> Self {
		match end_state {
			FlyingEndState::Complete(state) => state.into(),
			FlyingEndState::Continue(state) => state.into(),
		}
	}
}

impl From<PlayerState<Slashing>> for PlayerStateMachine {
	fn from(state: PlayerState<Slashing>) -> Self {
		PlayerStateMachine::Slashing(state)
	}
}

impl From<SlashingEndState> for PlayerStateMachine {
	fn from(end_state: SlashingEndState) -> Self {
		match end_state {
			SlashingEndState::Complete(state) => state.into(),
			SlashingEndState::Continue(state) => state.into(),
		}
	}
}

impl From<PlayerState<Beaming>> for PlayerStateMachine {
	fn from(state: PlayerState<Beaming>) -> Self {
		PlayerStateMachine::Beaming(state)
	}
}

impl From<BeamingEndState> for PlayerStateMachine {
	fn from(end_state: BeamingEndState) -> Self {
		match end_state {
			BeamingEndState::Complete(state) => state.into(),
			BeamingEndState::Continue(state) => state.into(),
		}
	}
}

impl From<PlayerState<Shielding>> for PlayerStateMachine {
	fn from(state: PlayerState<Shielding>) -> Self {
		PlayerStateMachine::Shielding(state)
	}
}

impl From<ShieldingEndState> for PlayerStateMachine {
	fn from(end_state: ShieldingEndState) -> Self {
		match end_state {
			ShieldingEndState::Complete(state) => state.into(),
			ShieldingEndState::Continue(state) => state.into(),
		}
	}
}

impl From<PlayerState<KnockedOut>> for PlayerStateMachine {
	fn from(state: PlayerState<KnockedOut>) -> Self {
		PlayerStateMachine::KnockedOut(state)
	}
}

#[derive(Clone, Debug)]
pub struct PlayerState<S> {
	pub context: PlayerContext,
	pub _state: S,
}

impl<S> PlayerState<S> {

	pub fn context(&self) -> &PlayerContext {
		&self.context
	}

	pub fn set_add_points(&mut self, point: i32) {
		self.context.add_points.append(&mut vec!(point));
	}

	pub fn get_add_points(&self) -> Vec<i32> {
		self.context.add_points.clone()
	}

	pub fn remove_add_points(&mut self) {
		self.context.add_points = vec!();
	}

	pub fn set_velocity_y(&mut self, y: f32) {
		self.context.velocity.y = y;
	}

	pub fn set_velocity_x(&mut self, x: f32) {
		self.context.velocity.x = x;
	}
}

impl PlayerState<Idle> {

	pub fn new(
		velocity_x: f32,
		gravity: f32,
		starting_point: f32,
		audio: Rc<Audio>, 
		sounds: PlayerSounds
	) -> Self {
		PlayerState {
			context: PlayerContext {
				status: Status {
					width: PLAYER_WIDTH,
					height: PLAYER_HEIGHT,
					slash_width: PLAYER_SLASHING_WIDTH,
					slash_height: PLAYER_SLASHING_HEIGHT,
					velocity_x,
					gravity,
					terminal_velocity_y: TERMINAL_VELOCITY,
					starting_point,
					fly_speed: FLY_SPEED,
					radius: 20.0,
					shield_frame: 0,
					frame: 0,
				},
				position: Point {
					x: starting_point,
					y: 50.0
				},
				center: Point {
					x: starting_point + PLAYER_WIDTH / 2.0 + 5.0,
					y: 50.0 + PLAYER_HEIGHT / 2.0 + 5.0
				},
				velocity: Point { x: 0.0, y: 0.0},
				audio,
				sounds,
				add_points: vec!()
			},
			_state: Idle,
		}
	}
	pub fn start(self) -> PlayerState<Falling> {
		PlayerState {
			context: self.context.reset_frame().move_right(),
			_state: Falling,
		}
	}

	pub fn frame_name(&self) -> &str {
		IDLE_FRAME_NAME
	}

	pub fn update(self) -> Self {
		self
	}
}

impl PlayerState<Falling> {

	pub fn frame_name(&self) -> &str {
		FALLING_FRAME_NAME
	}

	pub fn update(mut self) -> Self {
		if self.context.status.frame >= FALLING_FRAMES {
			self.context.status.frame = 12;
		}
		self.context = self.context.update(FALLING_FRAMES, false);
		self
	}

	pub fn fly(self) -> PlayerState<Flying> {

		self.context.play_fly_sound();

		let fly_speed = self.context.clone().status.fly_speed;

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(fly_speed),
			_state: Flying,
		}
	}

	pub fn slash(self) -> PlayerState<Slashing> {

		self.context.play_slash_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_slash_position()
				.set_vertical_velocity(0.0)
				,
			_state: Slashing,
		}
	}

	pub fn beam(self) -> PlayerState<Beaming> {

		self.context.play_beam_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0),
			_state: Beaming,
		}
	}

	pub fn shield(self) -> PlayerState<Shielding> {

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0),
			_state: Shielding
		}
	}

	pub fn knock_out(self) -> PlayerState<KnockedOut> {

		self.context.play_fail_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0)
				.stop(),
			_state: KnockedOut,
		}
	}
}

impl PlayerState<Flying> {

	pub fn frame_name(&self) -> &str {
		FLYING_FRAME_NAME
	}

	pub fn update(mut self) -> FlyingEndState {

		if self.context.status.frame >= FLYING_FRAMES {
			self.context.status.frame = 27;
		}

		self.context = self.context.update(FLYING_FRAMES, false);

		if self.context.position.y <= -16.0 {
			self.context.velocity.y = 0.0;
		}
		if self.context.velocity.y >= 0.0 {
			FlyingEndState::Complete(self.fall())
		} else {
			FlyingEndState::Continue(self)
		}
	}

	pub fn slash(self) -> PlayerState<Slashing> {

		self.context.play_slash_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_slash_position()
				.set_vertical_velocity(0.0),
			_state: Slashing,
		}
	}

	pub fn beam(self) -> PlayerState<Beaming> {

		self.context.play_beam_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0),
			_state: Beaming,
		}
	}

	pub fn shield(self) -> PlayerState<Shielding> {

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0),
			_state: Shielding
		}
	}

	pub fn fall(self) -> PlayerState<Falling> {

		PlayerState {
			context: self
				.context
				.reset_frame(),
			_state: Falling,
		}
	}

	pub fn knock_out(self) -> PlayerState<KnockedOut> {

		self.context.play_fail_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0)
				.stop(),
			_state: KnockedOut,
		}
	}
}

impl PlayerState<Slashing> {
	pub fn frame_name(&self) -> &str {
		SLASHING_FRAME_NAME
	}

	pub fn update(mut self) -> SlashingEndState {

		self.context = self.context.update(SLASHING_FRAMES, true);

		if self.context.status.frame >= SLASHING_FRAMES {
			SlashingEndState::Complete(self.fall())
		} else {
			SlashingEndState::Continue(self)
		}
	}

	pub fn fall(self) -> PlayerState<Falling> {

		PlayerState {
			context: self
				.context
				.reset_frame()
				.remove_slash_position(),
			_state: Falling,
		}
	}

	pub fn knock_out(self) -> PlayerState<KnockedOut> {

		self.context.play_fail_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.remove_slash_position()
				.set_vertical_velocity(0.0)
				.stop(),
			_state: KnockedOut,
		}
	}
}

impl PlayerState<Beaming> {

	pub fn frame_name(&self) -> &str {
		BEAMING_FRAME_NAME
	}

	pub fn update(mut self) -> BeamingEndState {

		self.context = self.context.update(BEAMING_FRAMES, true);

		if self.context.status.frame >= BEAMING_FRAMES {
			BeamingEndState::Complete(self.fall())
		} else {
			BeamingEndState::Continue(self)
		}
	}

	pub fn fall(self) -> PlayerState<Falling> {

		PlayerState {
			context: self
				.context
				.reset_frame(),
			_state: Falling,
		}
	}

	pub fn knock_out(self) -> PlayerState<KnockedOut> {

		self.context.play_fail_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0),
			_state: KnockedOut,
		}
	}
}

impl PlayerState<Shielding> {

	pub fn frame_name(&self) -> &str {
		SHIELDING_FRAME_NAME
	}

	pub fn update(mut self) -> ShieldingEndState {

		self.context = self.context.update(SHIELDING_FRAMES, true);
		self.context.status.shield_frame += 1;

		if self.context.status.shield_frame >= 249 {
			ShieldingEndState::Complete(self.fall())
		} else {
			ShieldingEndState::Continue(self)
		}
	}

	pub fn fall(mut self) -> PlayerState<Falling> {

		self.context.status.shield_frame = 0;

		PlayerState {
			context: self
				.context
				.reset_frame(),
			_state: Falling,
		}
	}

	pub fn fly(mut self) -> PlayerState<Flying> {

		self.context.status.shield_frame = 0;
		self.context.play_fly_sound();
		
		let fly_speed = self.context.clone().status.fly_speed;

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(fly_speed),
			_state: Flying,
		}
	}

	pub fn slash(mut self) -> PlayerState<Slashing> {

		self.context.status.shield_frame = 0;
		self.context.play_slash_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_slash_position()
				.set_vertical_velocity(0.0),
			_state: Slashing,
		}
	}

	pub fn beam(mut self) -> PlayerState<Beaming> {

		self.context.status.shield_frame = 0;
	self.context.play_beam_sound();
		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0),
			_state: Beaming,
		}
	}

	pub fn knock_out(mut self) -> PlayerState<KnockedOut> {

		self.context.status.shield_frame = 0;
		self.context.play_fail_sound();

		PlayerState {
			context: self
				.context
				.reset_frame()
				.set_vertical_velocity(0.0)
				.stop(),
			_state: KnockedOut,
		}
	}
}

impl PlayerState<KnockedOut> {
	pub fn frame_name(&self) -> &str {
		KNOCKED_OUT_FRAME_NAME
	}

	pub fn update(mut self) -> PlayerState<KnockedOut> {

		self.context = self.context.update(KNOCKED_OUT_FRAMES, false);

		PlayerState {
			context: self.context,
			_state: KnockedOut,
		}
	}

	pub fn go_back_to_title(self) -> PlayerState<Idle> {
		PlayerState {
			context: self
				.context
				.set_starting_point()
				.reset_frame(),
			_state: Idle,
		}
	}
}

#[derive(Debug)]
pub enum FlyingEndState {
	Complete(PlayerState<Falling>),
	Continue(PlayerState<Flying>),
}

#[derive(Debug)]
pub enum SlashingEndState {
	Complete(PlayerState<Falling>),
	Continue(PlayerState<Slashing>),
}

#[derive(Debug)]
pub enum BeamingEndState {
	Complete(PlayerState<Falling>),
	Continue(PlayerState<Beaming>),
}

#[derive(Debug)]
pub enum ShieldingEndState {
	Complete(PlayerState<Falling>),
	Continue(PlayerState<Shielding>),
}

#[derive(Clone, Debug)]
pub struct PlayerSounds {
	pub fly: Rc<Sound>,
	pub slash: Rc<Sound>,
	pub beam: Rc<Sound>,
	pub fail: Rc<Sound>
}

#[derive(Clone, Debug)]
pub struct Status {
	pub width: f32,
	pub height: f32,
	pub slash_width: f32,
	pub slash_height: f32,
	pub velocity_x: f32,
	pub gravity: f32,
	pub terminal_velocity_y: f32,
	pub starting_point: f32,
	pub fly_speed: f32,
	pub radius: f32,
	pub shield_frame: u16,
	pub frame: u8,
}
#[derive(Clone, Debug)]
pub struct PlayerContext {
	pub status: Status,
	pub position: Point, // プレイヤーの左上の座標
	pub center: Point,
	pub velocity: Point,
	pub audio: Rc<Audio>,
	pub sounds: PlayerSounds,
	pub add_points: Vec<i32>
}

impl PlayerContext {

	pub fn update(mut self, frame_count: u8, is_hover: bool) -> Self {

		if self.status.frame < frame_count {
			self.status.frame += 1;
		} else {
			self.status.frame = 0;
		}

		if !is_hover {
			if self.velocity.y < self.status.terminal_velocity_y {
				self.velocity.y += self.status.gravity;
			}

			self.position.y += self.velocity.y;
			self.center.y += self.velocity.y;
		}

		self
	}

	fn set_starting_point(mut self) -> Self {
		self.position = Point {
			x: self.status.starting_point + self.status.width / 2.0,
			y: 50.0  + self.status.width / 2.0
		};
		self
	}

	fn reset_frame(mut self) -> Self {
		self.status.frame = 0;
		self
	}

	fn move_right(mut self) -> Self {
		self.velocity.x = self.status.velocity_x;
		self
	}

	fn set_vertical_velocity(mut self, y: f32) -> Self {
		self.velocity.y = y;
		self
	}

	fn set_slash_position(mut self) -> Self {
		self.position.x = self.position.x - ( self.status.slash_width - self.status.width ) / 2.0;
		self.position.y = self.position.y - ( self.status.slash_height - self.status.height ) / 2.0;

		self
	}

	fn remove_slash_position(mut self) -> Self {
		self.position.x = self.position.x + ( self.status.slash_width - self.status.width ) / 2.0;
		self.position.y = self.position.y + ( self.status.slash_height - self.status.height ) / 2.0;

		self
	}

	fn stop(mut self) -> Self {
		self.velocity.x = 0.0;
		self
	}

	fn play_fly_sound(&self){
		if let Err(err) = self.audio.play_sound(&self.sounds.fly) {
			log!("Error playing fly_sound: {}", err);
		}
	}

	fn play_slash_sound(&self) {
		if let Err(err) = self.audio.play_sound(&self.sounds.slash) {
			log!("Error playing slash_sound: {}", err);
		}
	}

	fn play_beam_sound(&self) {
		if let Err(err) = self.audio.play_sound(&self.sounds.beam) {
			log!("Error playing beam_sound: {}", err);
		}
	}

	fn play_fail_sound(&self) {
		if let Err(err) = self.audio.play_sound(&self.sounds.fail) {
			log!("Error playing beam_sound: {}", err);
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Idle;

#[derive(Copy, Clone, Debug)]
pub struct Falling;

#[derive(Copy, Clone, Debug)]
pub struct Slashing;

#[derive(Copy, Clone, Debug)]
pub struct Flying;

#[derive(Copy, Clone, Debug)]
pub struct Beaming;

#[derive(Copy, Clone, Debug)]
pub struct Shielding;

#[derive(Copy, Clone, Debug)]
pub struct KnockedOut;
