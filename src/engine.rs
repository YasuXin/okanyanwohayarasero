use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::rc::Rc;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::sync::mpsc::{
	unbounded_channel,
	UnboundedReceiver,
	error::TryRecvError};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioBuffer, AudioContext, AudioContextState, CanvasRenderingContext2d, HtmlImageElement};
use serde::Deserialize;
use futures::channel::oneshot::channel;
use futures::lock::Mutex;
use rand::{thread_rng, Rng};
use wasm_bindgen::closure::Closure;
use crate::{browser, sound};
use crate::browser::context;
//-----------------------------------------------------------------------------
// ゲームエンジン
// Game の情報と、ゲーム内で使用する画像や音楽、キー入力などの処理を定義する
//-----------------------------------------------------------------------------


const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0; // 毎秒60フレーム

#[async_trait(?Send)]
pub trait Game: Debug {
	async fn initialize(&self) -> Result<Box<dyn Game>, anyhow::Error>;
	fn update(&mut self, key_state: &mut KeyState);
	fn draw(&self, renderer: &Renderer);
}

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;


//--------------------------------
// ループ処理
//--------------------------------
#[derive(Debug)]
pub struct GameLoop {
	last_frame: f64, // 直前のフレームがリクエストされた時刻を格納
	accumulated_delta: f32 // 最後に描画してから累積した差分時間を格納
}

impl GameLoop {
	// ループ処理
	pub async fn start(game: impl Game + 'static) -> Result<(), anyhow::Error> {

		// キーイベントを設定する
		let mut key_event_receiver = prepare_input()?;

		// Game トレイトを実装した FlyingOkanyan 構造体を初期化する
		let mut game = game.initialize().await?;

		// GameLoop 構造体を初期化する
		let mut game_loop = GameLoop {
			last_frame: browser::now()?, // 現在の時刻
			accumulated_delta: 0.0
		};

		let renderer = Renderer {
			context: browser::context()?
		};

		let f: SharedLoopClosure = Rc::new(RefCell::new(None));
		let g: SharedLoopClosure = f.clone();

		let mut key_state = KeyState::new();

		*g.borrow_mut() = Some(browser::create_raf_closure( move | perf: f64 | {
			
			process_input(&mut key_state, &mut key_event_receiver);

			game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;

			while game_loop.accumulated_delta > FRAME_SIZE {

				game.update(&mut key_state);

				game_loop.accumulated_delta -= FRAME_SIZE;

			}

			game_loop.last_frame = perf;
			game.draw(&renderer);

			browser::request_animation_frame(f.borrow().as_ref().unwrap())
				.expect("Animation Frame should exist");

		}));

		browser::request_animation_frame(
			g.borrow()
				.as_ref()
				.ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
		)?;

		Ok(())
	}
}


//-------------------------------------------------------
// x軸とy軸を格納する構造体
//-------------------------------------------------------
#[derive(Clone, Default, Debug)]
pub struct Point {
	pub x: f32,
	pub y: f32
}


//----------------------------------------
// 画像の処理関係の構造体
//----------------------------------------
#[derive(Debug, Clone)]
pub struct Image {
	pub element: Rc<HtmlImageElement>,
	pub bounding_box: Rect,
	pub destination: Rect
}

impl Image {
	pub fn new(element: Rc<HtmlImageElement>, position: Point, destination: Rect) -> Self {
		let bounding_box = Rect::new(
			position,
			element.width() as f32,
			element.height() as f32
		);

		Self { element, bounding_box, destination }
	}

	pub fn draw(
		&self,
		renderer: &Renderer
	) {
		renderer.draw_image(&self.element, &self.bounding_box, &self.destination);
	}

	pub fn draw_bg(&self, renderer: &Renderer) {
		renderer.draw_entire_image(&self.element, &self.bounding_box.position)
	}


	pub fn move_horizontally(&mut self, distance: f32) {
		self.bounding_box.set_x(self.bounding_box.x() + distance);
		self.destination.set_x(self.destination.x() + distance);
	}

	pub fn set_x(&mut self, x: f32) {
		self.bounding_box.set_x(x);
		self.destination.set_x(x);
	}

	pub fn right(&self) -> f32 {
		self.bounding_box.right()
	}
	
	pub fn shake_bounding_box(&mut self) {

		let mut rng = thread_rng();
		let shake_x = rng.gen_range(0..=10) as f32;
		let shake_y = rng.gen_range(0..=10) as f32;
		
		self.bounding_box.position.x = self.bounding_box.position.x + shake_x;
		self.bounding_box.position.y = self.bounding_box.position.y + shake_y;
		self.destination.position.x = self.destination.position.x + shake_x;
		self.destination.position.y = self.destination.position.y + shake_y;
	}
	
