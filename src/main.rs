use crate::tetrimino::{Movement, Shape, Tetrimino, TetriminoModel};
use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, surface::Surface,
};
use std::{iter, iter::FromIterator, thread::sleep, time::Duration};

#[macro_use]
extern crate strum_macros;
use strum::{EnumCount, IntoEnumIterator};

mod game;
mod tetrimino;

const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 10 * TILE_SIZE;
const HEIGHT: u32 = 20 * TILE_SIZE;

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2 Context");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to acquire Video Context");
    let mut canvas = video_subsystem
        .window("Tetris", WIDTH, HEIGHT)
        .build()
        .expect("Failed to build Window")
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to build Canvas");
    canvas.set_draw_color(Color::BLACK);

    let texture_creator = canvas.texture_creator();
    let mut surface = Surface::new(TILE_SIZE, TILE_SIZE, PixelFormatEnum::RGB24)
        .expect("Failed to create Surface");

    let models = Shape::iter()
        .map(|shape| TetriminoModel::new(shape, &mut surface, &texture_creator))
        .collect::<ArrayVec<[_; Shape::COUNT]>>();

    let mut rng = rand::thread_rng();
    let mut models_bag = iter::repeat(ArrayVec::<[_; Shape::COUNT]>::from_iter(0..))
        .map(|mut indices| {
            indices.shuffle(&mut rng);
            indices
        })
        .flatten()
        .map(|index| &models[index]);

    let mut current_tetrimino = Tetrimino::new(
        (5, 10),
        models_bag
            .next()
            .expect("Failed to get next Tetrimino Model"),
    );

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to acquire SDL2 Event Pump");

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => break 'game,
                    Keycode::Up => current_tetrimino.advance(Movement::Rotate),
                    Keycode::Left => current_tetrimino.advance(Movement::Left),
                    Keycode::Right => current_tetrimino.advance(Movement::Right),
                    Keycode::Down => current_tetrimino.advance(Movement::Down),
                    Keycode::Space => {
                        current_tetrimino = Tetrimino::new(
                            current_tetrimino.coords,
                            models_bag
                                .next()
                                .expect("Failed to get next Tetrimino Model"),
                        )
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        canvas.clear();

        current_tetrimino.render(&mut canvas, (TILE_SIZE, TILE_SIZE));

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
