use minifb::{Key, Window, WindowOptions};
use ndarray::{Array1, arr1, arr2};

use crate::assets::dd_drawing;
use crate::assets::matrix;

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

    // 3D cube
    let mut test_cube = vec![
        arr1(&[0.0, 0.0, 4.0, 1.0]),
        arr1(&[0.0, 0.0, 8.0, 1.0]),
        arr1(&[4.0, 0.0, 8.0, 1.0]),
        arr1(&[4.0, 0.0, 4.0, 1.0]),
        arr1(&[0.0, 4.0, 4.0, 1.0]),
        arr1(&[0.0, 4.0, 8.0, 1.0]),
        arr1(&[4.0, 4.0, 8.0, 1.0]),
        arr1(&[4.0, 4.0, 4.0, 1.0]),
    ];
    // test_cube = translate_mat2d(test_cube, 0.0, 0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // Resets all pixels to black
        }

        // Focal length in pixels
        let focal: f32 = 50.0;

        test_cube = matrix::rotate_obj(
            test_cube,
            3.1415 / 150.0,
            3.1415 / 150.0,
            3.1415 / 150.0,
            0.0,
            0.0,
            0.0,
        );

        // Drawing the cube
        for i in 0..test_cube.len() {
            // Only project points in front of the camera.
            if test_cube[i][2] > 0.001 {
                // Drawing verticies of cube
                let proj_coords =
                    project_to_2d(test_cube[i][0], test_cube[i][1], test_cube[i][2], focal);

                // Bounds check before converting to an index.
                if (0.0..WIDTH as f32).contains(&proj_coords[0])
                    && (0.0..HEIGHT as f32).contains(&proj_coords[1])
                {
                    let (index, col) = dd_drawing::draw_pixel(
                        proj_coords[0] as usize,
                        proj_coords[1] as usize,
                        RED,
                    );
                    buffer[index] = col;
                }
            }

            // Drawing edges of the cube
            if i < test_cube.len() - 1 {
                let proj_pt1 =
                    project_to_2d(test_cube[i][0], test_cube[i][1], test_cube[i][2], focal);
                let proj_pt2 = project_to_2d(
                    test_cube[i + 1][0],
                    test_cube[i + 1][1],
                    test_cube[i + 1][2],
                    focal,
                );

                let line = dd_drawing::draw_line(
                    proj_pt1[0] as usize,
                    proj_pt1[1] as usize,
                    proj_pt2[0] as usize,
                    proj_pt2[1] as usize,
                    GREEN,
                );
                for i in 0..(line.len()) {
                    buffer[line[i].0] = line[i].1;
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

/// Projects a 3D point onto a 2D plane, with a focal distance.
fn project_to_2d(x: f32, y: f32, z: f32, focal: f32) -> Vec<f32> {
    let inv_z = focal / z; // perspective divide

    let x_screen = WIDTH as f32 / 2.0 + x * inv_z; // flip x
    let y_screen = HEIGHT as f32 / 2.0 - y * inv_z; // flip y

    return vec![x_screen, y_screen];
}
