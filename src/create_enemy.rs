use std::f64::consts::PI;
use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::enemy::Enemy;
use crate::enemy_beam::{create_enemy_beam};
use crate::enemy_energy::create_enemy_energy;
use crate::enemy_fubura::create_enemy_fubura;
use crate::enemy_hato::create_enemy_hato;
use crate::enemy_kedama::{create_enemy_kedama, create_enemy_kedama_r};
use crate::enemy_okayu::create_enemy_okayu;
use crate::enemy_pokobe::create_enemy_pokobe;
use crate::enemy_rocket::{create_rocket_0, create_rocket_1};
use crate::engine::{Audio, Point, Sound, SpriteSheet};

pub fn rocket_0(
	sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	velocity_x: f32,
	width: f32,
	height: f32
) -> Vec<Box<dyn Enemy>> {
	vec!(
		Box::new(
			create_rocket_0(
				sheet,
				audio,
				sound,
				velocity_x,
				width,
				height
			)
		)
	)
}

pub fn rocket_1(
	sheet: Rc<SpriteSheet>, 
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	velocity_x: f32,
	width: f32,
	height: f32
) -> Vec<Box<dyn Enemy>> {
	vec!(
		Box::new(
			create_rocket_1(
				sheet,
				audio,
				sound,
				velocity_x,
				width,
				height
			)
		)
	)
}

pub fn okayu_beam (
	sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1], 
	sound: [Rc<Sound>; 1],
	width: f32,
	height: f32
) -> Vec<Box<dyn Enemy>> {
	let rng_y = height as i16 / 100 - 2;
	let mut rng = thread_rng();
	let position_y = rng.gen_range(0..rng_y);

	vec!(
		Box::new(
			create_enemy_okayu(sheet.clone(), position_y * 100, width)
		),
		Box::new(
			create_enemy_beam(
				sheet,
				position_y * 100,
				audio,
				sound,
				width
			)
		)
	)
}

pub fn okayu_energy (
	sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
	width: f32,
	height: f32
) -> Vec<Box<dyn Enemy>> {

	let velocity_x= -15.0;
	let theta_15: f64 = 1.0 / 20.0 * PI;
	let theta_30: f64 = 1.0 / 10.0 * PI;

	vec!(
		Box::new(
			create_enemy_okayu(sheet.clone(), height as i16 - 200, width)
		),
		Box::new(
			create_enemy_energy(
				sheet.clone(),
				360,
				audio.clone(),
				sound.clone(),
				Point {x: velocity_x, y: 0.0},
				0,
				width
			)
		),
		Box::new(
			create_enemy_energy(
				sheet.clone(),
				360,
				audio.clone(),
				sound.clone(),
				Point {
					x: (theta_15.cos() as f32) * velocity_x,
					y: (theta_15.sin() as f32) * velocity_x
				},
				-10,
				width
			)
		),
		Box::new(
			create_enemy_energy(
				sheet,
				360,
				audio,
				sound,
				Point {
					x: (theta_30.cos() as f32) * velocity_x,
					y: (theta_30.sin() as f32) * velocity_x
				},
				-20,
				width
			)
		),
	)
}

pub fn fubura(
	sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 3],
	width: f32,
	height: f32
) -> Vec<Box<dyn Enemy>> {

	vec!(
		Box::new(
			create_enemy_fubura(sheet, audio, sound, width, height)
		),
	)
}

pub fn kedama(sheet: Rc<SpriteSheet>, height: f32) -> Vec<Box<dyn Enemy>> {
	
	vec!(
		Box::new(
			create_enemy_kedama(sheet, height)
		)
	)
}


pub fn kedama_r(sheet: Rc<SpriteSheet>, height: f32) -> Vec<Box<dyn Enemy>> {

	vec!(
		Box::new(
			create_enemy_kedama_r(sheet, height)
		)
	)
}

pub fn hato(
	sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 2],
	width: f32,
	height: f32
) -> Vec<Box<dyn Enemy>> {

	vec!(
		Box::new(
			create_enemy_hato(sheet, audio, sound, width, height)
		),
	)
}

pub fn pokobe(sheet: Rc<SpriteSheet>, width: f32, height: f32) -> Vec<Box<dyn Enemy>> {

	vec!(
		Box::new(
			create_enemy_pokobe(sheet, width, height)
		)
	)
}