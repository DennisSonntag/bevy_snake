#![allow(dead_code)]

use bevy::{prelude::*, window::PresentMode};
use food::FoodPlugin;
use rand::{thread_rng, Rng};
use snake::SnakePlugin;

mod components;
mod food;
mod snake;

const HEIGHT: f32 = 900.;
const WIDTH: f32 = 900.;
const COUNT: i32 = 20;
const SIZE: f32 = HEIGHT / COUNT as f32;
const TILE_SIZE: Vec3 = Vec3::new(SIZE, SIZE, 0.0);
const TAIL_SIZE: Vec3 = Vec3::new(SIZE - 2., SIZE - 2., 0.0);
const TAIL_SIZE_MIN: Vec3 = Vec3::new(SIZE - 10., SIZE - 10., 0.0);
const TILE_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const TAIL_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const TAIL_COLOR_MIN: Color = Color::rgb(0.5, 0.0, 1.0);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const FIXED_TIMESTEP: f32 = 0.5;

#[derive(Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct TailSegment;

#[derive(Resource)]
struct Snake {
	vel: Point,
	tail: Vec<Point>,
	len: i32,
}

impl FromWorld for Snake {
	fn from_world(_world: &mut World) -> Self {
		Self {
			vel: Point { x: 0, y: 0 },
			tail: vec![],
			len: 5,
		}
	}
}

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "snake".into(),
				resolution: (WIDTH, HEIGHT).into(),
				present_mode: PresentMode::AutoVsync,
				// Tells wasm to resize the window according to the available canvas
				fit_canvas_to_parent: true,
				// Tells wasm not to override default event handling, like F5, Ctrl+R etc.
				prevent_default_event_handling: false,
				..default()
			}),
			..default()
		}))
		.add_plugin(FoodPlugin)
		.add_plugin(SnakePlugin)
		.add_startup_system(setup_system)
		.init_resource::<Snake>()
		.run();
}

fn pos_to_coord(pos: i32) -> f32 {
	(pos as f32 * TILE_SIZE.x) - (WIDTH / 2.) + (TILE_SIZE.x / 2.)
}

fn coord_to_pos(coord: f32) -> i32 {
	((coord + (WIDTH / 2.) - (TILE_SIZE.x / 2.)) / TILE_SIZE.x) as i32
}

fn rand_int_range(min: i32, max: i32) -> i32 {
	thread_rng().gen_range(min..=max)
}

fn setup_system(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}
