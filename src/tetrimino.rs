use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    surface::Surface,
    video::WindowContext,
};
use std::{
    iter::{Cycle, Peekable},
    slice::Iter,
};

type Point = (i8, i8);

pub enum Movement {
    Left,
    Right,
    Down,
    Rotate,
}

#[derive(EnumCount, EnumIter)]
pub enum Shape {
    O,
    I,
    S,
    Z,
    L,
    J,
    T,
}

pub struct TetriminoModel<'a> {
    pub texture: sdl2::render::Texture<'a>,
    pub states: Iter<'a, [Point; 4]>,
}

impl<'a> TetriminoModel<'a> {
    pub fn new(
        shape: Shape,
        surface: &mut Surface,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let (color, states) = match shape {
            Shape::O => (Color::BLUE, [[(0, 0), (1, 0), (0, 1), (1, 1)]].iter()),
            Shape::I => (
                Color::CYAN,
                [
                    [(1, 0), (1, -1), (1, 1), (1, 2)],
                    [(0, 0), (-1, 0), (1, 0), (2, 0)],
                ]
                .iter(),
            ),
            Shape::S => (
                Color::YELLOW,
                [
                    [(0, 0), (0, 1), (1, 0), (1, -1)],
                    [(0, 0), (1, 0), (0, -1), (-1, -1)],
                ]
                .iter(),
            ),
            Shape::Z => (
                Color::GREEN,
                [
                    [(0, 0), (1, 0), (1, 1), (0, -1)],
                    [(0, 0), (-1, 0), (0, -1), (1, -1)],
                ]
                .iter(),
            ),
            Shape::L => (
                Color::RED,
                [
                    [(0, 0), (0, 1), (0, -1), (1, -1)],
                    [(0, 0), (1, 0), (-1, 0), (-1, -1)],
                    [(0, 0), (0, -1), (0, 1), (-1, 1)],
                    [(0, 0), (-1, 0), (1, 0), (1, 1)],
                ]
                .iter(),
            ),
            Shape::J => (
                Color::RGB(255, 165, 0),
                [
                    [(0, 0), (0, 1), (0, -1), (-1, -1)],
                    [(0, 0), (1, 0), (-1, 0), (-1, 1)],
                    [(0, 0), (0, -1), (0, 1), (1, 1)],
                    [(0, 0), (-1, 0), (1, 0), (1, -1)],
                ]
                .iter(),
            ),
            Shape::T => (
                Color::MAGENTA,
                [
                    [(0, 0), (-1, 0), (0, 1), (1, 0)],
                    [(0, 0), (0, 1), (1, 0), (0, -1)],
                    [(0, 0), (1, 0), (0, -1), (-1, 0)],
                    [(0, 0), (0, -1), (-1, 0), (0, 1)],
                ]
                .iter(),
            ),
        };

        Self {
            texture: {
                surface
                    .fill_rect(None, color)
                    .expect("Failed to color Surface");
                surface
                    .as_texture(texture_creator)
                    .expect("Failed to create Texture")
            },
            states,
        }
    }
}

pub struct Tetrimino<'a> {
    pub coords: (i8, i8),
    pub current_state: &'a [Point; 4],
    pub states: Peekable<Cycle<Iter<'a, [Point; 4]>>>,
    pub texture: &'a Texture<'a>,
}

impl<'a> Tetrimino<'a> {
    pub fn new(coords: (i8, i8), model: &'a TetriminoModel) -> Self {
        let mut states = model.states.clone().cycle().peekable();
        Self {
            coords,
            current_state: states.next().expect("Failed to get next Tetrimino state"),
            states,
            texture: &model.texture,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, (width, height): (u32, u32)) {
        for (x, y) in self.current_state.iter() {
            canvas
                .copy(
                    &self.texture,
                    None,
                    Rect::new(
                        (*x + self.coords.0) as i32 * width as i32,
                        crate::HEIGHT as i32 - (*y + self.coords.1) as i32 * height as i32,
                        width,
                        height,
                    ),
                )
                .expect("Failed to copy Texture into Canvas")
        }
    }

    pub fn next(&mut self, movement: Movement) -> [Point; 4] {
        let mut result = *self.current_state;

        match movement {
            Movement::Left => result.iter_mut().for_each(|(x, _)| *x -= 1),
            Movement::Right => result.iter_mut().for_each(|(x, _)| *x += 1),
            Movement::Down => result.iter_mut().for_each(|(_, y)| *y -= 1),
            Movement::Rotate => {
                result = **self
                    .states
                    .peek()
                    .expect("Failed to peek next Tetrimino state")
            }
        };

        result.iter_mut().for_each(|(x, y)| {
            *x += self.coords.0;
            *y += self.coords.1;
        });

        result
    }

    pub fn advance(&mut self, movement: Movement) {
        match movement {
            Movement::Left => self.coords.0 -= 1,
            Movement::Right => self.coords.0 += 1,
            Movement::Down => self.coords.1 -= 1,
            Movement::Rotate => {
                self.current_state = &self
                    .states
                    .next()
                    .expect("Failed to get next Tetrimino state")
            }
        }
    }
}
