use minifb::{Key, Window, WindowOptions};
use ndarray::{Array1, Array2, arr1, arr2};

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

        // Getting the offset between the cube's center and 0,0,0
        let cube_offset = get_center_of_cube(test_cube.clone());

        // Moving cube's center to be 0,0,0
        test_cube = translate_mat2d(test_cube, -cube_offset[0], -cube_offset[1], -cube_offset[2]);

        // Rotating cube
        test_cube = rotate_mat2d(test_cube, 0.0, 0.0, 3.1415 / 100.0);

        // Moving the cube's center away from 0,0,0
        test_cube = translate_mat2d(test_cube, cube_offset[0], cube_offset[1], cube_offset[2]);

        // Drawing the points of the cube
        for i in 0..test_cube.len() {
            // Only project points in front of the camera.
            if test_cube[i][2] > 0.001 {
                let inv_z = focal / test_cube[i][2]; // perspective divide

                let x_screen = WIDTH as f32 / 2.0 + test_cube[i][0] * inv_z; // flip x
                let y_screen = HEIGHT as f32 / 2.0 - test_cube[i][1] * inv_z; // flip y

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

/// Rotate a 2D matix using ```yaw```, ```pitch``` and ```roll```.
fn rotate_mat2d(mat: Vec<Array1<f32>>, yaw: f32, pitch: f32, roll: f32) -> Vec<Array1<f32>> {
    let mut mat_out: Vec<Array1<f32>> = vec![];
    let rotation_mat = arr2(&[
        [
            yaw.cos() * pitch.cos(),
            yaw.cos() * pitch.sin() * roll.sin() - yaw.sin() * roll.cos(),
            yaw.cos() * pitch.sin() * roll.sin() + yaw.sin() * roll.cos(),
            0.0,
        ],
        [
            yaw.sin() * pitch.cos(),
            yaw.sin() * pitch.sin() * roll.sin() + yaw.cos() * roll.cos(),
            yaw.sin() * pitch.sin() * roll.cos() - yaw.cos() * roll.sin(),
            0.0,
        ],
        [
            -pitch.sin(),
            pitch.cos() * roll.sin(),
            pitch.cos() * roll.cos(),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    for i in 0..mat.len() {
        mat_out.push(mat[i].dot(&rotation_mat));
    }

    return mat_out;
}

/// Translate a 2D matix using ```tx```, ```ty``` and ```tz```.
fn translate_mat2d(mat: Vec<Array1<f32>>, tx: f32, ty: f32, tz: f32) -> Vec<Array1<f32>> {
    let mut mat_out: Vec<Array1<f32>> = vec![];

    for i in 0..mat.len() {
        mat_out.push(arr1(&[
            mat[i][0] + tx,
            mat[i][1] + ty,
            mat[i][2] + tz,
            mat[i][3],
        ]));
    }

    return mat_out;
}

/// Returns a vector with the ```x```, ```y``` and ```z``` offset the cube has from ```0,0,0```.
fn get_center_of_cube(mat: Vec<Array1<f32>>) -> Vec<f32> {
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut z_sum = 0.0;
    let n_pts = mat.len();

    for i in 0..n_pts {
        x_sum += mat[i][0];
        y_sum += mat[i][1];
        z_sum += mat[i][2];
    }

    return vec![
        x_sum / n_pts as f32,
        y_sum / n_pts as f32,
        z_sum / n_pts as f32,
    ];
}
