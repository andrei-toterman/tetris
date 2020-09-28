use crate::tetrimino::{Movement, Shape, Tetrimino, TetriminoModel};
use rand::seq::SliceRandom;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, surface::Surface,
};
use stackvec::TryCollect;
use std::{error::Error, thread::sleep, time::Duration};

mod tetrimino;

const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 10 * TILE_SIZE;
const HEIGHT: u32 = 20 * TILE_SIZE;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut canvas = video_subsystem
        .window("Tetris", WIDTH, HEIGHT)
        .build()?
        .into_canvas()
        .present_vsync()
        .build()?;
    let texture_creator = canvas.texture_creator();
    let mut surface = Surface::new(TILE_SIZE, TILE_SIZE, PixelFormatEnum::RGB24).unwrap();

    let mut models = [
        Shape::O,
        Shape::I,
        Shape::S,
        Shape::Z,
        Shape::L,
        Shape::J,
        Shape::T,
    ]
    .iter()
    .map(|shape| TetriminoModel::new(*shape, &mut surface, &texture_creator))
    .try_collect::<[TetriminoModel; 7]>()
    .expect("Failed to collect Tetrimino models");

    let mut rng = rand::thread_rng();
    models.shuffle(&mut rng);

    let mut models_iterator = models.iter_mut();

    let current_model = models_iterator.next().expect("Failed to get next model");

    let mut current_states = current_model.states.clone().cycle().peekable();
    let mut current_tetrimino = Tetrimino {
        coords: (5, 10),
        current_state: current_states.next().expect("Failed to get current states"),
        states: current_states,
        texture: &current_model.texture,
    };

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Up => current_tetrimino.advance(Movement::Rotate),
                    Keycode::Left => current_tetrimino.advance(Movement::Left),
                    Keycode::Right => current_tetrimino.advance(Movement::Right),
                    _ => (),
                },
                _ => (),
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        current_tetrimino.render(&mut canvas, (TILE_SIZE, TILE_SIZE));

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    Ok(())
}
