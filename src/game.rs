use crate::{tetrimino::Point, HEIGHT, TILE_SIZE, WIDTH};
use arrayvec::ArrayVec;
use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};

pub struct Field<'a>(
    pub ArrayVec<[Option<&'a Texture<'a>>; WIDTH as usize], { HEIGHT as usize + 4 }>,
);

impl<'a> Field<'a> {
    pub fn is_occupied(&self, points: [Point; 4]) -> bool {
        points.iter().any(|(x, y)| {
            (*y < 0 || *x < 0 || *x >= WIDTH as i8) || self.0[*y as usize][*x as usize].is_some()
        })
    }

    pub fn set_occupied(&mut self, points: [Point; 4], texture: &'a Texture) {
        points
            .iter()
            .for_each(|(x, y)| self.0[*y as usize][*x as usize] = Some(texture));
    }

    pub fn update_lines(&mut self, points: [Point; 4]) -> u32 {
        let mut number_of_lines = 0;
        points.iter().for_each(|(_, y)| {
            if self.0[*y as usize].iter().all(|x_line| x_line.is_some()) {
                let mut x_line = self.0.remove(*y as usize);
                x_line.iter_mut().for_each(|cell| *cell = None);
                self.0.push(x_line);
                number_of_lines += 1;
            }
        });
        number_of_lines
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, texture) in row.iter().enumerate() {
                if let Some(texture) = texture {
                    canvas
                        .copy(
                            texture,
                            None,
                            Rect::new(
                                x as i32 * TILE_SIZE as i32,
                                (HEIGHT * TILE_SIZE) as i32 - (y + 1) as i32 * TILE_SIZE as i32,
                                TILE_SIZE,
                                TILE_SIZE,
                            ),
                        )
                        .expect("Failed to copy Texture into Canvas")
                }
            }
        }
    }
}
