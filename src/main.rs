use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, render::Texture,
    surface::Surface,
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

    let tetriminos = [
        Shape::I,
        Shape::O,
        Shape::S,
        Shape::Z,
        Shape::L,
        Shape::J,
        Shape::T,
    ]
    .iter()
    .map(|shape| {
        surface
            .fill_rect(None, shape.color())
            .expect("Failed to color surface");
        Tetrimino {
            shape: *shape,
            points: shape.points(),
            texture: surface
                .as_texture(&texture_creator)
                .expect("Failed to create texture"),
        }
    })
    .try_collect::<[Tetrimino; 7]>()
    .expect("Failed to collect Tetriminos");

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
                _ => (),
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let t = &tetriminos[6];
        t.draw(&mut canvas);

        canvas.present();

        sleep(Duration::new(1, 1_000_000_000 / 60));
    }

    Ok(())
}
