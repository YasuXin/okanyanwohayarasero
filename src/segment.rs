use std::rc::Rc;
use rand::{thread_rng, Rng};
use crate::engine::{Audio, Point, Sound, SpriteSheet};
use crate::obstacle::Obstacle;
use crate::create_platform::{create_bonus, create_bonus_pillar_0, create_bonus_platform_0, create_bonus_platform_1, create_bonus_platform_2, create_bonus_platform_3, create_bonus_platform_4, create_bonus_platform_5, create_bonus_platform_6, create_checker, create_moving_platform_0_down_0, create_moving_platform_0_down_1, create_moving_platform_0_up_0, create_moving_platform_0_up_1, create_moving_platform_1_0, create_moving_platform_1_1, create_moving_platform_2_0, create_moving_platform_2_1, create_moving_platform_3_0, create_moving_platform_3_1, create_moving_platform_4_0, create_moving_platform_4_1, create_moving_platform_5_0, create_moving_platform_5_1, create_platform_0, create_platform_1, create_platform_2_0, create_platform_2_1, create_platform_2_2, create_platform_4_0, create_platform_4_1, create_platform_4_2, create_platform_5_0, create_platform_6_0, create_platform_6_2, create_platform_7_0, create_platform_arm_0, create_platform_pillar_0, create_platform_pillar_1, create_platform_pillar_2, create_platform_start_0, create_platform_start_1};

//-------------------------------------------
// Obstacleトレイトを実装したオブジェクトを作成する
//-------------------------------------------

pub fn start_platform(
	sprite_sheet: Rc<SpriteSheet>,
	_offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 0.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	vec!(
		Box::new(create_platform_start_0(
			sprite_sheet.clone(),
			Point {
				x: X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_start_1(
			sprite_sheet.clone(),
			Point {
				x: X_0,
				y: height - 50.0,
			}
		))
	)
}

pub fn start_platform_bg(
	sprite_sheet: Rc<SpriteSheet>,
	_offset_x: f32
) -> Vec<Box<dyn Obstacle>> {

	let mut platform: Vec<Box<dyn Obstacle>> = Vec::new();

	for i in 0..12 {
		platform.push(
			Box::new(create_checker(
				sprite_sheet.clone(),
				Point {
					x: 0.0,
					y: 0.0 + i as f32 * 50.0
				}
			))
		)
	}

	platform
}

pub fn platform_0(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = -200.0; // y軸

	const X_1: f32 = 800.0; // x軸
	const Y_1: f32 = 250.0; // y軸

	vec![
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet,
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		))
	]
}

pub fn platform_1(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸
	const Y_0: f32 = 0.0; //  y軸

	let mut rng = thread_rng();
	let size = rng.gen_range(0..=4);

	let mut result: Vec<Box<dyn Obstacle>> = Vec::new();

	for i in 0..=size {
		result.push(
			Box::new(create_platform_2_0(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: Y_0 + 50.0 * ( i - 1 ) as f32,
				}
			))
		)
	}

	result.push(
		Box::new(create_platform_2_1(
			sprite_sheet,
			Point {
				x: offset_x + X_0,
				y: Y_0 + 50.0 * size as f32,
			}
		)),
	);

	result
}

pub fn platform_2(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸

	let mut rng = thread_rng();
	let size = rng.gen_range(0..=4);

	let y: f32 = height - ( size - 1 ) as f32 * 50.0;

	let mut platforms: Vec<Box<dyn Obstacle>> = Vec::new();

	for i in 0..size {
		platforms.push(
			Box::new(create_platform_2_0(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: y + 50.0 * ( i + 1 ) as f32,
				}
			))
		)
	}

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_2_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y,
			}
		)),
	];

	result.append(&mut platforms);

	result
}

pub fn platform_3(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=8);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(0..=6);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=4);

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_4_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			},
			x_1,
			x_2
		)),
		];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: Y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_4_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1
		))
	);

	result
}

pub fn platform_4(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=8);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(0..=6);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=4);

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_5_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			},
			x_1,
			x_2
		)),
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
					y: Y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_4_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
				y: Y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1
		))
	);

	result
}

pub fn platform_5(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=8);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(0..=6);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=4);

	let y_0 = height - ( y + 2 ) as f32 * 50.0;

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_6_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0,
			},
			x_1
		))
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_6_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1,
			x_2
		)),
	);

	result
}

