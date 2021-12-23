use sdl_wrapper::ScreenContextManager;
use std::cmp;

#[derive(Debug)]
pub struct Line {
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32,
}

/// Trait that defines all the line drawing functions, only the Bresenham function should ever be
/// used seriously, the others are for academic purposes only
pub trait Lines
where
    Self: Sized,
{
    fn plot_pixel(&mut self, x: u32, y: u32);

    fn naive_line(&mut self, line: &Line) {
        let min_x = cmp::min(line.x0, line.x1);
        let max_x = cmp::max(line.x0, line.x1);
        let min_y = cmp::min(line.y0, line.y1);
        let max_y = cmp::max(line.y0, line.y1);

        //println!(
        //    "min x: {}, max x: {},\nmin y: {}, max y: {}",
        //    min_x, max_x, min_y, max_y
        //);

        if max_x - min_x > max_y - min_y {
            let m: f32 =
                (line.y1 as i32 - line.y0 as i32) as f32 / (line.x1 as i32 - line.x0 as i32) as f32;
            let b: f32 = line.y0 as f32 - m * line.x0 as f32;
            let mut y: f32;

            for i in min_x..max_x {
                y = m * i as f32 + b;
                //println!("Dibujando pixel en x: {}, y: {}", i, y);
                self.plot_pixel(i as u32, y.round() as u32);
            }
        } else {
            let m: f32 =
                (line.x1 as i32 - line.x0 as i32) as f32 / (line.y1 as i32 - line.y0 as i32) as f32;
            let b: f32 = line.x0 as f32 - m * line.y0 as f32;
            let mut x: f32;

            for i in min_y..max_y {
                x = m * i as f32 + b;
                //println!("Dibujando pixel en y: {}, x: {}", i, x);
                self.plot_pixel(x.round() as u32, i as u32);
            }
        }
    }

    fn incremental_line(&mut self, line: &Line) {
        if (line.x1 as i32 - line.x0 as i32).abs() > (line.y1 as i32 - line.y0 as i32).abs() {
            let (min_x, max_x, min_y, max_y) = if line.x1 > line.x0 {
                (line.x0, line.x1, line.y0, line.y1)
            } else {
                (line.x1, line.x0, line.y1, line.y0)
            };

            let m: f32 =
                (max_y as i32 - min_y as i32) as f32 / (max_x as i32 - min_x as i32) as f32;
            let mut y: f32 = min_y as f32;

            for i in min_x..max_x {
                self.plot_pixel(i as u32, y.round() as u32);
                y += m;
            }
        } else {
            let (min_x, max_x, min_y, max_y) = if line.y1 > line.y0 {
                (line.x0, line.x1, line.y0, line.y1)
            } else {
                (line.x1, line.x0, line.y1, line.y0)
            };
            let m: f32 =
                (max_x as i32 - min_x as i32) as f32 / (max_y as i32 - min_y as i32) as f32;
            let mut x: f32 = min_x as f32;

            for i in min_y..max_y {
                self.plot_pixel(x.round() as u32, i as u32);
                x += m;
            }
        }
    }

    fn better_line(&mut self, line: &Line) {
        let width = cmp::max(
            (line.x1 as i32 - line.x0 as i32).abs(),
            (line.y1 as i32 - line.y0 as i32).abs(),
        );
        let x_step = (line.x1 as i32 - line.x0 as i32) as f32 / width as f32;
        let y_step = (line.y1 as i32 - line.y0 as i32) as f32 / width as f32;

        //println!(
        //    "Dibujando línea: {:?}\nWidth: {}\n xstep: {}  ystep: {}",
        //    &line, width, x_step, y_step
        //);

        let mut x = line.x0 as f32;
        let mut y = line.y0 as f32;

        for _ in 0..width {
            self.plot_pixel(x.round() as u32, y.round() as u32);
            x += x_step;
            y += y_step;
        }
    }

    fn bresenham_line(&mut self, line: &Line) {
        if (line.y1 as i32 - line.y0 as i32).abs() < (line.x1 as i32 - line.x0 as i32).abs() {
            if line.x1 > line.x0 {
                bresenham_horizontal(self, line.x0, line.y0, line.x1, line.y1);
            } else {
                bresenham_horizontal(self, line.x1, line.y1, line.x0, line.y0);
            }
        } else {
            if line.y1 > line.y0 {
                bresenham_vertical(self, line.x0, line.y0, line.x1, line.y1);
            } else {
                bresenham_vertical(self, line.x1, line.y1, line.x0, line.y0);
            }
        }
    }
}

fn bresenham_horizontal<T: Lines>(screen: &mut T, x0: u32, y0: u32, x1: u32, y1: u32) {
    let dy = y1 as i32 - y0 as i32;
    // Check for decreasing horizontal quadrants
    let (yi, dy) = if dy < 0 { (-1, -dy) } else { (1, dy) };

    let dx = (x1 - x0) as i32;

    let delta_h = 2 * dy;
    let delta_d = 2 * (dy - dx);

    let mut y = y0 as i32;
    let mut d = 2 * dy - dx;

    for x in x0..x1 {
        screen.plot_pixel(x, y as u32);
        if d > 0 {
            y += yi;
            d += delta_d;
        } else {
            d += delta_h;
        }
    }
}

fn bresenham_vertical<T: Lines>(screen: &mut T, x0: u32, y0: u32, x1: u32, y1: u32) {
    let dx = x1 as i32 - x0 as i32;
    // Check for backwards vertical quadrants
    let (xi, dx) = if dx < 0 { (-1, -dx) } else { (1, dx) };

    let dy = (y1 - y0) as i32;

    let delta_v = 2 * dx;
    let delta_d = 2 * (dx - dy);

    let mut x = x0 as i32;
    let mut d = 2 * dx - dy;

    for y in y0..y1 {
        screen.plot_pixel(x as u32, y);
        if d > 0 {
            x += xi;
            d += delta_d;
        } else {
            d += delta_v;
        }
    }
}

impl Lines for ScreenContextManager {
    fn plot_pixel(&mut self, x: u32, y: u32) {
        self.plot_pixel(x, y);
    }
}
