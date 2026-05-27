use std::{
	fmt::Debug,
	rc::Rc
};
use anyhow::anyhow;
use async_trait::async_trait;
use rand::{thread_rng, Rng};
use crate::{
	browser,
	engine
};
use crate::engine::{Audio, Game, Image, KeyState, Point, Rect, Renderer, Sheet, SpriteSheet, draw_modal_background};
use crate::obstacle::{right_most, ObstacleKinds};
use crate::player::{Player, PlayerSounds, PlayerStateMachine};
use crate::segment::{start_platform, start_platform_bg};
use wasm_bindgen::JsValue;
use serde_wasm_bindgen;
use crate::browser::get_canvas_width;
use crate::data_for_playing::{BGMs, Backgrounds, DataForPlaying, GameSounds, SpecialImages, SpriteSheets, Status};
use crate::special_kazama::{KazamaSounds, SpecialKazama};
use crate::special_okanyan::{OkanyanSounds, SpecialOkanyan};

//const WIDTH: u16 = 1200;
//const HEIGHT: u16 = 560;

const TIME_LINE_MINIMUM: u16 = 1200;

const PLAYER_JSON: &str = "player_sheet.json";

const PLAYER_IMAGE: &str = "player_sheet.png";
const BEAM_IMAGE: &str = "beam_2.png";

const ENEMY_JSON: &str = "enemy_sheet.json";
const ENEMY_SHEET: &str = "enemy_sheet.png";

const FUBURA_JSON: &str = "fubura_sheet.json";
const FUBURA_SHEET: &str = "fubura_sheet.png";

const HATO_JSON: &str = "hato_sheet.json";
const HATO_SHEET: &str = "hato_sheet.png";
const KEDAMA_JSON: &str = "kedama_sheet.json";
const KEDAMA_SHEET: &str = "kedama_sheet.png";

const BONUS_JSON: &str = "bonus_sheet.json";
const BONUS_SHEET: &str = "bonus_sheet.png";

const KAZAMA_JSON: &str = "kazama_sheet.json";
const KAZAMA_SHEET: &str = "kazama_sheet.png";

const OKANYAN_SPECIAL_0: &str = "okanyan_special_0.png";
const OKANYAN_SPECIAL_1: &str = "okanyan_special_1.png";
const OKANYAN_SPECIAL_ENERGY: &str = "okanyan_special_energy.png";


const KAZAMA_SLASH_0: &str = "kazama_slash_0.png";
const KAZAMA_SLASH_1: &str = "kazama_slash_1.png";
const KAZAMA_SLASH_2: &str = "kazama_slash_2.png";
const KAZAMA_SLASH_3: &str = "kazama_slash_3.png";
const KAZAMA_BG: &str = "kazama_bg.png";

const BGM_PLAYING: &str = "bgm_1.mp3";

const FLY_SE: &str = "fly.mp3";
const SLASH_SE: &str = "slash.mp3";
const BEAM_SE: &str = "beam.mp3";
const FAIL_SE: &str = "fail.mp3";
const DECISION_SE: &str = "decision.mp3";

const SPECIAL_SE: &str = "special.mp3";
const SPECIAL_ENERGY_SE: &str = "special_energy.mp3";

const ENEMY_BEAM_SE: &str = "enemy_beam.mp3";
const ENEMY_DEFEATED_SE: &str = "enemy_defeated.mp3";

const FUBURA_THUNDER_SE: &str = "fubura_thunder.mp3";
const FUBURA_WULF_SE: &str = "fubura_wulf.mp3";
const FUBURA_DOGON_SE: &str = "fubura_dogon.mp3";
const HATO_EYE_SE: &str = "hato_eye.mp3";
const KOTO_SE : &str = "koto.mp3";
const SWORD_0_SE: &str = "sword_0.mp3";
const SWORD_1_SE: &str = "sword_1.mp3";

const BONUS_SE: &str = "bonus.mp3";
const ENERGY_MAX_SE: &str = "energy_max.mp3";

// 背景画像
const TITLE_BG: &str = "title_bg.png";
const PLAYING_BG_0: &str = "playing_bg_0.png";
const PLAYING_BG_1: &str = "playing_bg_1.png";
const PLAYING_BG_2: &str = "playing_bg_2.png";

const PLATFORM_JSON: &str = "platform_sheet.json";
const PLATFORM_IMAGE: &str = "platform_sheet.png";

const STARTING_POINT: u16 = 180;
const MOVING_SPEED: f32 = 2.4;
const GRAVITY: f32 = 0.2;

#[async_trait(?Send)]
impl Game for FlyingOkanyan {

