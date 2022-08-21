use crate::tetrimino::{Movement, Shape, Tetrimino, TetriminoModel};
use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, surface::Surface,
};
use std::{iter, iter::FromIterator, thread::sleep, time::Duration};
use strum::{EnumCount, IntoEnumIterator};

mod game;
mod tetrimino;

const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 10;
const HEIGHT: u32 = 20;
const SPAWN_COORDS: (i8, i8) = (WIDTH as i8 / 2 - 1, HEIGHT as i8 + 1);

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2 Context");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to acquire Video Context");
    let mut canvas = video_subsystem
        .window("Tetris", WIDTH * TILE_SIZE, HEIGHT * TILE_SIZE)
        .build()
        .expect("Failed to build Window")
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to build Canvas");
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to acquire SDL2 Event Pump");

    canvas.set_draw_color(Color::BLACK);
    let texture_creator = canvas.texture_creator();
    let mut surface = Surface::new(TILE_SIZE, TILE_SIZE, PixelFormatEnum::RGB24)
        .expect("Failed to create Surface");

    let models = Shape::iter()
        .map(|shape| TetriminoModel::new(shape, &mut surface, &texture_creator))
        .collect::<ArrayVec<_, { Shape::COUNT }>>();

    let mut rng = rand::thread_rng();
    let mut models_bag = iter::repeat(ArrayVec::<_, { Shape::COUNT }>::from_iter(&models))
        .flat_map(|mut models_refs| {
            models_refs.shuffle(&mut rng);
            models_refs
        });

    let mut tetrimino = Tetrimino::new(
        SPAWN_COORDS,
        models_bag
            .next()
            .expect("Failed to get next Tetrimino Model"),
    );
    let mut movement = None;
    let mut field = game::Field(ArrayVec::from(
        [[None; WIDTH as usize]; HEIGHT as usize + 4],
    ));

    let scores_per_lines = [0, 40, 100, 300, 1200];
    let mut total_cleared_lines = 0;
    let mut score = 0;
    let mut level = 0;

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => break 'game,
                    Keycode::Up => movement = Some(Movement::Rotate),
                    Keycode::Left => movement = Some(Movement::Left),
                    Keycode::Right => movement = Some(Movement::Right),
                    Keycode::Down => movement = Some(Movement::Down),
                    _ => (),
                },
                _ => (),
            }
        }

        if let Some(movement) = movement.take() {
            if !field.is_occupied(tetrimino.next_state(movement)) {
                tetrimino.advance(movement);
            } else if movement == Movement::Down {
                let current_state_coords = tetrimino.current_state();
                field.set_occupied(current_state_coords, tetrimino.texture);

                let cleared_lines = field.update_lines(current_state_coords);
                total_cleared_lines += cleared_lines;
                score += (level + 1) * scores_per_lines[cleared_lines as usize];
                level = total_cleared_lines / 10;

                tetrimino = Tetrimino::new(
                    SPAWN_COORDS,
                    models_bag
                        .next()
                        .expect("Failed to get next Tetrimino Model"),
                );
            }
        }

        canvas.clear();
        tetrimino.render(&mut canvas);
        field.render(&mut canvas);
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    println! {"{score}"};
}
