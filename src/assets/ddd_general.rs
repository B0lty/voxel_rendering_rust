use crate::assets::general::merge_sort;
use crate::assets::matrix::rotate_mat2d;
use crate::assets::matrix::translate_mat2d;

use crate::assets::general::ValueWithBaggage;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// ```1``` for point, ```0```  for direction
    pub is_pt: f32,
}

pub struct Obj3D {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub vertex_data: Vec<Vertex>,
}

pub struct Rectangle3D {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub obj: Obj3D,
    pub center_pos: Vec<f32>,
}

pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    return Vertex {
        x: x,
        y: y,
        z: z,
        is_pt: 1.0,
    };
}

fn build_obj(vertex_data: Vec<Vertex>, roll: f32, pitch: f32, yaw: f32) -> Obj3D {
    return Obj3D {
        yaw,
        pitch,
        roll,
        vertex_data,
    };
}

pub fn rectangle_3d(width: f32, height: f32, depth: f32, center_pos: Vec<f32>) -> Rectangle3D {
    let v_1 = vertex(
        (width / 2.0) + center_pos[0],
        (height / 2.0) + center_pos[1],
        (depth / 2.0) + center_pos[2],
    );
    let v_2 = vertex(
        (width / 2.0) + center_pos[0],
        (height / 2.0) + center_pos[1],
        (-depth / 2.0) + center_pos[2],
    );
    let v_3 = vertex(
        (-width / 2.0) + center_pos[0],
        (height / 2.0) + center_pos[1],
        (-depth / 2.0) + center_pos[2],
    );
    let v_4 = vertex(
        (-width / 2.0) + center_pos[0],
        (height / 2.0) + center_pos[1],
        (depth / 2.0) + center_pos[2],
    );
    let v_5 = vertex(
        (width / 2.0) + center_pos[0],
        (-height / 2.0) + center_pos[1],
        (depth / 2.0) + center_pos[2],
    );
    let v_6 = vertex(
        (width / 2.0) + center_pos[0],
        (-height / 2.0) + center_pos[1],
        (-depth / 2.0) + center_pos[2],
    );
    let v_7 = vertex(
        (-width / 2.0) + center_pos[0],
        (-height / 2.0) + center_pos[1],
        (-depth / 2.0) + center_pos[2],
    );
    let v_8 = vertex(
        (-width / 2.0) + center_pos[0],
        (-height / 2.0) + center_pos[1],
        (depth / 2.0) + center_pos[2],
    );

    let obj = build_obj(vec![v_1, v_2, v_3, v_4, v_5, v_6, v_7, v_8], 0.0, 0.0, 0.0);

    return Rectangle3D {
        width,
        height,
        depth,
        obj,
        center_pos,
    };
}

/// Returns a vector with the ```x```, ```y``` and ```z``` offset the cube has from ```0,0,0```.
fn get_center_of_obj(obj: &Obj3D) -> Vec<f32> {
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut z_sum = 0.0;
    let n_pts = obj.vertex_data.len();

    for i in 0..n_pts {
        x_sum += obj.vertex_data[i].x;
        y_sum += obj.vertex_data[i].y;
        z_sum += obj.vertex_data[i].z;
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
    obj: &Obj3D,
    yaw: f32,
    pitch: f32,
    roll: f32,
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
) -> Obj3D {
    let mut obj_out: Obj3D;
    // Getting the offset between the cube's center and 0,0,0
    let cube_offset = get_center_of_obj(&obj);

    // Moving cube's center to be 0,0,0
    let obj_center = build_obj(
        translate_mat2d(
            &obj.vertex_data,
            -cube_offset[0] + x_offset,
            -cube_offset[1] + y_offset,
            -cube_offset[2] + z_offset,
        ),
        0.0,
        0.0,
        0.0,
    );

    // Rotating cube
    obj_out = build_obj(
        rotate_mat2d(&obj_center.vertex_data, yaw, pitch, roll),
        yaw,
        pitch,
        roll,
    );

    // Moving the cube's center away from 0,0,0
    obj_out.vertex_data = translate_mat2d(
        &obj_out.vertex_data,
        cube_offset[0] - x_offset,
        cube_offset[1] - y_offset,
        cube_offset[2] - z_offset,
    );

    return obj_out;
}

/// Converts cartesian co-ordinates to polar co-ordinates.\
///
/// # Returns:
/// ```result[0]```: rho, distance from ```0,0,0```, f32\
/// ```result[1]```: azimuth, yaw, f32\
/// ```result[2]```: altitude, pitch, f32
fn cartesian_to_polar_3d(x: f32, y: f32, z: f32) -> Vec<f32> {
    let rho = (x * x + y * y + z * z).sqrt();
    let azimuth = y.atan2(x);
    let altitude = (z / rho).acos();

    return vec![rho, azimuth, altitude];
}

/// Returns all the edges of an object
pub fn get_obj_edges(obj: &Obj3D) -> Vec<Vec<Vertex>> {
    //                                     x    y    z  weight
    let mut verticies_weights_polar: Vec<ValueWithBaggage> = vec![];
    let mut obj_edges: Vec<Vec<Vertex>> = vec![];
    let obj_center = get_center_of_obj(&obj);

    for vertex in obj.vertex_data.clone() {
        let polar_coord = cartesian_to_polar_3d(
            vertex.x - obj_center[0],
            vertex.y - obj_center[1],
            vertex.z - obj_center[2],
        );

        let weight_and_pos: ValueWithBaggage = ValueWithBaggage {
            value: polar_coord[1] + polar_coord[2],
            baggage: vec![vertex.x, vertex.y, vertex.z],
        };

        verticies_weights_polar.push(weight_and_pos);
    }

    let sorted = merge_sort(verticies_weights_polar);

    for i in 0..(sorted.len() - 1) {
        obj_edges.push(vec![
            vertex(
                sorted[i].baggage[0],
                sorted[i].baggage[1],
                sorted[i].baggage[2],
            ),
            vertex(
                sorted[i + 1].baggage[0],
                sorted[i + 1].baggage[1],
                sorted[i + 1].baggage[2],
            ),
        ])
    }

    return obj_edges;
}

fn get_triangales_of_obj(obj: &Obj3D) -> Vec<Vec<Vertex>> {
    let obj_triangles: Vec<Vec<Vertex>> = vec![];
    let obj_edges = get_obj_edges(&obj);

    return obj_triangles;
}