pub fn platform_6(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=8);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(0..=6);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=4);

	let y_0 = height - ( y + 2 ) as f32 * 50.0;

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_6_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
				y: y_0,
			},
			x_1
		))
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
					y: y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_7_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1,
			x_2
		))
	);

	result
}

pub fn platform_7(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = 200.0; // y軸

	const X_1: f32 = 700.0; // x軸
	const Y_1_0: f32 = -200.0; // y軸
	const Y_1_1: f32 = 500.0; // y軸

	const X_2: f32 = 1000.0; // x軸
	const Y_2_0: f32 = -400.0; // y軸
	const Y_2_1: f32 = 300.0; // y軸

	const X_3: f32 = 1300.0; // x軸
	const Y_3_0: f32 = -250.0; // y軸
	const Y_3_1: f32 = 450.0; // y軸

	const X_4: f32 = 1600.0; // x軸
	const Y_4: f32 = -100.0; // y軸

	vec![
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1_1,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2_1,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3_1,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_4,
				y: Y_4,
			}
		)),
	]
}

pub fn platform_8(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸

	let mut rng = thread_rng();
	let y = rng.gen_range(100..=400) as f32;

	vec![
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y - 500.0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet,
			Point {
				x: offset_x + X_0,
				y: y + 100.0,
			}
		))
	]
}

pub fn platform_10(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = -250.0; // y軸

	const X_1: f32 = 800.0; // x軸
	const Y_1: f32 = 200.0; // y軸

	vec![
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet,
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		))
	]
}

pub fn platform_11(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=10);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(0..=4);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=6);

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_4_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			},
			x_1,
			x_2
		)),
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: Y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_4_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1
		))
	);

	result.push(
		Box::new(create_moving_platform_2_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0 + ( x_1 + 3 ) as f32 * 50.0,
				y: height + y as f32 * 50.0
			}
		))
	);

	for i in 0..6 {
		result.push(
			Box::new(create_moving_platform_2_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0 + ( x_1 + 3 ) as f32 * 50.0,
					y: height + y as f32 * 50.0 + ( i + 1 ) as f32 * 50.0,
				}
			))
		)
	}

	result
}

pub fn platform_12(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=10);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(8..=10);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=6);

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_5_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			},
			x_1,
			x_2
		)),
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
					y: Y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_4_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
				y: Y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1
		))
	);

	result.push(
		Box::new(create_moving_platform_2_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: height + y as f32 * 50.0
			}
		))
	);

	for i in 0..6 {
		result.push(
			Box::new(create_moving_platform_2_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: height + y as f32 * 50.0 + ( i + 1 ) as f32 * 50.0,
				}
			))
		)
	}

	result
}

pub fn platform_13(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=10);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(0..=10);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=6);

	let y_0 = height - ( y + 2 ) as f32 * 50.0;

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_6_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0,
			},
			x_1
		))
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_6_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1,
			x_2
		)),
	);

	for i in 0..6 {
		result.push(
			Box::new(create_moving_platform_1_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0 + ( x_1 + 3 ) as f32 * 50.0,
					y: ( -y as f32 * 50.0 ) - 350.0  + ( i + 1 ) as f32 * 50.0,
				}
			))
		)
	}

	result.push(
		Box::new(create_moving_platform_1_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0 + ( x_1 + 3 ) as f32 * 50.0,
				y: ( -y as f32 * 50.0 ) - 50.0
			}
		))
	);



	result
}

pub fn platform_14(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸

	let mut rng = thread_rng();
	let x_1 = rng.gen_range(0..=10);

	let mut rng = thread_rng();
	let x_2 = rng.gen_range(8..=10);

	let mut rng = thread_rng();
	let y = rng.gen_range(0..=6);

	let y_0 = height - ( y + 2 ) as f32 * 50.0;

	let mut result: Vec<Box<dyn Obstacle>> = vec![
		Box::new(create_platform_6_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
				y: y_0,
			},
			x_1
		))
	];

	for i in 0..y {
		result.push(
			Box::new(create_platform_4_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0 + ( x_2 + 1 ) as f32 * 50.0,
					y: y_0 + ( i + 1 ) as f32 * 50.0,
				},
				x_1
			))
		)
	}

	result.push(
		Box::new(create_platform_7_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + ( y + 1 ) as f32 * 50.0,
			},
			x_1,
			x_2
		))
	);

	for i in 0..6 {
		result.push(
			Box::new(create_moving_platform_1_1(
				sprite_sheet.clone(),
				Point {
					x: offset_x + X_0,
					y: ( -y as f32 * 50.0 ) - 350.0  + ( i + 1 ) as f32 * 50.0,
				}
			))
		)
	}

	result.push(
		Box::new(create_moving_platform_1_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: ( -y as f32 * 50.0 ) - 50.0
			}
		))
	);

	result
}

