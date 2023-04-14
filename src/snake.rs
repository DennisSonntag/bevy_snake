#![allow(dead_code)]

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
	coord_to_pos, pos_to_coord, rand_int_range, Food, Player, Point, Snake, TailSegment, COUNT,
	HEIGHT, TAIL_COLOR, TAIL_COLOR_MIN, TAIL_SIZE, TAIL_SIZE_MIN, TILE_COLOR, TILE_SIZE, WIDTH,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(spawn_player_system)
			.add_system(keyboard_input_system)
			.add_system(snake_movment_system.run_if(on_timer(Duration::from_secs_f32(0.08))));
	}
}

fn color_lerp(color1: Color, color2: Color, t: f32) -> Color {
	let r = color1.r() * (1.0 - t) + color2.r() * t;
	let g = color1.g() * (1.0 - t) + color2.g() * t;
	let b = color1.b() * (1.0 - t) + color2.b() * t;
	let a = color1.a() * (1.0 - t) + color2.a() * t;
	Color::rgba(r, g, b, a)
}

fn snake_movment_system(
	mut snake: ResMut<Snake>,
	mut player_query: Query<&mut Transform, (With<Player>, Without<Food>)>,
	mut food_query: Query<&mut Transform, With<Food>>,
	tail_query: Query<Entity, With<TailSegment>>,
	mut commands: Commands,
) {
	let mut player_transform = player_query.single_mut();

	let value_x: f32 = snake.vel.x as f32 * TILE_SIZE.x;
	let value_y: f32 = snake.vel.y as f32 * TILE_SIZE.y;

	player_transform.translation.x += value_x;
	player_transform.translation.y += value_y;

	if player_transform.translation.x <= -(WIDTH / 2.0) {
		player_transform.translation.x = (WIDTH / 2.0) - (TILE_SIZE.x / 2.0);
	} else if player_transform.translation.x >= (WIDTH / 2.0) {
		player_transform.translation.x = -(WIDTH / 2.0) + (TILE_SIZE.x / 2.0);
	}

	if player_transform.translation.y <= -(HEIGHT / 2.0) {
		player_transform.translation.y = (HEIGHT / 2.0) - (TILE_SIZE.y / 2.0);
	} else if player_transform.translation.y >= (HEIGHT / 2.0) {
		player_transform.translation.y = -(HEIGHT / 2.0) + (TILE_SIZE.y / 2.0);
	}

	let (player_x, player_y) = (
		coord_to_pos(player_transform.translation.x),
		coord_to_pos(player_transform.translation.y),
	);

	let mut food_transform = food_query.single_mut();

	let (food_x, food_y) = (
		coord_to_pos(food_transform.translation.x),
		coord_to_pos(food_transform.translation.y),
	);

	if player_x == food_x && player_y == food_y {
		loop {
			let (new_x, new_y) = (rand_int_range(0, COUNT - 1), rand_int_range(0, COUNT - 1));
			if new_x != player_x
				&& new_y != player_y
				&& !snake.tail.contains(&Point { x: new_x, y: new_y })
			{
				food_transform.translation.x = pos_to_coord(new_x);
				food_transform.translation.y = pos_to_coord(new_y);
				snake.len += 1;
				break;
			}
		}
	}

	snake.tail.push(Point {
		x: player_x,
		y: player_y,
	});

	while snake.tail.len() > snake.len as usize {
		snake.tail.remove(0);
	}
	for entity in tail_query.iter() {
		commands.entity(entity).despawn_recursive();
	}

	use bevy::prelude::*;

	let num_segments = snake.tail.len();
	let tail_size_step = (TAIL_SIZE - TAIL_SIZE_MIN) / (num_segments - 1) as f32; // Calculate the size step between segments

	let tail_color_step_r = (TAIL_COLOR.r() - TAIL_COLOR_MIN.r()) / (num_segments - 1) as f32; // Calculate the color step for the red component
	let tail_color_step_g = (TAIL_COLOR.g() - TAIL_COLOR_MIN.g()) / (num_segments - 1) as f32; // Calculate the color step for the green component
	let tail_color_step_b = (TAIL_COLOR.b() - TAIL_COLOR_MIN.b()) / (num_segments - 1) as f32; // Calculate the color step for the blue component

	for (index, point) in snake.tail.iter().enumerate() {
		let (seg_x, seg_y) = (pos_to_coord(point.x), pos_to_coord(point.y));
		let tail_scale = TAIL_SIZE - tail_size_step * (num_segments - index - 1) as f32; // Calculate the scale for this segment in reverse order

		let color_factor = index as f32 / (num_segments - 1) as f32; // Calculate the color factor based on the index of the segment
		let tail_color = Color::rgb(
			TAIL_COLOR.r() - tail_color_step_r * color_factor,
			TAIL_COLOR.g() - tail_color_step_g * color_factor,
			TAIL_COLOR.b() - tail_color_step_b * color_factor,
		); // Calculate the color for this segment based on the color factor

		commands.spawn((
			SpriteBundle {
				transform: Transform {
					translation: Vec3::new(seg_x, seg_y, 0.0),
					scale: tail_scale,
					..default()
				},
				sprite: Sprite {
					color: tail_color,
					..default()
				},
				..default()
			},
			TailSegment,
		));
	}
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut snake: ResMut<Snake>) {
	if let Some(key) = keyboard_input.get_just_pressed().next() {
		match key {
			KeyCode::Up => {
				if snake.vel.y != -1 {
					snake.vel.x = 0;
					snake.vel.y = 1;
				}
			}
			KeyCode::Down => {
				if snake.vel.y != 1 {
					snake.vel.x = 0;
					snake.vel.y = -1;
				}
			}
			KeyCode::Left => {
				if snake.vel.x != 1 {
					snake.vel.x = -1;
					snake.vel.y = 0;
				}
			}
			KeyCode::Right => {
				if snake.vel.x != -1 {
					snake.vel.x = 1;
					snake.vel.y = 0;
				}
			}
			_ => {}
		}
	}
}

fn spawn_player_system(mut commands: Commands) {
	let center_pos = pos_to_coord(COUNT / 2);

	commands.spawn((
		SpriteBundle {
			transform: Transform {
				translation: Vec3::new(center_pos, center_pos, 1.0),
				scale: TILE_SIZE,
				..default()
			},
			sprite: Sprite {
				color: TILE_COLOR,
				..default()
			},
			..default()
		},
		Player,
	));
}