	async fn initialize(&self) -> Result<Box<dyn Game>, anyhow::Error> {

		match self.machine {

			None => {

				let width = get_canvas_width().expect("error getting canvas width");
				let height = width * 56 / 120;


				// 音楽データ読み込み
				let audio = Rc::new(Audio::new()?);
				let fly = Rc::new(audio.load_sound(FLY_SE).await.expect("Failed to load flying sound"));
				let slash = Rc::new(audio.load_sound(SLASH_SE).await.expect("Failed to load slash sound"));
				let energy = Rc::new(audio.load_sound(BEAM_SE).await.expect("Failed to load beam sound"));
				let fail = Rc::new(audio.load_sound(FAIL_SE).await.expect("Failed to load fail sound"));
				let decision = Rc::new(audio.load_sound(DECISION_SE).await.expect("Failed to load decision sound"));
				let enemy_beam = Rc::new(audio.load_sound(ENEMY_BEAM_SE).await.expect("Failed to load enemy beam sound"));
				let enemy_defeated = Rc::new(audio.load_sound(ENEMY_DEFEATED_SE).await.expect("Failed to load enemy defeated sound"));
				let fubura_thunder = Rc::new(audio.load_sound(FUBURA_THUNDER_SE).await.expect("Failed to load fubura thunder sound"));
				let fubura_wulf = Rc::new(audio.load_sound(FUBURA_WULF_SE).await.expect("Failed to load fubura wulf sound"));
				let fubura_dogon = Rc::new(audio.load_sound(FUBURA_DOGON_SE).await.expect("Failed to load fubura dogon sound"));
				let hato_eye = Rc::new(audio.load_sound(HATO_EYE_SE).await.expect("Failed to load hato eye sound"));
				let special = Rc::new(audio.load_sound(SPECIAL_SE).await.expect("Failed to load special sound"));
				let special_energy = Rc::new(audio.load_sound(SPECIAL_ENERGY_SE).await.expect("Failed to load special energy sound"));
				let special_koto =  Rc::new(audio.load_sound(KOTO_SE).await.expect("Failed to load koto sound"));
				let special_sword_0 =  Rc::new(audio.load_sound(SWORD_0_SE).await.expect("Failed to load sword sound"));
				let special_sword_1 =  Rc::new(audio.load_sound(SWORD_1_SE).await.expect("Failed to load sword sound"));
				let bonus =  Rc::new(audio.load_sound(BONUS_SE).await.expect("Failed to load bonus sound"));
				let energy_max =  Rc::new(audio.load_sound(ENERGY_MAX_SE).await.expect("Failed to load energy max sound"));

				let beam = energy.clone();

				let sounds = PlayerSounds {
					fly, slash, beam, fail
				};

				//
				let playing_audio = Audio::new()?;
				let playing_sound = playing_audio.load_sound(BGM_PLAYING).await.expect("Failed to load background sound");

				let bgm = BGMs {
					current_audio: None,
					_current_sound: None,
					playing_audio,
					_playing_sound: playing_sound,
				};

				let player_json: JsValue = browser::fetch_json(PLAYER_JSON)
					.await
					.expect("player_sheet.json does not exists");

				let player_sheet: Sheet = serde_wasm_bindgen::from_value(player_json)
					.expect("failed to deserialize player JSON");

				let player = Player::new(
					MOVING_SPEED,
					GRAVITY,
					STARTING_POINT as f32,
					player_sheet,
					engine::load_image(PLAYER_IMAGE)
						.await
						.expect("player_sheet.png does not exists"),
					audio.clone(),
					sounds
				);

				let beam_image = engine::load_image(BEAM_IMAGE)
					.await
					.expect("beam.png does not exists");

				let enemy_json: JsValue = browser::fetch_json(ENEMY_JSON)
					.await
					.expect("enemy_sheet.json does not exists");

				let enemy_sheet: Sheet = serde_wasm_bindgen::from_value(enemy_json)
					.expect("failed to deserialize player JSON");

				let enemy_sprite_sheet = Rc::new(
					SpriteSheet::new(
						enemy_sheet,
						engine::load_image(ENEMY_SHEET)
							.await
							.expect("enemy_sheet.png does not exists")
					)
				);

				let fubura_json: JsValue = browser::fetch_json(FUBURA_JSON)
					.await
					.expect("enemy_sheet.json does not exists");

				let fubura_sheet: Sheet = serde_wasm_bindgen::from_value(fubura_json)
					.expect("failed to deserialize fubura JSON");

				let fubura_sprite_sheet = Rc::new(
					SpriteSheet::new(
						fubura_sheet,
						engine::load_image(FUBURA_SHEET)
							.await
							.expect("fubura_sheet.png does not exists")
					)
				);

				let hato_json: JsValue = browser::fetch_json(HATO_JSON)
					.await
					.expect("enemy_sheet.json does not exists");

				let hato_sheet: Sheet = serde_wasm_bindgen::from_value(hato_json)
					.expect("failed to deserialize hato JSON");

				let hato_sprite_sheet = Rc::new(
					SpriteSheet::new(
						hato_sheet,
						engine::load_image(HATO_SHEET)
							.await
							.expect("hato_sheet.png does not exists")
					)
				);

				let kedama_json: JsValue = browser::fetch_json(KEDAMA_JSON)
					.await
					.expect("enemy_sheet.json does not exists");

				let kedama_sheet: Sheet = serde_wasm_bindgen::from_value(kedama_json)
					.expect("failed to deserialize kedama JSON");

				let kedama_sprite_sheet = Rc::new(
					SpriteSheet::new(
						kedama_sheet,
						engine::load_image(KEDAMA_SHEET)
							.await
							.expect("kedama_sheet.png does not exists")
					)
				);

				let bonus_json: JsValue = browser::fetch_json(BONUS_JSON)
					.await
					.expect("bonus_sheet.json does not exists");

				let bonus_sheet: Sheet = serde_wasm_bindgen::from_value(bonus_json)
					.expect("failed to deserialize bonus JSON");

				let bonus_sprite_sheet = Rc::new(
					SpriteSheet::new(
						bonus_sheet,
						engine::load_image(BONUS_SHEET)
							.await
							.expect("bonus_sheet.png does not exists")
					)
				);

				let kazama_json: JsValue = browser::fetch_json(KAZAMA_JSON)
					.await
					.expect("kazama_sheet.json does not exists");

				let kazama_sheet: Sheet = serde_wasm_bindgen::from_value(kazama_json).
					expect("failed to deserialize kazama JSON");

				let kazama_sprite_sheet = Rc::new(
					SpriteSheet::new(
						kazama_sheet,
						engine::load_image(KAZAMA_SHEET)
							.await
							.expect("kazama_sheet.png does not exists"))
				);

				let title_bg_image = engine::load_image(TITLE_BG)
					.await
					.expect("title_bg.png does not exists");

				let playing_bg_image_0 = engine::load_image(PLAYING_BG_0)
					.await
					.expect("playing_bg.png does not exists");

				let playing_bg_image_1 = engine::load_image(PLAYING_BG_1)
					.await
					.expect("playing_bg.png does not exists");

				let playing_bg_image_2 = engine::load_image(PLAYING_BG_2)
					.await
					.expect("playing_bg.png does not exists");

				let special_image_0 = engine::load_image(OKANYAN_SPECIAL_0)
					.await
					.expect("okanyan_special_0.png does not exists");

				let special_image_1 = engine::load_image(OKANYAN_SPECIAL_1)
					.await
					.expect("okanyan_special_1.png does not exists");

				let special_energy_image = engine::load_image(OKANYAN_SPECIAL_ENERGY)
					.await
					.expect("okanyan_special_energy.png does not exists");

				let kazama_slash_image_0 = engine::load_image(KAZAMA_SLASH_0)
					.await
					.expect("kazama_slash_0.png does not exists");
				let kazama_slash_image_1 = engine::load_image(KAZAMA_SLASH_1)
					.await
					.expect("kazama_slash_1.png does not exists");
				let kazama_slash_image_2 = engine::load_image(KAZAMA_SLASH_2)
					.await
					.expect("kazama_slash_2.png does not exists");
				let kazama_slash_image_3 = engine::load_image(KAZAMA_SLASH_3)
					.await
					.expect("kazama_slash_3.png does not exists");

				let kazama_bg_image = engine::load_image(KAZAMA_BG)
					.await
					.expect("kazama_bg.png does not exists");

				let destination = Rect::new(
					Point { x: 0.0, y: 0.0 },
					width as f32,
					height as f32
				);

				let title_bgs = [
					Image::new(
						Rc::new(title_bg_image),
						Point{ x: 0.0, y: 0.0 },
						destination.clone()
					)
				];

				let playing_bgs_0 = [
					Image::new(
						Rc::new(playing_bg_image_0),
						Point{ x: 0.0, y: 0.0 },
						destination.clone()
					)
				];

				let playing_bgs_1 = [
					Image::new(
						Rc::new(playing_bg_image_1.clone()),
						Point{ x: 0.0, y: 0.0 },
						destination.clone()
					),
					Image::new(
						Rc::new(playing_bg_image_1),
						Point{ x: width as f32, y: 0.0 },
						destination.clone()
					)
				];

				let playing_bgs_2 = [
					Image::new(
						Rc::new(playing_bg_image_2.clone()),
						Point{ x: 0.0, y: 0.0 },
						destination.clone()
					),
					Image::new(
						Rc::new(playing_bg_image_2),
						Point{ x: width as f32, y: 0.0 },
						destination.clone()
					)
				];

				let special_images = SpecialImages {
					okanyan: [
						Image::new(
							Rc::new(special_image_0),
							Point {x: 0.0, y:0.0},
							destination.clone()
						),
						Image::new(
							Rc::new(special_image_1),
							Point {x: 0.0, y: 0.0},
							destination.clone()
						),
						Image::new(
							Rc::new(special_energy_image),
							Point {x: 0.0, y: 0.0},
							destination.clone()
						)
					],
					kazama: [
						Image::new(
							Rc::new(kazama_slash_image_0),
							Point {x: 0.0, y: 0.0},
							destination.clone()
						),
						Image::new(
							Rc::new(kazama_slash_image_1),
							Point {x: 0.0, y: 0.0},
							destination.clone()
						),
						Image::new(
							Rc::new(kazama_slash_image_2),
							Point {x: 0.0, y: 0.0},
							destination.clone()
						),
						Image::new(
							Rc::new(kazama_slash_image_3),
							Point {x: 0.0, y: 0.0},
							destination.clone()
						),
						Image::new(
							Rc::new(kazama_bg_image),
							Point {x: 0.0, y: 0.0},
							destination
						),
					]
				};

				let platform_json = browser::fetch_json(PLATFORM_JSON)
					.await
					.expect("tiles.json does not exists");

				let platform_sheet = serde_wasm_bindgen::from_value(platform_json)
					.expect("failed to deserialize player JSON");

				let platform_sprite_sheet = Rc::new(
					SpriteSheet::new(
						platform_sheet,
						engine::load_image(PLATFORM_IMAGE)
							.await
							.expect("tiles.png does not exists")
					)
				);

				let starting_obstacles = start_platform(
					platform_sprite_sheet.clone(),
					0.0,
					height as f32
				);

				let starting_checker = start_platform_bg(
					platform_sprite_sheet.clone(),
					0.0
				);

				let time_line = right_most(&starting_obstacles) as f32;
				
				let sounds = GameSounds {
					decision,
					enemy_beam,
					enemy_energy: energy,
					enemy_defeated,
					fubura_thunder,
					fubura_wulf,
					fubura_dogon,
					hato_eye,
					special,
					special_energy,
					special_koto,
					special_sword_0,
					special_sword_1,
					bonus,
					energy_max
				};

				let sprite_sheets = SpriteSheets {
					kazama_sheet: kazama_sprite_sheet,
					bonus_sheet: bonus_sprite_sheet,
					obstacle_sheet: platform_sprite_sheet,
					enemies_sheet: enemy_sprite_sheet,
					fubura_sheet: fubura_sprite_sheet,
					hato_sheet: hato_sprite_sheet,
					kedama_sheet: kedama_sprite_sheet,
				};

				let machine = GameStateMachine::new(
					DataForPlaying {
						status: Status {
							width: width as f32,
							height: height as f32,
							gravity: GRAVITY,
							time_line,
							frame: 0,
							score: 0,
							special_energy: 0,
						},
						player,
						backgrounds: Backgrounds {
							title: title_bgs,
							playing_0: playing_bgs_0,
							playing_1: playing_bgs_1,
							playing_2: playing_bgs_2,
						},
						special_images,
						special: None,
						sprite_sheets,
						balloons: Vec::new(),
						obstacles: starting_obstacles,
						obstacles_bg: starting_checker,
						beam_image,
						beams: Vec::new(),
						enemies: Vec::new(),
						objects: Vec::new(),
						audio,
						sounds,
						bgm,
					}
				);

				let _receiver = browser::set_class_name_by_id("title-area", "")
					.expect("game : FlyingOkanyan : initialize() : error draw text");

				Ok(
					Box::new(
						FlyingOkanyan {
							machine: Some(machine)
						}
					)
				)
			},
			Some(_) => {
				Err(anyhow!("Error: Game is already initialized!"))
			}
		}
	}

