use crate::assets::matrix::rotate_mat2d;
use crate::assets::matrix::translate_mat2d;
use crate::{HEIGHT, WIDTH};

use std::collections::HashSet;

use parry3d::math::Vec3;
use parry3d::transformation::convex_hull;

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

impl Obj3D {
    /// Returns a vector with the ```x```, ```y``` and ```z``` offset the cube has from ```0,0,0```.
    pub fn center(&self) -> [f32; 3] {
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut z_sum = 0.0;
        let n_pts = self.vertex_data.len();

        for i in 0..n_pts {
            x_sum += self.vertex_data[i].x;
            y_sum += self.vertex_data[i].y;
            z_sum += self.vertex_data[i].z;
        }

        return [
            x_sum / n_pts as f32,
            y_sum / n_pts as f32,
            z_sum / n_pts as f32,
        ];
    }

    /// Rotate an object.\
    /// ```obj```: A vector of 1D arrays of f32\
    /// ```yaw```, ```pitch```, ```roll```: In radians\
    /// ```x```, ```y```, ```z``` offset: An offset from\
    /// the center of the object, as f32
    pub fn rotate(
        &mut self,
        yaw: f32,
        pitch: f32,
        roll: f32,
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
    ) {
        // Getting the offset between the cube's center and 0,0,0
        let offset = self.center();

        // Moving cube's center to be 0,0,0
        self.vertex_data = translate_mat2d(
            &self.vertex_data,
            -offset[0] + x_offset,
            -offset[1] + y_offset,
            -offset[2] + z_offset,
        );

        // Rotating cube
        self.vertex_data = rotate_mat2d(&self.vertex_data, yaw, pitch, roll);

        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;

        // Moving the cube's center away from 0,0,0
        self.vertex_data = translate_mat2d(
            &self.vertex_data,
            offset[0] - x_offset,
            offset[1] - y_offset,
            offset[2] - z_offset,
        );
    }

    /// Returns all the triangles that make up the surface of an object
    pub fn surface(&self) -> Vec<Vec<Vertex>> {
        let mut obj_triangles: Vec<Vec<Vertex>> = vec![];
        let hull = build_hull(&self);

        for triangle_index in hull.1 {
            obj_triangles.push(vec![
                vertex(
                    hull.0[triangle_index[0] as usize][0],
                    hull.0[triangle_index[0] as usize][1],
                    hull.0[triangle_index[0] as usize][2],
                ),
                vertex(
                    hull.0[triangle_index[1] as usize][0],
                    hull.0[triangle_index[1] as usize][1],
                    hull.0[triangle_index[1] as usize][2],
                ),
                vertex(
                    hull.0[triangle_index[2] as usize][0],
                    hull.0[triangle_index[2] as usize][1],
                    hull.0[triangle_index[2] as usize][2],
                ),
            ])
        }

        return obj_triangles;
    }

    /// Returns all the edges of an object
    pub fn edges(&self) -> Vec<Vec<Vertex>> {
        let (pts, tris) = build_hull(self);

        // Key on the unordered vertex-index pair. `(min, max)` canonicalises
        // so (a,b) and (b,a) map to the same key.
        let mut seen: HashSet<(u32, u32)> = HashSet::new();
        let mut obj_edges: Vec<Vec<Vertex>> = Vec::new();

        for tri in &tris {
            for i in 0..3 {
                let a = tri[i];
                let b = tri[(i + 1) % 3];
                let key = if a < b { (a, b) } else { (b, a) };

                if !seen.insert(key) {
                    continue; // edge already emitted by a neighbour triangle
                }

                obj_edges.push(vec![
                    vertex_from_vec3(&pts[a as usize]),
                    vertex_from_vec3(&pts[b as usize]),
                ]);
            }
        }

        return obj_edges;
    }
}

pub struct Rectangle3D {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub obj: Obj3D,
    pub center_pos: Vec<f32>,
}

pub struct Structure {
    pub obj_list: Vec<Obj3D>,
}

