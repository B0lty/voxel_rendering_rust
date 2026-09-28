use ndarray::{Array1, arr1, arr2};

/// Translate a 2D matix using ```tx```, ```ty``` and ```tz```.
pub fn translate_mat2d(mat: Vec<Array1<f32>>, tx: f32, ty: f32, tz: f32) -> Vec<Array1<f32>> {
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

/// Returns a vector with the ```x```, ```y``` and ```z``` offset the cube has from ```0,0,0```.
fn get_center_of_obj(mat: Vec<Array1<f32>>) -> Vec<f32> {
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

/// Rotate a 3D object.\
/// ```obj```: A vector of 1D arrays of f32\
/// ```yaw```, ```pitch```, ```roll```: In radians\
/// ```x```, ```y```, ```z``` offset: An offset from\
/// the center of the object, as f32
pub fn rotate_obj(
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
