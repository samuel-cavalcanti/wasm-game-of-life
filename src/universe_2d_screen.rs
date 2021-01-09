use crate::conways_game_of_life::{Universe, Cell};
use web_sys::CanvasRenderingContext2d;

use crate::colors::Colors;

pub struct Universe2DScreen {
    context: CanvasRenderingContext2d,
    cell_size_in_pixels: i32,
    universe: Universe,
}

impl Universe2DScreen {
    pub fn new(context: CanvasRenderingContext2d, universe: Universe) -> Universe2DScreen {
        Universe2DScreen {
            context,
            cell_size_in_pixels: 5,
            universe,
        }
    }

    pub fn draw(&mut self) {
        self.universe.tick();
        self.draw_grid();
        self.draw_cells();
    }

    fn draw_grid(&self) {
        self.context.begin_path();

        self.context.set_stroke_style(&Colors::GridColor.to_hex_string().into());
        let cell_size = (self.cell_size_in_pixels as f64) + 1.0;
        let height = self.universe.height as f64;
        let width = self.universe.width as f64;

        for i in 0..self.universe.width {
            let i = i as f64;

            let x = i * cell_size + 1.0;
            let y = cell_size * height + 1.0;

            self.context.move_to(x, 0.0);
            self.context.line_to(x, y)
        }

        for j in 0..self.universe.height {
            let j = j as f64;

            let x = cell_size * width + 1.0;
            let y = j * cell_size + 1.0;


            self.context.move_to(0.0, y);
            self.context.line_to(x, y)
        }

        self.context.stroke();
    }

    fn draw_cells(&self) {
        let cell_size = self.cell_size_in_pixels as f64;
        let cell_size_plus_offset = cell_size + 2.0;

        for row in 0..self.universe.height {
            for col in 0..self.universe.width {
                let cell = self.universe.get_cell(row, col);

                let color = match cell {
                    Cell::Dead => { Colors::DeadColor.to_hex_string() }
                    Cell::Alive => { Colors::AliveColor.to_hex_string() }
                };

                let rect_x = cell_size_plus_offset * (col as f64);
                let rect_y = cell_size_plus_offset * (row as f64);

                self.context.set_fill_style(&color.into());

                self.context.fill_rect(rect_x, rect_y, cell_size, cell_size);
            }
        }

        self.context.stroke();
    }
}