	pub fn reset_bounding_box(&mut self) {
		self.bounding_box.position.x = 0.0;
		self.bounding_box.position.y = 0.0;
		self.destination.position.x = 0.0;
		self.destination.position.y = 0.0;
	}
}

#[derive(Clone, Debug)]
pub struct SpriteSheet {
	pub sheet: Sheet,
	pub image: HtmlImageElement,
}

impl SpriteSheet {
	pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
		SpriteSheet { sheet, image }
	}

	pub fn cell(&self, name: &str) -> Option<&Cell> {
		self.sheet.frames.get(name)
	}

	pub fn draw(&self, renderer: &Renderer, source: &Rect, destination: &Rect) {
		renderer.draw_image(&self.image, source, destination);
	}
}

#[derive(Deserialize, Clone, Debug)]
pub struct SheetRect {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32
}

#[derive(Deserialize, Clone, Debug)]
pub struct Sheet {
	pub frames: HashMap<String, Cell>
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
	pub frame: SheetRect,
	pub sprite_source_size: SheetRect
}

#[derive(Default, Debug, Clone)]
pub struct Rect {
	pub position: Point,
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub const fn new(position: Point, width: f32, height: f32) -> Self {
		Rect { position, width, height }
	}
	pub const fn new_from_x_y(x: f32, y: f32, width: f32, height: f32) -> Self {
		Rect::new(Point {x, y}, width, height)
	}
	// 衝突判定
	// 2つのオブジェクトが衝突している場合は true を返す
	pub fn intersects(&self, rect: &Rect) -> bool {
		self.x() < (rect.x() + rect.width) && self.x() + self.width > rect.x()
			&& self.y() < (rect.y() + rect.height) && self.y() + self.height > rect.y()
	}

	pub fn x(&self) -> f32 {
		self.position.x
	}
	pub fn set_x(&mut self, x: f32) {
		self.position.x = x;
	}
	pub fn y(&self) -> f32 {
		self.position.y
	}
	pub fn set_y(&mut self, y: f32) {
		self.position.y = y;
	}

	pub fn right(&self) -> f32 {
		self.x() + self.width
	}
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement, anyhow::Error> {
	
	let image = browser::new_image()?;

	let (complete_tx, complete_rx) = channel::<Result<(), anyhow::Error>>();

	let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
	let error_tx = Rc::clone(&success_tx);

	let success_callback = browser::closure_once( move || {
		let success_tx = success_tx.clone();
		browser::spawn_local( async move {
			if let Some(success_tx) = success_tx.lock().await.take() {
				success_tx.send(Ok(())).ok();
			}
		})
	});

	let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once( move | err | {
		let error_tx = error_tx.clone();
		browser::spawn_local( async move {
			if let Some(error_tx) = error_tx.lock().await.take() {
				error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err))).expect("TODO: panic message");
			}
		})
	});

	image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
	image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
	image.set_src(source);

	complete_rx.await??;

	Ok(image)
}


//---------------------------------
// レンダラ
//---------------------------------
#[derive(Clone, Debug)]
pub struct Renderer {
	context: CanvasRenderingContext2d,
}

impl Renderer {
	pub fn clear(&self, rect: &Rect) {
		self.context.clear_rect(
			rect.x().into(),
			rect.y().into(),
			rect.width.into(),
			rect.height.into()
		);
	}
	pub fn draw_image(
		&self,
		image: &HtmlImageElement,
		frame: &Rect,
		destination: &Rect
	) {
		self.context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
			&image,
			frame.x().into(),
			frame.y().into(),
			frame.width.into(),
			frame.height.into(),
			destination.x().into(),
			destination.y().into(),
			destination.width.into(),
			destination.height.into()
		)
			.expect("Drawing is throwing exceptions! Unrecoverable error.");
	}

	pub fn draw_entire_image(
		&self,
		image: &HtmlImageElement,
		position: &Point
	) {
		self.context.draw_image_with_html_image_element(image, position.x.into(), position.y.into())
			.expect("Drawing is throwing exceptions! Unrecoverable error.");
	}
}


//----------------------------
// 音楽の処理
//----------------------------
#[derive(Clone, Debug)]
pub struct Audio {
	context: AudioContext,
}

impl Audio {
	pub fn new() -> Result<Self, anyhow::Error> {
		Ok(Audio { context: sound::create_audio_context()? })
	}

	pub async fn load_sound(&self, filename: &str) -> Result<Sound, anyhow::Error> {
		let array_buffer = browser::fetch_array_buffer(filename).await?;
		let audio_buffer = sound::decode_audio_data(&self.context, &array_buffer).await?;

		Ok( Sound { buffer: audio_buffer } )
	}

	// 効果音など、一度だけ音源を再生する
	pub fn play_sound(&self, sound: &Sound) -> Result<(), anyhow::Error> {
		sound::play_sound(&self.context, &sound.buffer, sound::LOOPING::NO)
	}

