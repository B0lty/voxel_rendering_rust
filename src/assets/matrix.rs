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
pub fn translate_mat2d(mat: &Vec<Vertex>, tx: f32, ty: f32, tz: f32) -> Vec<Vertex> {
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
pub fn rotate_mat2d(mat: &Vec<Vertex>, yaw: f32, pitch: f32, roll: f32) -> Vec<Vertex> {
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
