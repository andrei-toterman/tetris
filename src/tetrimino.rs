use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

type Point = (i8, i8);

#[derive(Copy, Clone)]
enum Shape {
    I,
    O,
    S,
    Z,
    L,
    J,
    T,
}

impl Shape {
    fn color(&self) -> Color {
        match self {
            Shape::I => Color::CYAN,
            Shape::O => Color::BLUE,
            Shape::S => Color::YELLOW,
            Shape::Z => Color::GREEN,
            Shape::L => Color::RED,
            Shape::J => Color::RGB(255, 165, 0),
            Shape::T => Color::MAGENTA,
        }
    }

    fn points(&self) -> [Point; 4] {
        match self {
            Shape::I => [(0, 0), (0, 0), (0, 0), (0, 0)],
            Shape::O => [(0, 0), (0, 0), (0, 0), (0, 0)],
            Shape::S => [(0, 0), (0, 0), (0, 0), (0, 0)],
            Shape::Z => [(0, 0), (0, 0), (0, 0), (0, 0)],
            Shape::L => [(0, 0), (0, 0), (0, 0), (0, 0)],
            Shape::J => [(0, 0), (0, 0), (0, 0), (0, 0)],
            Shape::T => [(5, 5), (5, 6), (4, 5), (6, 5)],
        }
    }
}

#[derive(Copy, Clone)]
enum Movement {
    Left,
    Right,
    Down,
    Rotate,
}

struct Tetrimino<'a> {
    shape: Shape,
    points: [Point; 4],
    texture: Texture<'a>,
}

impl<'a> Tetrimino<'a> {
    fn render(&self, canvas: &mut WindowCanvas, (width, height): (u32, u32)) {
        for (x, y) in self.points.iter() {
            canvas
                .copy(
                    &self.texture,
                    None,
                    Rect::new(
                        *x as i32 * width as i32,
                        *y as i32 * height as i32,
                        width,
                        height,
                    ),
                )
                .unwrap_or_else(|err| panic!(err))
        }
    }

    fn next(&self, movement: Movement) -> [Point; 4] {
        let mut result = self.points;

        match movement {
            Movement::Left => result.iter_mut().for_each(|(x, _)| *x -= 1),
            Movement::Right => result.iter_mut().for_each(|(x, _)| *x += 1),
            Movement::Down => result.iter_mut().for_each(|(_, y)| *y -= 1),
            Movement::Rotate => match self.shape {
                Shape::I => {}
                Shape::O => {}
                Shape::S => {}
                Shape::Z => {}
                Shape::L => {}
                Shape::J => {}
                Shape::T => {}
            },
        }

        result
    }

    fn advance(&mut self, movement: Movement) {
        self.points = self.next(movement);
    }
}
