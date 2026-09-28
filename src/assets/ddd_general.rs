#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// ```1``` for true, ```0```  for false
    pub is_pt: f32,
}

pub struct Rectangle3D {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub center_pos: Vec<f32>,
    pub vertex_data: Vec<Vertex>,
}

pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    return Vertex {
        x: x,
        y: y,
        z: z,
        is_pt: 1.0,
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

    return Rectangle3D {
        width,
        height,
        depth,
        yaw: 0.0,
        pitch: 0.0,
        roll: 0.0,
        center_pos,
        vertex_data: vec![v_1, v_2, v_3, v_4, v_5, v_6, v_7, v_8],
    };
}