pub fn platform_15(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = 200.0; // y軸

	const X_1: f32 = 700.0; // x軸
	const Y_1_0: f32 = -200.0; // y軸
	const Y_1_1: f32 = 450.0; // y軸

	const X_2: f32 = 1000.0; // x軸
	const Y_2_0: f32 = -400.0; // y軸
	const Y_2_1: f32 = 250.0; // y軸

	const X_3: f32 = 1300.0; // x軸
	const Y_3_0: f32 = -250.0; // y軸
	const Y_3_1: f32 = 400.0; // y軸

	const X_4: f32 = 1600.0; // x軸
	const Y_4: f32 = -100.0; // y軸

	vec![
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1_1,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2_1,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3_0,
			}
		)),
		Box::new(create_platform_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3_1,
			}
		)),
		Box::new(create_platform_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_4,
				y: Y_4,
			}
		)),
	]
}

pub fn moving_platform_0(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 200.0; // x軸
	const Y_0_0: f32 = 0.0; // y軸
	let y_0_1: f32 = height - 50.0; // y軸

	vec![
		Box::new(create_moving_platform_0_down_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0_0 - 200.0,
			}
		)),
		Box::new(create_moving_platform_0_down_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0_0 - 150.0,
			}
		)),
		Box::new(create_moving_platform_0_down_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0_0 - 100.0,
			}
		)),
		Box::new(create_moving_platform_0_down_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0_0 - 50.0,
			}
		)),
		Box::new(create_moving_platform_0_down_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0_0,
			}
		)),
		Box::new(create_moving_platform_0_up_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0_1,
			}
		)),
		Box::new(create_moving_platform_0_up_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0_1 + 50.0,
			}
		)),
		Box::new(create_moving_platform_0_up_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0_1 + 100.0,
			}
		)),
		Box::new(create_moving_platform_0_up_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0_1 + 150.0,
			}
		)),
		Box::new(create_moving_platform_0_up_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0_1 + 200.0,
			}
		)),
	]
}

pub fn moving_platform_1(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	vec![
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 400.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 350.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 300.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 250.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 200.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 150.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 100.0,
			}
		)),
		Box::new(create_moving_platform_1_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 50.0,
			}
		)),
		Box::new(create_moving_platform_1_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
	]
}

pub fn moving_platform_2(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	let y_0: f32 = height - 50.0; // y軸

	vec![
		Box::new(create_moving_platform_2_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 50.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 100.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 150.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 200.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 250.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 300.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 350.0,
			}
		)),
		Box::new(create_moving_platform_2_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 400.0,
			}
		)),

	]
}

pub fn moving_platform_3(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = 0.0; // y軸

	vec![
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 400.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 350.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 300.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 250.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 200.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 150.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 100.0,
			}
		)),
		Box::new(create_moving_platform_3_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0 - 50.0,
			}
		)),
		Box::new(create_moving_platform_3_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
	]
}

pub fn moving_platform_4(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32,
	height: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	let y_0: f32 = height - 50.0; // y軸

	vec![
		Box::new(create_moving_platform_4_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 50.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 100.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 150.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 200.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 250.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 300.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 350.0,
			}
		)),
		Box::new(create_moving_platform_4_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y_0 + 400.0,
			}
		)),

	]
}

pub fn moving_platform_5(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸

	let mut rng = thread_rng();
	let y = rng.gen_range(100..=400) as f32;

	vec![
		Box::new(create_moving_platform_5_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: y - 500.0,
			}
		)),
		Box::new(create_moving_platform_5_1(
			sprite_sheet,
			Point {
				x: offset_x + X_0,
				y: y + 100.0,
			}
		))
	]
}


pub fn platform_arms_0(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 300.0; // x軸
	const Y_0: f32 = 600.0; // y軸

	vec![
		Box::new(create_platform_arm_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
	]
}

pub fn platform_arms_pillars_0(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0; // x軸
	const Y_0: f32 = 600.0; // y軸

	const X_1: f32 = 800.0;
	const Y_1: f32 = -650.0;

	const X_2: f32 = 1200.0;
	const Y_2: f32 = 600.0;

	vec![
		Box::new(create_platform_arm_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_pillar_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		)),
		Box::new(create_platform_arm_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2,
			}
		)),
	]
}

