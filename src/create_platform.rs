use std::rc::Rc;
use crate::bonus_wood_box::BonusWoodBox;
use crate::engine::{Audio, Point, Rect, Sound, SpriteSheet};
use crate::platform::{Kinds, Platform};

pub fn create_platform_start_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites: Vec<&str> = Vec::new();

	for _i in 0..10 {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_10.png");
	
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 545.0, 45.0),
	);
	
	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_start_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites: Vec<&str> = Vec::new();

	for _i in 0..10 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 545.0, 50.0),
	);
	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_checker(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites: Vec<&str> = Vec::new();
	
	for _i in 0..6 {
		sprites.push("platform_6.png")
	}
	
	sprites.push("checker.png");
	
	let bounding_box = vec!();
	
	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

// 上ポジション
// 下ポジションで判定あり
pub fn create_platform_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!("platform_0.png");
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 45.0, 495.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

// 下ポジション
// 上ポジションで判定あり
pub fn create_platform_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!("platform_0.png");
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 45.0, 500.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_2_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_5.png");

	for _in in 0..6 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_2_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let mut sprites = vec!("platform_8.png");

	for _i in 0..6 {
		sprites.push("platform_9.png")
	}
	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 45.0),
	);
	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_2_2(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let mut sprites = vec!("platform_2.png");

	for _i in 0..6 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 390.0, 45.0),
	);
	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_4_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x_1: u8,
	x_2: u8
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_5.png");

	for _i in 0..x_1 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_14.png");

	for _i in 0..x_2 {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x_1 + x_2 + 3 ) as f32 * 50.0 - 10.0, 50.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}
pub fn create_platform_4_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x: u8,
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_5.png");

	for _i in 0..x {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x + 2 ) as f32 * 50.0 - 10.0, 50.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}
pub fn create_platform_4_2(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x: u8
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_8.png");

	for _i in 0..x {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x + 2 ) as f32 * 50.0 - 10.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_5_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x_1: u8,
	x_2: u8
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_8.png");

	for _i in 0..x_2 {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_13.png");

	for _i in 0..x_1 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x_1 + x_2 + 3 ) as f32 * 50.0 - 10.0, 50.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_6_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x_1: u8,
	x_2: u8
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_5.png");

	for _i in 0..x_1 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_12.png");

	for _i in 0..x_2 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x_1 + x_2 + 3 ) as f32 * 50.0 - 10.0, 50.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_6_2(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x: u8
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_2.png");

	for _i in 0..x {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x + 2 ) as f32 * 50.0 - 10.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_platform_7_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	x_1: u8,
	x_2: u8
) -> Platform {

	let mut sprites: Vec<&str> = vec!("platform_2.png");

	for _i in 0..x_2 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_11.png");

	for _i in 0..x_1 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, ( x_1 + x_2 + 3 ) as f32 * 50.0 - 10.0, 50.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_moving_platform_0_up_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_2.png",
	);

	for _i in 0..28 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 1490.0, 45.0),
	);
	Platform::new(
		Kinds::MovingUp2,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		2.0
	)
}

pub fn create_moving_platform_0_up_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_5.png",
	);

	for _i in 0..28 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 1490.0, 50.0),
	);
	Platform::new(
		Kinds::MovingUp2,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		2.0
	)
}

pub fn create_moving_platform_0_down_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_8.png",
	);

	for _i in 0..28 {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 1490.0, 45.0),
	);
	Platform::new(
		Kinds::MovingDown2,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		2.0
	)
}

pub fn create_moving_platform_0_down_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_5.png",
	);

	for _i in 0..28 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 1490.0, 50.0),
	);
	Platform::new(
		Kinds::MovingDown2,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		2.0
	)
}

pub fn create_moving_platform_1_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_8.png"
	);
	for _i in 0..6 {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingDown1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		16.0
	)
}

pub fn create_moving_platform_1_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_5.png",
	);

	for _i in 0..6 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingDown1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		16.0
	)
}

pub fn create_moving_platform_2_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_2.png",
	);

	for _i in 0..6 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingUp1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		16.0
	)
}