	fn update(&mut self, key_state: &mut KeyState) {

		if let Some(machine) = self.machine.take() {
			let _old = self.machine.replace(machine.update(key_state));
		}
	}
	
	fn draw(&self, renderer: &Renderer) {
		let width: f32;
		if let Some(machine) = self.machine.as_ref() {
			width = machine.get_width()
		} else {
			width = 0.0;
		}

		renderer.clear(
			&Rect::new(
				Point {
					x: 0.0,
					y: 0.0
				},
				width , width * 560.0 / 1200.0
			)
		);

		if let Some(machine) = &self.machine {
			machine.draw(renderer);
		}
	}
}

#[derive(Debug)]
pub struct FlyingOkanyan {
	machine: Option<GameStateMachine>
}

impl FlyingOkanyan {
	pub fn new() -> Self {
		FlyingOkanyan { machine: None }
	}
}

#[derive(Clone, Debug)]
struct Title;
#[derive(Clone, Debug)]
struct Playing;
#[derive(Clone, Debug)]
struct SpecialMove;
#[derive(Clone, Debug)]
struct GameOver;

#[derive(Debug)]
enum GameStateMachine {
	Title(GameState<Title>),
	Playing(GameState<Playing>),
	SpecialMove(GameState<SpecialMove>),
	GameOver(GameState<GameOver>)
}