impl Structure {
    /// Returns a vector with the ```x```, ```y``` and ```z``` offset the structure has from ```0,0,0```.
    pub fn center(&self) -> [f32; 3] {
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut z_sum = 0.0;
        let mut n_pts = 0;

        for obj in &self.obj_list {
            n_pts += obj.vertex_data.len();
            for i in 0..obj.vertex_data.len() {
                x_sum += obj.vertex_data[i].x;
                y_sum += obj.vertex_data[i].y;
                z_sum += obj.vertex_data[i].z;
            }
        }

        return [
            x_sum / n_pts as f32,
            y_sum / n_pts as f32,
            z_sum / n_pts as f32,
        ];
    }

    /// Rotate an object.\
    /// ```obj```: A vector of 1D arrays of f32\
    /// ```yaw```, ```pitch```, ```roll```: In radians\
    /// ```x```, ```y```, ```z``` offset: An offset from\
    /// the center of the object, as f32
    pub fn rotate(
        &mut self,
        yaw: f32,
        pitch: f32,
        roll: f32,
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
    ) {
        let structure_center = self.center();

        for obj in &mut self.obj_list {
            // Getting the offset between the cube's center and 0,0,0
            let offset = obj.center();

            obj.rotate(
                yaw,
                pitch,
                roll,
                offset[0] - structure_center[0] + x_offset,
                offset[1] - structure_center[1] + y_offset,
                offset[2] - structure_center[2] + z_offset,
            );
        }
    }
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

pub fn rectangle_3d(width: f32, height: f32, depth: f32, center_pos: [f32; 3]) -> Rectangle3D {
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
        center_pos: center_pos.to_vec(),
    };
}

pub fn cube(size: f32, center_pos: [f32; 3]) -> Obj3D {
    let v_1 = vertex(
        (size / 2.0) + center_pos[0],
        (size / 2.0) + center_pos[1],
        (size / 2.0) + center_pos[2],
    );
    let v_2 = vertex(
        (size / 2.0) + center_pos[0],
        (size / 2.0) + center_pos[1],
        (-size / 2.0) + center_pos[2],
    );
    let v_3 = vertex(
        (-size / 2.0) + center_pos[0],
        (size / 2.0) + center_pos[1],
        (-size / 2.0) + center_pos[2],
    );
    let v_4 = vertex(
        (-size / 2.0) + center_pos[0],
        (size / 2.0) + center_pos[1],
        (size / 2.0) + center_pos[2],
    );
    let v_5 = vertex(
        (size / 2.0) + center_pos[0],
        (-size / 2.0) + center_pos[1],
        (size / 2.0) + center_pos[2],
    );
    let v_6 = vertex(
        (size / 2.0) + center_pos[0],
        (-size / 2.0) + center_pos[1],
        (-size / 2.0) + center_pos[2],
    );
    let v_7 = vertex(
        (-size / 2.0) + center_pos[0],
        (-size / 2.0) + center_pos[1],
        (-size / 2.0) + center_pos[2],
    );
    let v_8 = vertex(
        (-size / 2.0) + center_pos[0],
        (-size / 2.0) + center_pos[1],
        (size / 2.0) + center_pos[2],
    );

    return build_obj(vec![v_1, v_2, v_3, v_4, v_5, v_6, v_7, v_8], 0.0, 0.0, 0.0);
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
    let cube_offset = obj.center();

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

fn polar_to_cartesian_3d(rho: f32, az: f32, alt: f32) -> Vec<f32> {
    let x = rho * alt.sin() * az.cos();
    let y = rho * alt.sin() * az.sin();
    let z = rho * alt.cos();

    return vec![x, y, z];
}

/// Creates a convex hull of an Obj3D using quickhull
fn build_hull(obj: &Obj3D) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    // Drop the homogeneous w component; hull operates on 3D points.
    let pts: Vec<Vec3> = obj
        .vertex_data
        .iter()
        .map(|v| Vec3::new(v.x, v.y, v.z))
        .collect();

    return convex_hull(&pts);
}

fn vertex_from_vec3(p: &Vec3) -> Vertex {
    vertex(p[0], p[1], p[2])
}

/// Projects a 3D point onto a 2D plane, with a focal distance.
pub fn project_to_2d(x: f32, y: f32, z: f32, focal: f32) -> Vec<f32> {
    let inv_z = focal / z; // perspective divide

    let x_screen = WIDTH as f32 / 2.0 + x * inv_z; // flip x
    let y_screen = HEIGHT as f32 / 2.0 - y * inv_z; // flip y

    return vec![x_screen, y_screen];
}
