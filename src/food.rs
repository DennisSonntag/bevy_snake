#![allow(dead_code)]

use bevy::prelude::*;

use crate::{pos_to_coord, rand_int_range, Food, COUNT, FOOD_COLOR, TILE_SIZE};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(spawn_food_system);
	}
}

fn spawn_food_system(mut commands: Commands) {
	let (food_pos_x, food_pos_y) = (
		pos_to_coord(rand_int_range(0, COUNT - 1)),
		pos_to_coord(rand_int_range(0, COUNT - 1)),
	);

	commands.spawn((
		SpriteBundle {
			transform: Transform {
				translation: Vec3::new(food_pos_x, food_pos_y, 0.0),
				scale: TILE_SIZE,
				..default()
			},
			sprite: Sprite {
				color: FOOD_COLOR,
				..default()
			},
			..default()
		},
		Food,
	));
}