impl GameStateMachine {
	
	fn new(data_for_playing: DataForPlaying) -> Self {
		GameStateMachine::Title(GameState::new(data_for_playing))
	}
	
	fn update(self, key_state: &mut KeyState) -> Self {
		match self {
			GameStateMachine::Title(state) => state.update(key_state).into(),
			GameStateMachine::Playing(state) => state.update(key_state).into(),
			GameStateMachine::SpecialMove(state) => state.update().into(),
			GameStateMachine::GameOver(state) => state.update(key_state).into(),
		}
	}
	
	fn draw(&self, renderer: &Renderer) {
		match self {
			GameStateMachine::Title(state) => state.draw(renderer),
			GameStateMachine::Playing(state) => state.draw(renderer),
			GameStateMachine::SpecialMove(state) => state.draw(renderer),
			GameStateMachine::GameOver(state) => state.draw(renderer),
		}
	}

	fn get_width(&self) -> f32 {
		match self {
			GameStateMachine::Title(state) => state.data_for_playing.status.width,
			GameStateMachine::Playing(state) => state.data_for_playing.status.width,
			GameStateMachine::SpecialMove(state) => state.data_for_playing.status.width,
			GameStateMachine::GameOver(state) => state.data_for_playing.status.width,
		}
	}
}

#[derive(Debug)]
struct GameState<T> {
	_state: T,
	data_for_playing: DataForPlaying
}