pub fn platform_pillars_0(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0;
	const Y_0: f32 = 675.0;

	const X_1: f32 = 800.0;
	const Y_1: f32 = -700.0;

	const X_2: f32 = 1200.0;
	const Y_2: f32 = 650.0;

	const X_3: f32 = 1600.0;
	const Y_3: f32 = -700.0;

	const X_4: f32 = 2000.0;
	const Y_4: f32 = 650.0;

	vec![
		Box::new(create_platform_pillar_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_pillar_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		)),
		Box::new(create_platform_pillar_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2,
			}
		)),
		Box::new(create_platform_pillar_0(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3,
			}
		)),
		Box::new(create_platform_pillar_1(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_4,
				y: Y_4,
			}
		)),
	]
}

pub fn platform_pillars_1(
	sprite_sheet: Rc<SpriteSheet>,
	offset_x: f32
) -> Vec<Box<dyn Obstacle>> {
	const X_0: f32 = 400.0;
	const Y_0: f32 = 275.0;

	const X_1: f32 = 750.0;
	const Y_1: f32 = -600.0;

	const X_2: f32 = 1100.0;
	const Y_2: f32 = 250.0;

	const X_3: f32 = 1450.0;
	const Y_3: f32 = -600.0;

	const X_4: f32 = 1800.0;
	const Y_4: f32 = 225.0;

	vec![
		Box::new(create_platform_pillar_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_0,
				y: Y_0,
			}
		)),
		Box::new(create_platform_pillar_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		)),
		Box::new(create_platform_pillar_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2,
			}
		)),
		Box::new(create_platform_pillar_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3,
			}
		)),
		Box::new(create_platform_pillar_2(
			sprite_sheet.clone(),
			Point {
				x: offset_x + X_4,
				y: Y_4,
			}
		)),
	]
}

pub fn platform_bonus_0(
	gravity: f32,
	offset_x: f32,
	bonus_sprite_sheet: Rc<SpriteSheet>,
	obstacle_sprite_sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
) -> Vec<Box<dyn Obstacle>> {

	const X_1: f32 = 600.0;
	const Y_1: f32 = -400.0;

	const X_2: f32 = 600.0;
	const Y_2: f32 = 200.0;

	const X_3: f32 = 600.0;
	const Y_3: f32 = 300.0;

	vec![
		Box::new(create_bonus_pillar_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		)),
		Box::new(create_bonus(
			gravity,
			bonus_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2,
			},
			audio,
			sound
		)),
		Box::new(create_bonus_pillar_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3,
			}
		)),
	]
}

pub fn platform_bonus_1(
	gravity: f32,
	offset_x: f32,
	bonus_sprite_sheet: Rc<SpriteSheet>,
	obstacle_sprite_sheet: Rc<SpriteSheet>,
	audio: [Rc<Audio>; 1],
	sound: [Rc<Sound>; 1],
) -> Vec<Box<dyn Obstacle>> {
	const X_1: f32 = 300.0;
	const Y_1: f32 = -300.0;

	const X_2: f32 = 600.0;
	const Y_2: f32 = 0.0;

	const X_3: f32 = 1100.0;
	const Y_3: f32 = 300.0;

	vec![
		Box::new(create_bonus_pillar_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_1,
				y: Y_1,
			}
		)),
		Box::new(create_bonus_platform_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2,
			}
		)),
		Box::new(create_bonus_platform_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 50.0,
			}
		)),
		Box::new(create_bonus_platform_1(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 100.0,
			}
		)),
		Box::new(create_bonus(
			gravity,
			bonus_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 150.0,
			},
			audio,
			sound
		)),
		Box::new(create_bonus_platform_2(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 250.0,
			}
		)),
		Box::new(create_bonus_platform_3(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 300.0,
			}
		)),
		Box::new(create_bonus_platform_4(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 350.0,
			}
		)),
		Box::new(create_bonus_platform_5(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 500.0,
			}
		)),
		Box::new(create_bonus_platform_6(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_2,
				y: Y_2 + 550.0,
			}
		)),
		Box::new(create_bonus_platform_2(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3,
			}
		)),
		Box::new(create_bonus_platform_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3 + 50.0,
			}
		)),
		Box::new(create_bonus_platform_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3 + 100.0,
			}
		)),
		Box::new(create_bonus_platform_0(
			obstacle_sprite_sheet.clone(),
			Point {
				x: offset_x + X_3,
				y: Y_3 + 150.0,
			}
		)),
	]
}