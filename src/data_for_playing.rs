use std::rc::Rc;
use rand::{thread_rng, Rng};
use web_sys::HtmlImageElement;
use crate::beam::Beam;
use crate::bonus_balloon::Balloon;
use crate::create_enemy::{fubura, hato, kedama, kedama_r, okayu_beam, okayu_energy, pokobe, rocket_0, rocket_1};
use crate::enemy::{Enemy, Kinds};
use crate::engine::{Audio, Image, Point, Renderer, Sound, SpriteSheet};
use crate::obstacle::{right_most, Obstacle};
use crate::player::Player;
use crate::segment::{moving_platform_0, moving_platform_1, moving_platform_2, moving_platform_3, moving_platform_4, moving_platform_5, platform_0, platform_1, platform_10, platform_11, platform_12, platform_13, platform_14, platform_15, platform_2, platform_3, platform_4, platform_5, platform_6, platform_7, platform_8, platform_arms_0, platform_arms_pillars_0, platform_bonus_0, platform_bonus_1, platform_pillars_0, platform_pillars_1, start_platform, start_platform_bg};
use crate::special::Special;

/*#[derive(Debug)]
pub struct BGMs {
	pub current_audio: Option<Audio>,
	pub _current_sound: Option<Sound>,
	pub playing_audio: Audio,
	pub _playing_sound: Sound,
}*/
#[derive(Debug)]
pub struct GameSounds {
	pub decision: Rc<Sound>,
	pub enemy_beam: Rc<Sound>,
	pub enemy_energy: Rc<Sound>,
	pub enemy_defeated: Rc<Sound>,
	pub fubura_thunder: Rc<Sound>,
	pub fubura_wulf: Rc<Sound>,
	pub fubura_dogon: Rc<Sound>,
	pub hato_eye: Rc<Sound>,
	pub special: Rc<Sound>,
	pub special_energy: Rc<Sound>,
	pub special_koto: Rc<Sound>,
	pub special_sword_0: Rc<Sound>,
	pub special_sword_1: Rc<Sound>,
	pub bonus: Rc<Sound>,
	pub energy_max: Rc<Sound>,
}

#[derive(Debug)]
pub struct Backgrounds {
	pub title: [Image; 1],
	pub playing_0: [Image; 1], // 一番遠くの画像　移動しない
	pub playing_1: [Image; 2], // 遠くのビルの画像 低速で移動する
	pub playing_2: [Image; 2], // 一番前のビルの画像　プレーヤーと同じ速度で移動する
}

#[derive(Debug)]
pub struct SpecialImages {
	pub okanyan: [Image; 3],
	pub kazama: [Image; 5]
}

#[derive(Debug)]
pub struct Status {
	pub width: f32,
	pub height: f32,
	pub gravity: f32,
	pub time_line: f32, //
	pub frame: i32,
	pub score: i32,
	pub special_energy: u8,
}

#[derive(Debug)]
pub struct SpriteSheets {
	pub kazama_sheet: Rc<SpriteSheet>,
	pub obstacle_sheet: Rc<SpriteSheet>, // オブジェクトのスプライトシートの情報
	pub enemies_sheet: Rc<SpriteSheet>, // 敵キャラクターのスプライトシート
	pub fubura_sheet: Rc<SpriteSheet>,
	pub kedama_sheet: Rc<SpriteSheet>,
	pub hato_sheet: Rc<SpriteSheet>,
	pub bonus_sheet: Rc<SpriteSheet>,
}

#[derive(Debug)]
pub struct DataForPlaying {
	pub status: Status,
	pub player: Player, // 操作キャラの情報
	pub backgrounds: Backgrounds, // 背景の情報
	pub special_images: SpecialImages,
	pub beam_image: HtmlImageElement,
	pub sprite_sheets: SpriteSheets,
	pub special: Option<Box<dyn Special>>,
	pub obstacles: Vec<Box<dyn Obstacle>>, //
	pub obstacles_bg: Vec<Box<dyn Obstacle>>,
	pub enemies: Vec<Box<dyn Enemy>>,
	pub objects: Vec<Box<dyn Enemy>>,
	pub balloons: Vec<Balloon>,
	pub beams: Vec<Beam>,
	pub audio: Rc<Audio>,
	pub sounds: GameSounds,
	//pub bgm: BGMs,
}

impl DataForPlaying {

	pub fn draw_title_bg(&self, renderer: &Renderer) {
		self.backgrounds.title.iter().for_each(|bg| {
			bg.draw_bg(renderer);
		});
	}

	pub fn draw_playing_bg(&self, renderer: &Renderer) {
		self.backgrounds.playing_0.iter().for_each(|bg| {
			bg.draw_bg(renderer);
		});
		self.backgrounds.playing_1.iter().for_each(|bg| {
			bg.draw_bg(renderer);
		});
		self.backgrounds.playing_2.iter().for_each(|bg| {
			bg.draw_bg(renderer);
		});
	}