impl GameState<Title> {
	
	fn new(data_for_playing: DataForPlaying) -> Self {

		GameState {
			_state: Title,
			data_for_playing
		}
	}
	
	fn start_move(mut self) -> GameState<Playing> {

		self.data_for_playing.play_decision_sound();

		if self.data_for_playing.bgm.current_audio.is_none() &&
			self.data_for_playing.bgm._current_sound.is_none() {

			self.data_for_playing.bgm.current_audio = Some(self.data_for_playing.bgm.playing_audio.clone());
			self.data_for_playing.bgm._current_sound = Some(self.data_for_playing.bgm._playing_sound.clone());

			if let Some(audio) = self.data_for_playing.bgm.current_audio.take() &&
				let Some(sound) = self.data_for_playing.bgm._current_sound.take() {

				audio.play_looping_sound(&sound)
					.expect("Error when playing sound");

				self.data_for_playing.bgm.current_audio = Some(audio);
				self.data_for_playing.bgm._current_sound = Some(sound);
			}
		} else {
			if let Some(audio) = self.data_for_playing.bgm.current_audio.take() &&
				let Some(sound) = self.data_for_playing.bgm._current_sound.take() {

				audio.resume_sound();
				self.data_for_playing.bgm.current_audio = Some(audio);
				self.data_for_playing.bgm._current_sound = Some(sound);
			}
		}

		let _receiver = browser::set_class_name_by_id("right-top-area", "")
			.expect("game : GameState : start_move() : error draw text");

		let _receiver = browser::set_class_name_by_id("title-area", "hidden")
			.expect("game : GameState : start_move() : error draw text");

		GameState {
			_state: Playing,
			data_for_playing: self.data_for_playing
		}
	}
	
	fn update(mut self, key_state: &mut KeyState) -> TitleEndState {

		if key_state.is_pressed("ArrowRight") && !key_state.flag.arrow_right {
			key_state.flag.arrow_right = true;

			let _receiver = browser::set_class_name_by_id("explain-area", "")
				.expect("game : GameState : update() : error draw text");
		}

		if key_state.is_pressed("ArrowLeft") && !key_state.flag.arrow_left {
			key_state.flag.arrow_left = true;

			let _receiver = browser::set_class_name_by_id("explain-area", "hidden")
				.expect("game : GameState : update() : error draw text");
		}

		if key_state.is_pressed("Space") && !key_state.flag.space {

			key_state.flag.space = true;

			let _receiver = browser::set_class_name_by_id("explain-area", "hidden")
				.expect("game : GameState : update() : error draw text");

			self.data_for_playing.player.start_right();

			TitleEndState::Complete(self.start_move())

		} else {

			TitleEndState::Continue(self)
		}
	}

	pub fn draw(&self, renderer: &Renderer) {
		self.data_for_playing.draw_title_bg(renderer);
	}
}

impl GameState<Playing> {