pub fn create_moving_platform_2_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!(
		"platform_5.png"
	);

	for _i in 0..6 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingUp1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		16.0
	)
}

pub fn create_moving_platform_3_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_8.png");

	for _i in 0..6 {
		sprites.push("platform_9.png")
	}

	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingDown3,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		12.0
	)
}

pub fn create_moving_platform_3_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_5.png");

	for _i in 0..6 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingDown3,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		12.0
	)
}

pub fn create_moving_platform_4_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_2.png");

	for _i in 0..6 {
		sprites.push("platform_3.png")
	}

	sprites.push("platform_4.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingUp3,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		12.0
	)
}

// 上ポジション
// 下ポジションで判定あり
pub fn create_moving_platform_5_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!("platform_0.png");
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 45.0, 495.0),
	);

	Platform::new(
		Kinds::MovingUpDown1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		5.0
	)
}

// 下ポジション
// 上ポジションで判定あり
pub fn  create_moving_platform_5_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!("platform_0.png");
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 45.0, 500.0),
	);

	Platform::new(
		Kinds::MovingUpDown1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		5.0
	)
}


pub fn create_moving_platform_4_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_5.png");

	for _in in 0..6 {
		sprites.push("platform_6.png")
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 50.0),
	);
	Platform::new(
		Kinds::MovingUp3,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		12.0
	)
}

pub fn create_platform_arm_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!(
		"platform_16.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(25.0, 65.0, 40.0, 535.0),
	);

	Platform::new(
		Kinds::MovingUp1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		24.0
	)
}

pub fn create_platform_pillar_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!(
		"platform_15.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 90.0, 590.0),
	);

	Platform::new(
		Kinds::MovingDown1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		24.0
	)
}

pub fn create_platform_pillar_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {

	let sprites = vec!(
		"platform_15.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 90.0, 590.0),
	);

	Platform::new(
		Kinds::MovingUp1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		24.0
	)
}

pub fn create_platform_pillar_2(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let sprites = vec!(
		"platform_15.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 90.0, 590.0),
	);

	Platform::new(
		Kinds::MovingUpDown1,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		24.0
	)
}

pub fn create_bonus(
	gravity: f32,
	sprite_sheet: Rc<SpriteSheet>,
	position: Point,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
) -> BonusWoodBox {
	
	BonusWoodBox::new(
		gravity, sprite_sheet, position, audio, sound
	)
}
pub fn create_bonus_pillar_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let sprites = vec!(
		"platform_15.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 90.0, 590.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_bonus_platform_0(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let sprites = vec!(
		"platform_5.png",
		"platform_7.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 90.0, 50.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}
pub fn create_bonus_platform_1(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let sprites = vec!(
		"platform_8.png",
		"platform_10.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 90.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_bonus_platform_2(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let sprites = vec!(
		"platform_2.png",
		"platform_4.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 90.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}

pub fn create_bonus_platform_3(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let sprites = vec!(
		"platform_5.png",
		"platform_12.png",
		"platform_3.png",
		"platform_3.png",
		"platform_3.png",
		"platform_3.png",
		"platform_3.png",
		"platform_4.png",
	);
	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 390.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}
pub fn create_bonus_platform_4(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_8.png");

	for _i in 0..6 {
		sprites.push("platform_9.png");
	}

	sprites.push("platform_10.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 0.0, 390.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}
pub fn create_bonus_platform_5(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_2.png");

	for _i in 0..9 {
		sprites.push("platform_3.png");
	}

	sprites.push("platform_11.png");
	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 590.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}
pub fn create_bonus_platform_6(
	sprite_sheet: Rc<SpriteSheet>,
	position: Point
) -> Platform {
	let mut sprites = vec!("platform_5.png");

	for _i in 0..10 {
		sprites.push("platform_6.png");
	}

	sprites.push("platform_7.png");

	let bounding_box = vec!(
		Rect::new_from_x_y(5.0, 5.0, 590.0, 45.0),
	);

	Platform::new(
		Kinds::Normal,
		sprite_sheet,
		position,
		&sprites,
		&bounding_box,
		0.0
	)
}