	pub fn draw_player(&self, renderer: &Renderer) {
		self.player.draw(renderer);
	}

	pub fn draw_beam(&self, renderer: &Renderer) {

		let image = self.beam_image.clone();

		self.beams.iter().for_each(|beam| {

			beam.draw(&image, renderer);

		});
	}

	pub fn draw_obstacles_bg(&self, renderer: &Renderer) {
		self.obstacles_bg.iter().for_each(|obstacle| {
			obstacle.draw(renderer);
		});
	}

	pub fn draw_obstacles(&self, renderer: &Renderer) {
		self.obstacles.iter().for_each(|obstacle| {
			obstacle.draw(renderer);
		});
	}

	pub fn draw_objects(&self, renderer: &Renderer) {
		self.objects.iter().for_each(|obj| {
			obj.draw(renderer);
		})
	}

	pub fn draw_enemies(&self, renderer: &Renderer) {
		self.enemies.iter().for_each(|enemy| {
			enemy.draw(renderer);
		})
	}

	pub fn draw_balloons(&self, renderer: &Renderer) {
		self.balloons.iter().for_each(|balloon| {
			balloon.draw(renderer);
		})
	}

	pub fn velocity_x(&self) -> f32 {
		-self.player.moving_speed()
	}

	pub fn generate_beam (&mut self) {
		let center_x = self.player.center_x();
		let center_y = self.player.center_y();

		self.beams.append(
			&mut vec!(Beam::new(
				Point {
					x: center_x + 20.0,
					y: center_y - 25.0
				}
			))
		);
	}