	fn update(mut self, key_state: &mut KeyState) -> PlayingEndState {

		match self.data_for_playing.player.state_machine.clone() {

			PlayerStateMachine::KnockedOut(_) => PlayingEndState::Complete(self.end_game()),

			_ => {

				if self.data_for_playing.player.pos_y() > self.data_for_playing.status.height as f32 {
					self.data_for_playing.player.knock_out();
				}

				if key_state.is_pressed("Space") && !key_state.flag.space &&
					self.data_for_playing.status.special_energy >= 100 {

					key_state.flag.space = true;

					PlayingEndState::Special(self.do_special_move())

				} else {
					if key_state.is_pressed("ArrowUp") && !key_state.flag.arrow_up {

						key_state.flag.arrow_up = true;
						self.data_for_playing.player.fly();

					}

					if key_state.is_pressed("ArrowDown") && !key_state.flag.arrow_down {

						key_state.flag.arrow_down = true;
						self.data_for_playing.player.slash();

					}

					if key_state.is_pressed("ArrowLeft") && !key_state.flag.arrow_left &&
						self.data_for_playing.status.special_energy >= 50 {

						self.data_for_playing.status.special_energy -= 50;

						key_state.flag.arrow_left = true;
						self.data_for_playing.player.shield();
					}

					if key_state.is_pressed("ArrowRight") && !key_state.flag.arrow_right &&
						self.data_for_playing.status.special_energy >= 25 {

						self.data_for_playing.generate_beam();

						self.data_for_playing.status.special_energy -= 25;

						key_state.flag.arrow_right = true;
						self.data_for_playing.player.beam();

					}

					let velocity_x = self.data_for_playing.velocity_x();
					
					let velocity_bg_2 = velocity_x / 2.0;

					// 一番前の背景の処理　プレイヤーと同じ速度で移動する
					let [first_bg, second_bg] = &mut self.data_for_playing.backgrounds.playing_2;

					first_bg.move_horizontally(velocity_bg_2);
					second_bg.move_horizontally(velocity_bg_2);

					if first_bg.right() < 0.0 {
						first_bg.set_x(second_bg.right());
					}

					if second_bg.right() < 0.0 {
						second_bg.set_x(first_bg.right());
					}

					let velocity_bg_1 = velocity_x / 4.0;

					// 後ろのビルのの背景の処理　低速で移動する
					let [first_bg, second_bg] = &mut self.data_for_playing.backgrounds.playing_1;
					first_bg.move_horizontally(velocity_bg_1);
					second_bg.move_horizontally(velocity_bg_1);

					if first_bg.right() < 0.0 {
						first_bg.set_x(second_bg.right());
					}

					if second_bg.right() < 0.0 {
						second_bg.set_x(first_bg.right());
					}

					// 地形と障害物に関する処理
					self.data_for_playing.obstacles_bg.retain(|obstacle| obstacle.right() > -50);
					self.data_for_playing.obstacles_bg.iter_mut().for_each(|obstacle| {

						obstacle.update(
							velocity_x,
							&mut self.data_for_playing.player,
							&mut self.data_for_playing.beams
						);
					});

					self.data_for_playing.obstacles.retain(|obstacle| obstacle.right() > -50);
					self.data_for_playing.obstacles.iter_mut().for_each(|obstacle| {

						obstacle.update(
							velocity_x,
							&mut self.data_for_playing.player,
							&mut self.data_for_playing.beams
						);
					});

					self.data_for_playing.objects.retain(|obj| {
						obj.pos_x() > -500.0
					});

					self.data_for_playing.objects.iter_mut().for_each(|obj| {
						obj.update(
							&mut self.data_for_playing.player,
							&self.data_for_playing.beams
						);
					});

					self.data_for_playing.enemies.retain(|enemy| {
						enemy.pos_x() > -200.0 && enemy.pos_y() > -200.0
					});

					self.data_for_playing.enemies.iter_mut().for_each(|enemy| {
						enemy.update(
							&mut self.data_for_playing.player,
							&self.data_for_playing.beams
						);
					});

					self.data_for_playing.beams.retain(|beam| beam.bounding_box.position.x <= self.data_for_playing.status.width);
					self.data_for_playing.beams.iter_mut().for_each(|beam| {

						beam.update();

					});

					self.data_for_playing.balloons.retain(|b| b.top() > -250);
					self.data_for_playing.balloons.iter_mut().for_each(|balloon| {

						balloon.update(
							&mut self.data_for_playing.player,
							&mut self.data_for_playing.beams
						);
					});

					if self.data_for_playing.status.time_line < TIME_LINE_MINIMUM as f32 {
						self.data_for_playing.generate_next_segment();
					} else {
						self.data_for_playing.status.time_line += velocity_x;
					}

					self.data_for_playing.generate_enemy();

					self.data_for_playing.generate_balloon();

					self.data_for_playing.player.update();

					self.data_for_playing.status.frame += 1;

					self.data_for_playing.status.score += 1;

					if self.data_for_playing.status.frame % 30 == 0 {

						let mut rng = thread_rng();
						let next = rng.gen_range(0..10);

						self.data_for_playing.status.score += next;

						let _receiver = browser::remove_first_child("score-right")
							.expect("game : GameState<Playing> : update() : error draw text");

						let _receiver = browser::insert_adjacent_html(
							"score-right",
							format!("{:012}", self.data_for_playing.status.score
							)
						).expect("game : GameState<Playing> : update() : error draw text");
					}

					if self.data_for_playing.status.frame % 10 == 0 {

						match self.data_for_playing.player.state_machine {
							PlayerStateMachine::Shielding(_) => {},
							_ => self.data_for_playing.status.special_energy += 1
						}

						if self.data_for_playing.status.special_energy == 100 {

							self.data_for_playing.play_energy_max_sound();
						}
					}

					if self.data_for_playing.status.special_energy > 100 {
						self.data_for_playing.status.special_energy = 100;

					}

					if self.data_for_playing.status.frame % 10 == 0 {

						let _receiver = browser::remove_first_child("special-energy-right")
							.expect("game : GameState<Playing> : update() : error draw text");

						let _receiver = browser::insert_adjacent_html(
							"special-energy-right",
							format!("<div id='special-energy-mater' style='width: {}%'></div>", self.data_for_playing.status.special_energy)
						);

						match self.data_for_playing.status.special_energy {
							0..20 => browser::set_class_name_by_id("special-energy-mater", "min")
								.expect("game : GameState<Playing> : error draw text"),
							20..50 => browser::set_class_name_by_id("special-energy-mater", "alert")
								.expect("game : GameState<Playing> : error draw text"),
							50..100 => browser::set_class_name_by_id("special-energy-mater", "mid")
								.expect("game : GameState<Playing> : error draw text"),
							100 => browser::set_class_name_by_id("special-energy-mater", "max")
								.expect("game : GameState<Playing> : error draw text"),
							_ =>  browser::set_class_name_by_id("special-energy-mater", "min")
								.expect("game : GameState<Playing> : error draw text"),
						};
					}

					self.data_for_playing.player.get_add_points().iter().for_each(|point| {

						self.data_for_playing.status.score += point;

						if self.data_for_playing.status.score <= 0 {
							self.data_for_playing.status.score = 0
						}

					});

					self.data_for_playing.player.remove_add_points();

					if self.data_for_playing.status.frame % 600 == 0 {
						self.data_for_playing.player.add_moving_speed(0.05);
					}

					PlayingEndState::Continue(self)
				}

			}
		}
	}

