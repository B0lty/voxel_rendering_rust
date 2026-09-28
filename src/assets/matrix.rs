use crate::assets::ddd_general::{Vertex, vertex};
use ndarray::{Array1, arr1, arr2};

fn vertex_to_matrix(vertex: Vertex) -> Array1<f32> {
    return arr1(&[vertex.x, vertex.y, vertex.z, vertex.is_pt]);
}

fn matrix_to_vertex(mat: Array1<f32>) -> Vertex {
    return Vertex {
        x: mat[0],
        y: mat[1],
        z: mat[2],
        is_pt: mat[3],
    };
}

/// Translate a 2D matix using ```tx```, ```ty``` and ```tz```.
pub fn translate_mat2d(mat: Vec<Vertex>, tx: f32, ty: f32, tz: f32) -> Vec<Vertex> {
    let mut mat_out: Vec<Vertex> = vec![];

    for i in 0..mat.len() {
        mat_out.push(vertex(mat[i].x + tx, mat[i].y + ty, mat[i].z + tz));
    }

    return mat_out;
}

/// Rotate a 2D matix using ```yaw```, ```pitch``` and ```roll```.
///
/// # Warning
/// This rotates around ```0,0,0```
fn rotate_mat2d(mat: Vec<Vertex>, yaw: f32, pitch: f32, roll: f32) -> Vec<Vertex> {
    let mut mat_out: Vec<Vertex> = vec![];
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
        mat_out.push(matrix_to_vertex(
            vertex_to_matrix(mat[i]).dot(&rotation_mat),
        ));
    }

    return mat_out;
}

/// Returns a vector with the ```x```, ```y``` and ```z``` offset the cube has from ```0,0,0```.
fn get_center_of_obj(mat: Vec<Vertex>) -> Vec<f32> {
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut z_sum = 0.0;
    let n_pts = mat.len();

    for i in 0..n_pts {
        x_sum += mat[i].x;
        y_sum += mat[i].y;
        z_sum += mat[i].z;
    }

    return vec![
        x_sum / n_pts as f32,
        y_sum / n_pts as f32,
        z_sum / n_pts as f32,
    ];
}

/// Rotate a 3D object.\
/// ```obj```: A vector of 1D arrays of f32\
/// ```yaw```, ```pitch```, ```roll```: In radians\
/// ```x```, ```y```, ```z``` offset: An offset from\
/// the center of the object, as f32
pub fn rotate_obj(
    obj: Vec<Vertex>,
    yaw: f32,
    pitch: f32,
    roll: f32,
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
) -> Vec<Vertex> {
    let mut mat_out: Vec<Vertex> = vec![];
    // Getting the offset between the cube's center and 0,0,0
    let cube_offset = get_center_of_obj(obj.clone());

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