	// BGMなど、繰り返し音源を再生する
	pub fn play_looping_sound(&self, sound: &Sound) -> Result<(), anyhow::Error> {
		sound::play_sound(&self.context, &sound.buffer, sound::LOOPING::YES)
	}

	// 一時停止した音源を再度再生します
	pub fn resume_sound(&self) {
		if self.context.state() == AudioContextState::Suspended {
			let _result = self.context.resume().expect("Couldn't resume sound");
		}
	}

	// 音源を一時停止します
	pub fn suspend_sound(&self) {
		let _result = self.context.suspend().expect("Couldn't suspend sound");
	}

}

#[derive(Clone, Debug)]
pub struct Sound {
	pub buffer: AudioBuffer
}


//--------------------------------------------
// キー入力処理
//--------------------------------------------
#[derive(Clone, Debug)]
pub struct KeyFlag {
	pub arrow_up: bool,
	pub arrow_down: bool,
	pub arrow_left: bool,
	pub arrow_right: bool,
	pub space: bool,
}

#[derive(Clone, Debug)]
pub enum KeyPress {
	KeyUp(web_sys::KeyboardEvent),
	KeyDown(web_sys::KeyboardEvent)
}

#[derive(Clone, Debug)]
pub struct KeyState {
	pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
	pub flag: KeyFlag,
}

impl KeyState {
	fn new() -> Self {
		KeyState {
			pressed_keys: HashMap::new(),
			flag: KeyFlag {
				arrow_up: false,
				arrow_right: false,
				arrow_left: false,
				arrow_down: false,
				space: false
			}
		}
	}

	pub fn is_pressed(&self, code: &str) -> bool {
		self.pressed_keys.contains_key(code)
	}

	pub fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
		self.pressed_keys.insert(code.into(), event);
	}

	pub fn set_released(&mut self, code: &str) {
		match code {
			"ArrowUp" => self.flag.arrow_up = false,
			"ArrowDown" => self.flag.arrow_down = false,
			"ArrowLeft" => self.flag.arrow_left = false,
			"ArrowRight" => self.flag.arrow_right = false,
			"Space" => self.flag.space = false,
			_ => {}
		}
		self.pressed_keys.remove(code);
	}
}

fn prepare_input() -> Result<UnboundedReceiver<KeyPress>, anyhow::Error> {

	let (keydown_sender, key_event_receiver) = unbounded_channel();

	let keyup_sender = keydown_sender.clone();

	let onkeydown = browser::closure_wrap(
		Box::new( move | keycode: web_sys::KeyboardEvent | {
			keydown_sender
				.send(KeyPress::KeyDown(keycode))
				.expect("keydown_sender failed");
		}) as Box<dyn FnMut(web_sys::KeyboardEvent)>
	);

	let onkeyup = browser::closure_wrap(
		Box::new( move | keycode: web_sys::KeyboardEvent | {
			keyup_sender
				.send(KeyPress::KeyUp(keycode))
				.expect("keyup_sender failed");
		}) as Box<dyn FnMut(web_sys::KeyboardEvent)>
	);

	browser::window()?
		.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));

	browser::window()?
		.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));

	onkeydown.forget();
	onkeyup.forget();

	Ok(key_event_receiver)
}

fn process_input(
	state: &mut KeyState,
	key_event_receiver: &mut UnboundedReceiver<KeyPress>
) {
	loop {
		match key_event_receiver.try_recv() {
			Err(TryRecvError::Empty) => break,
			Err(TryRecvError::Disconnected) => break,
			Ok(evt) => match evt {
				KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
				KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt)
			},
		};
	}
}


//---------------------------------------
// クリックイベントを登録する
//---------------------------------------
/*
pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {

	let (click_sender, click_receiver) = unbounded_channel();

	let on_click = browser::closure_wrap(
		Box::new(move || {
			click_sender
				.send(())
				.expect("engine : add_click_handler() : click handler error");

		}) as Box<dyn FnMut()>
	);

	elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));

	on_click.forget();

	click_receiver
}
*/
//--------------------------
// 図形の描画
//--------------------------

pub fn draw_modal_background(color_str: &str) {

	let context = context().expect("context not initialized!");

	context.begin_path();
	context.line_to(1200.0, 0.0);
	context.line_to(1200.0, 560.0);
	context.line_to(0.0, 560.0);
	context.line_to(0.0, 0.0);
	context.close_path();
	context.set_fill_style_str(color_str);
	context.fill();
}

pub fn draw_circle(color_str: &str, position: &Point, radius: f32) {

	let context = context().expect("context not initialized!");

	context.begin_path();
	context.arc(
		position.x as f64,
		position.y as f64,
		radius as f64,
		0.0,
		2.0 * PI
	).expect("TODO: panic message");
	context.set_fill_style_str(color_str);
	context.fill();
}