	fn do_special_move(mut self) -> GameState<SpecialMove> {

		let mut rng = thread_rng();
		let next_sp = rng.gen_range(0..8);

		match next_sp {
			0 => {
				let mut def_rng = thread_rng();
				let next_def = def_rng.gen_range(0..5);
				let is_def = if next_def != 4 { true } else { false };

				self.data_for_playing.special = Some(
					Box::new(
						SpecialKazama::new(
							self.data_for_playing.special_images.kazama.clone(),
							self.data_for_playing.sprite_sheets.kazama_sheet.sheet.clone(),
							self.data_for_playing.sprite_sheets.kazama_sheet.image.clone(),
							Point {
								x: self.data_for_playing.status.width / 2.0,
								y: self.data_for_playing.status.height / 2.0,
							},
							self.data_for_playing.audio.clone(),
							KazamaSounds::new(
								self.data_for_playing.sounds.special_koto.clone(),
								self.data_for_playing.sounds.special_sword_0.clone(),
								self.data_for_playing.sounds.special_sword_1.clone()
							),
							is_def
						)
					)
				);
			}
			_ => {
				self.data_for_playing.special = Some(
					Box::new(
						SpecialOkanyan::new(
							self.data_for_playing.special_images.okanyan.clone(),
							Point {
								x: self.data_for_playing.status.width / 2.0,
								y: self.data_for_playing.status.height / 2.0
							},
							self.data_for_playing.audio.clone(),
							OkanyanSounds::new(
								self.data_for_playing.sounds.special.clone(),
								self.data_for_playing.sounds.special_energy.clone()
							)
						)
					)
				);
			}
		}

		GameState {
			_state: SpecialMove,
			data_for_playing: self.data_for_playing,
		}
	}

	fn end_game(mut self) -> GameState<GameOver> {

		if let Some(audio) = self.data_for_playing.bgm.current_audio.take() {
			audio.suspend_sound();
			self.data_for_playing.bgm.current_audio = Some(audio);
		}

		let _receiver = browser::set_class_name_by_id("game-over-area", "")
			.expect("game : GameState<Playing> : end_game() : error draw text");

		GameState {
			_state: GameOver,
			data_for_playing: self.data_for_playing
		}
	}

	pub fn draw(&self, renderer: &Renderer) {
		self.data_for_playing.draw_playing_bg(renderer);
		self.data_for_playing.draw_obstacles_bg(renderer);
		self.data_for_playing.draw_player(renderer);
		self.data_for_playing.draw_obstacles(renderer);
		self.data_for_playing.draw_enemies(renderer);
		self.data_for_playing.draw_balloons(renderer);
		self.data_for_playing.draw_beam(renderer);
		self.data_for_playing.draw_objects(renderer);
	}
}

impl GameState<SpecialMove> {

	// 必殺技

	fn update(mut self) -> SpecialMoveEndState {

		if let Some(sp) = self.data_for_playing.special.as_mut() {

			if sp.is_end() {

				self.data_for_playing.player.state_machine.set_velocity_y(0.0);
				self.defeat_enemies();
				self.defeat_balloons();
				self.defeat_wood_boxes();
				self.defeat_objects();
				SpecialMoveEndState::Complete(self.replay())

			} else {
				sp.update();
				SpecialMoveEndState::Continue(self)
			}
		} else {
			panic!()
		}
	}
	
	fn replay(mut self) -> GameState<Playing> {

		self.data_for_playing.special = None;
		self.data_for_playing.status.special_energy = 0;

		let _receiver = browser::set_class_name_by_id("special-energy-mater", "")
			.expect("game : GameState<SpecialMove> : replay() : error draw text");

		GameState {
			_state: Playing,
			data_for_playing: self.data_for_playing,
		}
	}

	pub fn draw(&self, renderer: &Renderer) {

		self.data_for_playing.draw_playing_bg(renderer);
		self.data_for_playing.draw_obstacles_bg(renderer);
		self.data_for_playing.draw_player(renderer);
		self.data_for_playing.draw_obstacles(renderer);
		self.data_for_playing.draw_enemies(renderer);
		self.data_for_playing.draw_balloons(renderer);
		self.data_for_playing.draw_beam(renderer);
		self.data_for_playing.draw_objects(renderer);

		self.draw_special_move(renderer);
	}

