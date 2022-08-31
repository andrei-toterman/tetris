use crate::tetrimino::{Movement, Shape, Tetrimino, TetriminoModel};
use arrayvec::ArrayVec;
use rand::seq::SliceRandom;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, rect::Rect,
    surface::Surface,
};
use std::{
    boxed::Box,
    iter,
    iter::FromIterator,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};
use strum::{EnumCount, IntoEnumIterator};

mod game;
mod tetrimino;

struct Tick;

const TICK_SPEED: u32 = 500;
const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 10;
const HEIGHT: u32 = 20;
const SPAWN_COORDS: (i8, i8) = (WIDTH as i8 / 2 - 1, HEIGHT as i8 + 1);

fn main() {
    let sdl2 = sdl2::init().expect("failed to initialize sdl2");
    let video_subsystem = sdl2.video().expect("failed to get video subsystem");
    let mut canvas = video_subsystem
        .window("Tetris", WIDTH * TILE_SIZE, HEIGHT * TILE_SIZE)
        .build()
        .expect("failed to build window")
        .into_canvas()
        .present_vsync()
        .build()
        .expect("failed to build canvas");
    let mut event_pump = sdl2.event_pump().expect("failed to get event pump");
    let event_subsystem = sdl2.event().expect("failed to get event pump");
    let timer_subsystem = sdl2.timer().expect("failed to get timer subsystem");
    let ttf_subsystem = sdl2::ttf::init().expect("failed to get ttf subsystem");

    let font = sdl2::rwops::RWops::from_bytes(include_bytes!("font.ttf"))
        .expect("failed to load font into rwops");
    let font = ttf_subsystem
        .load_font_from_rwops(font, 12)
        .expect("failed to load font from rwops");

    event_subsystem
        .register_custom_event::<Tick>()
        .expect("failed to register tick event");

    canvas.set_draw_color(Color::BLACK);
    let texture_creator = canvas.texture_creator();
    let mut surface = Surface::new(TILE_SIZE, TILE_SIZE, PixelFormatEnum::RGB24)
        .expect("failed to create surface");

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
            .expect("failed to get next tetrimino model"),
    );
    let mut movement = None;
    let mut field = game::Field(ArrayVec::from(
        [[None; WIDTH as usize]; HEIGHT as usize + 4],
    ));
    let scores_per_lines = [0, 40, 100, 300, 1200];
    let mut total_cleared_lines = 0;
    let mut score = 0;
    let mut level = 1;
    let tick_speed = Arc::new(RwLock::new(TICK_SPEED));
    let timer_tick_speed = tick_speed.clone();
    let _timer = timer_subsystem.add_timer(
        TICK_SPEED,
        Box::new(|| {
            event_subsystem
                .push_custom_event(Tick)
                .expect("failed to push tick event");
            *timer_tick_speed.read().expect("failed to lock tick speed")
        }),
    );

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => *tick_speed.write().expect("failed to lock tick speed") = TICK_SPEED,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => break 'game,
                    Keycode::Up => movement = Some(Movement::Rotate),
                    Keycode::Left => movement = Some(Movement::Left),
                    Keycode::Right => movement = Some(Movement::Right),
                    Keycode::Down => {
                        movement = Some(Movement::Down);
                        *tick_speed.write().expect("failed to lock tick speed") = TICK_SPEED / 10;
                    }
                    _ => (),
                },
                event if event.as_user_event_type::<Tick>().is_some() => {
                    movement = Some(Movement::Down)
                }

                _ => (),
            }
        }

        if let Some(movement) = movement.take() {
            if !field.is_occupied(tetrimino.next_state(movement)) {
                tetrimino.advance(movement);
            } else if movement == Movement::Down {
                let current_state_coords = tetrimino.current_state();
                if current_state_coords.iter().all(|&(_, y)| y >= HEIGHT as i8) {
                    println!("game over");
                    break 'game;
                }
                field.set_occupied(current_state_coords, tetrimino.texture);

                let cleared_lines = field.update_lines(current_state_coords);
                total_cleared_lines += cleared_lines;
                score += level * scores_per_lines[cleared_lines as usize];
                level = total_cleared_lines / 10 + 1;

                tetrimino = Tetrimino::new(
                    SPAWN_COORDS,
                    models_bag
                        .next()
                        .expect("failed to get next tetrimino model"),
                );
            }
        }

        canvas.clear();
        tetrimino.render(&mut canvas);
        field.render(&mut canvas);
        let score = score.to_string();
        canvas
            .copy(
                &font
                    .render(&score)
                    .solid(Color::WHITE)
                    .expect("failed to render score to surface")
                    .as_texture(&texture_creator)
                    .expect("failed to render score to texture"),
                None,
                Rect::new(
                    (WIDTH * TILE_SIZE) as i32 - 30 * score.len() as i32,
                    5,
                    30 * score.len() as u32,
                    30,
                ),
            )
            .expect("failed to copy score texture to canvas");

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    println!("{score}");
}
