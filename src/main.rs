use minifb::{Key, Window, WindowOptions};
use ndarray::{Array1, arr1, arr2};

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

        // Focal length in pixels
        let focal: f32 = 50.0;

        test_cube = rotate_obj(
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

/// Rotate a 2D matix using ```yaw```, ```pitch``` and ```roll```.
///
/// # Warning
/// This rotates around ```0,0,0```
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

/// Rotate a 3D object.\
/// ```obj```: A vector of 1D arrays of f32\
/// ```yaw```, ```pitch```, ```roll```: In radians\
/// ```x```, ```y```, ```z``` offset: An offset from\
/// the center of the object, as f32
fn rotate_obj(
    obj: Vec<Array1<f32>>,
    yaw: f32,
    pitch: f32,
    roll: f32,
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
) -> Vec<Array1<f32>> {
    let mut mat_out: Vec<Array1<f32>> = vec![];
    // Getting the offset between the cube's center and 0,0,0
    let cube_offset = get_center_of_cube(obj.clone());

    // Moving cube's center to be 0,0,0
    let mat_c = translate_mat2d(
        obj,
        -cube_offset[0] + x_offset,
        -cube_offset[1] + y_offset,
        -cube_offset[2] + z_offset,
    );

    // Rotating cube
    mat_out = rotate_mat2d(mat_c, yaw, pitch, roll);

    // Moving the cube's center away from 0,0,0
    mat_out = translate_mat2d(
        mat_out,
        cube_offset[0] - x_offset,
        cube_offset[1] - y_offset,
        cube_offset[2] - z_offset,
    );

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

/// Projects a 3D point onto a 2D plane, with a focal distance.
fn project_to_2d(x: f32, y: f32, z: f32, focal: f32) -> Vec<f32> {
    let inv_z = focal / z; // perspective divide

    let x_screen = WIDTH as f32 / 2.0 + x * inv_z; // flip x
    let y_screen = HEIGHT as f32 / 2.0 - y * inv_z; // flip y

    return vec![x_screen, y_screen];
}