	pub fn draw_special_move(&self, renderer: &Renderer) {

		if let Some(sp) = self.data_for_playing.special.as_ref() {
			sp.draw(renderer);
		}

	}

	fn defeat_enemies(&mut self) {
		self.data_for_playing.enemies
			.iter_mut()
			.for_each(|e|
				e.be_defeated()
			);
	}

	fn defeat_balloons(&mut self) {
		self.data_for_playing.balloons
			.iter_mut()
			.for_each(|balloon| {
				balloon.be_defeated()
			});
	}

	fn defeat_wood_boxes(&mut self) {
		self.data_for_playing.obstacles
			.iter_mut()
			.filter(|obs| match obs.get_kinds() {
				ObstacleKinds::WoodBox => true,
				_ => false,
			})
			.for_each(|obs| {
				obs.be_defeated()
			})
	}

	fn defeat_objects(&mut self) {
		self.data_for_playing.objects
			.iter_mut()
			.for_each(|o| {
				o.be_defeated()
			})
	}
}

impl GameState<GameOver> {

	fn update(self, key_state: &mut KeyState) -> GameOverEndState {

		if key_state.is_pressed("Space") && !key_state.flag.space {
			key_state.flag.space = true;
			GameOverEndState::Complete(self.go_back_to_title())
		} else {
			GameOverEndState::Continue(self)
		}
	}

	fn go_back_to_title(mut self) -> GameState<Title> {

		let _receiver = browser::set_class_name_by_id("right-top-area", "hidden")
			.expect("game : GameState<Playing> : update() : error draw text");

		let _receiver = browser::set_class_name_by_id("game-over-area", "hidden")
			.expect("game : GameState<GameOver> : go_back_to_title() : error draw text");

		let _receiver = browser::set_class_name_by_id("title-area", "")
			.expect("game : FlyingOkanyan : initialize() : error draw text");

		self.data_for_playing.player.go_back_to_title();

		GameState {
			_state: Title,
			data_for_playing: self.data_for_playing.reset(MOVING_SPEED)
		}
	}

	pub fn draw(&self, renderer: &Renderer) {
		self.data_for_playing.draw_playing_bg(renderer);
		self.data_for_playing.draw_obstacles_bg(renderer);
		self.data_for_playing.draw_player(renderer);
		self.data_for_playing.draw_obstacles(renderer);
		self.data_for_playing.draw_enemies(renderer);
		self.data_for_playing.draw_balloons(renderer);
		self.data_for_playing.draw_beam(renderer);
		self.data_for_playing.draw_objects(renderer);

		draw_modal_background("rgba(0, 0, 0, 0.8)");
	}
}

impl From<GameState<Title>> for GameStateMachine {
	fn from(state: GameState<Title>) -> Self {
		GameStateMachine::Title(state)
	}
}
impl From<GameState<Playing>> for GameStateMachine {
	fn from(state: GameState<Playing>) -> Self {
		GameStateMachine::Playing(state)
	}
}
impl From<GameState<SpecialMove>> for GameStateMachine {
	fn from(state: GameState<SpecialMove>) -> Self {
		GameStateMachine::SpecialMove(state)
	}
}
impl From<GameState<GameOver>> for GameStateMachine {
	fn from(state: GameState<GameOver>) -> Self {
		GameStateMachine::GameOver(state)
	}
}

#[derive(Debug)]
enum TitleEndState {
	Complete(GameState<Playing>),
	Continue(GameState<Title>)
}
impl From<TitleEndState> for GameStateMachine {
	fn from(state: TitleEndState) -> Self {
		match state {
			TitleEndState::Complete(state) => state.into(),
			TitleEndState::Continue(state) => state.into()
		}
	}
}

#[derive(Debug)]
enum PlayingEndState {
	Complete(GameState<GameOver>),
	Special(GameState<SpecialMove>),
	Continue(GameState<Playing>)
}
impl From<PlayingEndState> for GameStateMachine {
	fn from(state: PlayingEndState) -> Self {
		match state {
			PlayingEndState::Complete(state) => state.into(),
			PlayingEndState::Special(state) => state.into(),
			PlayingEndState::Continue(state) => state.into()
		}
	}
}

#[derive(Debug)]
enum SpecialMoveEndState {
	Complete(GameState<Playing>),
	Continue(GameState<SpecialMove>)
}
impl From<SpecialMoveEndState> for GameStateMachine {
	fn from(state: SpecialMoveEndState) -> Self {
		match state {
			SpecialMoveEndState::Complete(state) => state.into(),
			SpecialMoveEndState::Continue(state) => state.into()
		}
	}
}

#[derive(Debug)]
enum GameOverEndState {
	Complete(GameState<Title>),
	Continue(GameState<GameOver>)
}
impl From<GameOverEndState> for GameStateMachine {
	fn from(state: GameOverEndState) -> Self {
		match state {
			GameOverEndState::Complete(state) => state.into(),
			GameOverEndState::Continue(state) => state.into()
		}
	}
}
