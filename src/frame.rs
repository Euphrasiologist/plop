use crate::{Canvas, Data};

pub const PAD: usize = 2;

pub(crate) struct Frame {
    width: usize,
    height: usize,

    min_x: f64,
    max_x: f64,
    range_x: f64,

    min_y: f64,
    max_y: f64,
    range_y: f64,
}

impl Frame {
    pub(crate) fn new_over(width: usize, height: usize, data: &Data) -> Self {
        // Calculate X bounds
        let min_x = data
            .xs
            .iter()
            .filter(|v| v.is_finite())
            .copied()
            .min_by(f64::total_cmp);
        let max_x = data
            .xs
            .iter()
            .filter(|v| v.is_finite())
            .copied()
            .max_by(f64::total_cmp);

        // Calculate Y bounds
        let min_y = data
            .ys
            .iter()
            .flatten()
            .filter(|v| v.is_finite())
            .copied()
            .min_by(f64::total_cmp);
        let max_y = data
            .ys
            .iter()
            .flatten()
            .filter(|v| v.is_finite())
            .copied()
            .max_by(f64::total_cmp);

        let (Some(mut min_x), Some(mut max_x)) = (min_x, max_x) else {
            panic!("No finite x-values found in data");
        };
        let (Some(mut min_y), Some(mut max_y)) = (min_y, max_y) else {
            panic!("No finite y-values found in data");
        };

        // Make sure ranges are not zero (avoid divide-by-zero)
        let mut range_x = max_x - min_x;
        if range_x == 0.0 {
            range_x = 1.0;
            max_x = min_x + range_x;
        }

        let mut range_y = max_y - min_y;
        if range_y == 0.0 {
            range_y = 1.0;
            max_y = min_y + range_y;
        }

        // Force Y min to 0.0 if all data is above 0 (common for positive distributions)
        if min_y > 0.0 {
            min_y = 0.0;
            range_y = max_y - min_y;
            if range_y == 0.0 {
                range_y = 1.0; // Safety fallback
                max_y = min_y + range_y;
            }
        }

        Self {
            width,
            height,
            min_x,
            max_x,
            range_x,
            min_y,
            max_y,
            range_y,
        }
    }

    pub(crate) fn x_bounds(&self) -> (f64, f64) {
        (self.min_x, self.max_x)
    }

    pub(crate) fn y_bounds(&self) -> (f64, f64) {
        (self.min_y, self.max_y)
    }

    pub(crate) fn range_xy(&self) -> (f64, f64) {
        (self.range_x, self.range_y)
    }

    pub(crate) fn x_to_column(&self, x: f64) -> usize {
        let plot_width = (self.width - PAD) as f64;
        let x_as_fraction_of_axis = (x - self.min_x) / self.range_x;
        let x_cell = (plot_width * x_as_fraction_of_axis).round() as usize;
        x_cell
    }

    pub(crate) fn y_to_row(&self, y: f64) -> usize {
        let plot_height = (self.height - PAD) as f64;
        let y_as_fraction_of_axis = (y - self.min_y) / self.range_y;
        let y_cell_from_top = (plot_height * y_as_fraction_of_axis).round() as usize;
        // flip y; 0 at bottom of plot
        let y_cell = self.height - y_cell_from_top - 1;
        y_cell
    }

    pub(crate) fn point_to_cell(&self, (x, y): (f64, f64)) -> (usize, usize) {
        (self.y_to_row(y), self.x_to_column(x))
    }

    pub(crate) fn draw_into(&self, canvas: &mut Canvas) {
        // figure out where to draw the axes in the frame
        let y0_is_visible = self.min_y <= 0. && self.max_y >= 0.;
        let x0_is_visible = self.min_x <= 0. && self.max_x >= 0.;

        let mut draw_vertical_at_x = 0.;
        let mut draw_horizontal_at_y = 0.;
        if !x0_is_visible {
            if self.min_x > 0. {
                draw_vertical_at_x = self.min_x;
            } else {
                draw_vertical_at_x = self.max_x;
            }
        }
        if !y0_is_visible {
            if self.min_y > 0. {
                draw_horizontal_at_y = self.min_y;
            } else {
                draw_horizontal_at_y = self.max_y;
            }
        }

        let draw_horizontal_at_row = self.point_to_cell((0., draw_horizontal_at_y)).0;
        let draw_vertical_at_column = self.point_to_cell((draw_vertical_at_x, 0.)).1;

        // draw in the axes
        // draw the vertical (Y) axis (so where X = 0)
        for row in 0..self.height {
            let c = if x0_is_visible {
                if row % 5 == 0 {
                    b'+'
                } else {
                    b'|'
                }
            } else {
                if row % 5 == 0 {
                    b'.'
                } else {
                    b' '
                }
            };
            let Some(cell) = canvas.cell(row, draw_vertical_at_column) else {
                panic!("invalid cell ({row}, {draw_vertical_at_column}) for axis component ({draw_vertical_at_x}, _)");
            };
            *cell = c;
        }
        // draw the horizontal (X) axis (so where Y = 0)
        for column in 0..self.width {
            let c = if y0_is_visible {
                if column % 5 == 0 {
                    b'+'
                } else {
                    b'-'
                }
            } else {
                if column % 5 == 0 {
                    b'.'
                } else {
                    b' '
                }
            };
            let Some(cell) = canvas.cell(draw_horizontal_at_row, column) else {
                panic!("invalid cell ({draw_horizontal_at_row}, {column}) for axis component ({draw_horizontal_at_y}, _)");
            };
            *cell = c;
        }
        // where the axes meet, put a +
        let intersection = canvas
            .cell(draw_horizontal_at_row, draw_vertical_at_column)
            .expect("must have hit one of the panics above");
        *intersection = b'+';
    }
}
