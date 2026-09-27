use minifb::{Key, Window, WindowOptions};
use ndarray::arr2;

use crate::assets::dd_drawing;

pub mod assets;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

const RED: u32 = 0x00ff0000;
const GREEN: u32 = 0x0000ff00;
const BLUE: u32 = 0x000000ff;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // Resets all pixels to black
        }

        // let rect = dd_drawing::draw_rect(20, 40, 40, 20, BLUE);
        // for i in 0..rect.len() {
        //     buffer[rect[i].0] = rect[i].1;
        // }

        // let line = dd_drawing::draw_line(40, 40, 10, 20, GREEN);
        // for i in 0..(line.len()) {
        //     buffer[line[i].0] = line[i].1;
        // }

        // let triangle = dd_drawing::draw_triangle(150, 50, 200, 150, 50, 170, RED);
        // for i in 0..(triangle.len()) {
        //     buffer[triangle[i].0] = triangle[i].1;
        // }

        // Focal length in pixels
        let focal: f32 = 50.0;

        // 3D cube
        let mut test_cube = arr2(&[
            [0.0, 0.0, 4.0],
            [0.0, 0.0, 8.0],
            [4.0, 0.0, 8.0],
            [4.0, 0.0, 4.0],
            [0.0, 4.0, 4.0],
            [0.0, 4.0, 8.0],
            [4.0, 4.0, 8.0],
            [4.0, 4.0, 4.0],
        ]);

        let angle: f32 = 3.1415 / 10.0;
        let rotation_matrix = arr2(&[
            [1.0, 0.0, 0.0],
            [0.0, angle.cos(), -1.0 * angle.sin()],
            [0.0, angle.sin(), angle.cos()],
        ]);

        test_cube = test_cube.dot(&rotation_matrix);

        for i in 0..test_cube.dim().0 {
            // Only project points in front of the camera.
            if test_cube[(i, 2)] > 0.001 {
                let inv_z = focal / test_cube[(i, 2)]; // perspective divide

                let x_screen = WIDTH as f32 / 2.0 + test_cube[(i, 0)] * inv_z; // flip x
                let y_screen = HEIGHT as f32 / 2.0 - test_cube[(i, 1)] * inv_z; // flip y

                // Bounds check before converting to an index.
                if (0.0..WIDTH as f32).contains(&x_screen)
                    && (0.0..HEIGHT as f32).contains(&y_screen)
                {
                    let (index, col) =
                        dd_drawing::draw_pixel(x_screen as usize, y_screen as usize, RED);
                    buffer[index] = col;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
