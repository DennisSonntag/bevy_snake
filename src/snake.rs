#![allow(dead_code, unused, clippy::module_name_repetitions, clippy::needless_pass_by_value)]

use std::time::Duration;

use bevy::{app::AppExit, prelude::*, time::common_conditions::on_timer};

use crate::{
	coord_to_pos, pos_to_coord, rand_int_range, Food, Player, Point, Snake, TailSegment, COUNT,
	HEAD_COLOR, HEIGHT, TAIL_COLOR, TAIL_COLOR_MIN, TAIL_SIZE, TAIL_SIZE_MIN, TILE_SIZE, WIDTH,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_player_system).add_systems(
			Update,
			(
				keyboard_input_system,
				snake_movment_system.run_if(on_timer(Duration::from_secs_f32(0.08))),
			),
		);
	}
}

fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
	move |mut timer: Local<f32>, time: Res<Time>| {
		// Tick the timer
		*timer += time.delta_seconds();
		// Return true if the timer has passed the time
		*timer >= t
	}
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
	to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn snake_movment_system(
	mut snake: ResMut<Snake>,
	mut player_query: Query<&mut Transform, (With<Player>, Without<Food>)>,
	mut food_query: Query<&mut Transform, With<Food>>,
	mut exit_events: EventWriter<AppExit>,
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

	if snake.tail.contains(&Point {
		x: player_x,
		y: player_y,
	}) {
		snake.len = 5;
	}

	if snake.len == (3 * 3) {
		exit_events.send(AppExit);
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

	let num_segments = snake.tail.len();
	let tail_size_step = (TAIL_SIZE - TAIL_SIZE_MIN) / (num_segments - 1) as f32;

	let tail_color_step_r = (TAIL_COLOR.r() - TAIL_COLOR_MIN.r()) / (num_segments - 1) as f32;
	let tail_color_step_g = (TAIL_COLOR.g() - TAIL_COLOR_MIN.g()) / (num_segments - 1) as f32;
	let tail_color_step_b = (TAIL_COLOR.b() - TAIL_COLOR_MIN.b()) / (num_segments - 1) as f32;

	for (index, point) in snake.tail.iter().enumerate() {
		let (seg_x, seg_y) = (pos_to_coord(point.x), pos_to_coord(point.y));
		let tail_scale = TAIL_SIZE - tail_size_step * (num_segments - index - 1) as f32;

		let tail_color = Color::rgb(
			tail_color_step_r.mul_add(index as f32 + 1., TAIL_COLOR_MIN.r()),
			tail_color_step_g.mul_add(index as f32 + 1., TAIL_COLOR_MIN.g()),
			tail_color_step_b.mul_add(index as f32 + 1., TAIL_COLOR_MIN.b())
		);

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
				color: HEAD_COLOR,
				..default()
			},
			..default()
		},
		Player,
	));
}