	pub fn generate_next_segment(&mut self) {
		let mut rng_size = ( self.status.frame / 300 ) + 4;

		if rng_size > 26 {
			rng_size = 26
		}

		let mut rng = thread_rng();
		let next_segment = rng.gen_range(0..=rng_size);

		let mut next_obstacles = match next_segment {
			0 => platform_0(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			1 => platform_1(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			2 => platform_2(Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			3 => platform_3(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			4 => platform_4(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			5 => platform_5(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			6 => platform_6(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			7 => platform_7(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			8 => platform_8(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			9 => platform_arms_0(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			10=> platform_arms_pillars_0(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			11 => platform_pillars_0(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			12 => platform_pillars_1(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			13 => moving_platform_0(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			14 => moving_platform_1(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			15 => moving_platform_2(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			16 => moving_platform_3(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			17 => moving_platform_4(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			18 => platform_bonus_0(
				self.status.gravity,
				self.status.time_line,
				Rc::clone(&self.sprite_sheets.bonus_sheet),
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				[Rc::clone(&self.audio)],
				[Rc::clone(&self.sounds.enemy_defeated)]
			),
			19=> platform_10(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			20 => platform_11(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			21 => platform_12(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			22 => platform_13(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			23 => platform_14(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line,
				self.status.height
			),
			24 => platform_15(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			25 => moving_platform_5(
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				self.status.time_line
			),
			26 => platform_bonus_1(
				self.status.gravity,
				self.status.time_line,
				Rc::clone(&self.sprite_sheets.bonus_sheet),
				Rc::clone(&self.sprite_sheets.obstacle_sheet),
				[Rc::clone(&self.audio)],
				[Rc::clone(&self.sounds.enemy_defeated)]
			),
			_ => vec![]
		};

		self.status.time_line = right_most(&next_obstacles) as f32;
		self.obstacles.append(&mut next_obstacles);
	}

	pub fn generate_enemy(&mut self) {
		if self.status.frame % 100 == 0 {
			let mut range = 500 - self.status.frame / 10;
			if range < 100 {
				range = 100;
			}
			let mut rng = thread_rng();
			let next = rng.gen_range(0..range);

			let mut next_enemy = match next {
				0..30 => rocket_0(
					Rc::clone(&self.sprite_sheets.enemies_sheet),
					[Rc::clone(&self.audio)],
					[Rc::clone(&self.sounds.enemy_defeated)],
					self.player.moving_speed(),
					self.status.width,
					self.status.height,
				),
				30..60 => rocket_1(
					Rc::clone(&self.sprite_sheets.enemies_sheet),
					[Rc::clone(&self.audio)],
					[Rc::clone(&self.sounds.enemy_defeated)],
					self.player.moving_speed(),
					self.status.width,
					self.status.height,
				),
				60..70 => {

					if self.status.frame >= 1800 {

						if !self.enemies.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Okayu => true,
							_ => false,
						}) {
							okayu_beam(
								Rc::clone(&self.sprite_sheets.enemies_sheet),
								[Rc::clone(&self.audio)],
								[Rc::clone(&self.sounds.enemy_beam)],
								self.status.width,
								self.status.height,
							)
						} else {
							vec!()
						}
					} else {
						vec!()
					}
				},
				70..80 => {

					if self.status.frame >= 180 {

						if !self.enemies.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Okayu => true,
							_ => false,
						}) {
							okayu_energy(
								Rc::clone(&self.sprite_sheets.enemies_sheet),
								[Rc::clone(&self.audio)],
								[Rc::clone(&self.sounds.enemy_energy)],
								self.status.width,
								self.status.height,
							)
						} else {
							vec!()
						}
					} else {
						vec!()
					}
				},
				_ => vec!()
			};

			self.enemies.append(&mut next_enemy);

			if self.status.frame % 300 == 0 && self.status.frame >= 300 {
				let mut next_object = match next {
					0..20 => {
						if !self.objects.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Fubura | Kinds::Kedama | Kinds::Hato | Kinds::Pokobe => true,
							_ => false,
						}) {
							fubura(
								Rc::clone(&self.sprite_sheets.fubura_sheet),
								[Rc::clone(&self.audio)],
								[
									Rc::clone(&self.sounds.fubura_thunder),
									Rc::clone(&self.sounds.fubura_wulf),
									Rc::clone(&self.sounds.fubura_dogon)
								],
								self.status.width,
								self.status.height,
							)
						} else {
							vec!()
						}
					},
					20..35 => {
						if !self.objects.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Fubura | Kinds::Kedama | Kinds::Hato | Kinds::Pokobe => true,
							_ => false,
						}) {
							kedama(
								Rc::clone(&self.sprite_sheets.kedama_sheet),
								self.status.height,
							)
						} else {
							vec!()
						}
					},
					35..50 => {
						if !self.objects.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Fubura | Kinds::Kedama | Kinds::Hato | Kinds::Pokobe => true,
							_ => false,
						}) {
							kedama_r(
								Rc::clone(&self.sprite_sheets.kedama_sheet),
								self.status.height,
							)
						} else {
							vec!()
						}
					},
					50..70 => {
						if !self.objects.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Fubura | Kinds::Kedama | Kinds::Hato | Kinds::Pokobe => true,
							_ => false,
						}) {
							hato(
								Rc::clone(&self.sprite_sheets.hato_sheet),
								[
									Rc::clone(&self.audio)
								],
								[
									Rc::clone(&self.sounds.fubura_thunder),
									Rc::clone(&self.sounds.hato_eye)
								],
								self.status.width,
								self.status.height,
							)
						} else {
							vec!()
						}
					},
					70..100 => {
						if !self.objects.iter().any(|e| match e.get_struct_kinds() {
							Kinds::Fubura | Kinds::Kedama | Kinds::Hato | Kinds::Pokobe => true,
							_ => false,
						}) {
							pokobe(
								Rc::clone(&self.sprite_sheets.kedama_sheet),
								self.status.width,
								self.status.height,
							)
						} else {
							vec!()
						}
					},
					_ => vec!()
				};

				self.objects.append(&mut next_object);
			}
		}
	}

	pub fn generate_balloon(&mut self) {

		if self.status.frame % 60 == 0 {
			let mut rng = thread_rng();
			let next_balloon = rng.gen_range(0..10);

			let mut new_balloon = match next_balloon {
				0 => vec!(
					Balloon::new(
						Rc::clone(&self.sprite_sheets.bonus_sheet),
						[Rc::clone(&self.audio)],
						[Rc::clone(&self.sounds.bonus)],
						self.status.width,
						self.status.height,
					)
				),
				_ => vec!()
			};

			self.balloons.append(&mut new_balloon);
		}
	}

	pub fn reset(self, start_velocity_x: f32) -> Self {

		let starting_obstacles = start_platform(
			Rc::clone(&self.sprite_sheets.obstacle_sheet),
			0.0,
			self.status.height
		);

		let starting_checker = start_platform_bg(
			Rc::clone(&self.sprite_sheets.obstacle_sheet),
			0.0
		);

		let time_line = right_most(&starting_obstacles) as f32;

		let new_player = self.player.reset(start_velocity_x);

		DataForPlaying {
			status: Status {
				width: self.status.width,
				height: self.status.height,
				gravity: self.status.gravity,
				time_line,
				frame: 0,
				score: 0,
				special_energy: 0,
			},
			player: new_player,
			backgrounds: self.backgrounds,
			special_images: self.special_images,
			special: None,
			balloons: Vec::new(),
			sprite_sheets: self.sprite_sheets,
			obstacles: starting_obstacles,
			obstacles_bg: starting_checker,
			beam_image: self.beam_image,
			beams: Vec::new(),
			enemies: vec!(),
			objects: vec!(),
			audio: self.audio,
			sounds: self.sounds,
			//bgm: self.bgm,
		}
	}

	pub fn play_decision_sound(&self) {
		if let Err(err) = self.audio.play_sound(&self.sounds.decision) {
			log!("Error playing decision sound: {}", err);
		}
	}
	pub fn play_energy_max_sound(&self) {
		if let Err(err) = self.audio.play_sound(&self.sounds.energy_max) {
			log!("Error playing energy_max sound: {}", err);
		}
	}
}