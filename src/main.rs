use minifb::{Key, KeyRepeat, MouseButton, Window, WindowOptions};

pub mod assets;

use crate::assets::buttons;
use crate::assets::buttons::Button;

use crate::assets::dd_drawing;
use crate::assets::dd_drawing::draw_line;
use crate::assets::dd_drawing::draw_rect;

use crate::assets::ddd_general::Structure;
use crate::assets::ddd_general::cube;
use crate::assets::ddd_general::project_to_2d;

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

    // Defining 3D objects in the world
    let mut world: Structure = Structure {
        obj_list: vec![
            cube(1.0, [-3.0, -3.0, 5.0]), // start L
            cube(1.0, [-4.0, -3.0, 5.0]),
            cube(1.0, [-5.0, -3.0, 5.0]),
            cube(1.0, [-5.0, -2.0, 5.0]),
            cube(1.0, [-5.0, -1.0, 5.0]),
            cube(1.0, [-5.0, 0.0, 5.0]),
            cube(1.0, [-5.0, 1.0, 5.0]),
            cube(1.0, [-5.0, 2.0, 5.0]),
            cube(1.0, [-1.0, 2.0, 5.0]), // start O
            cube(1.0, [0.0, 2.0, 5.0]),
            cube(1.0, [1.0, 2.0, 5.0]),
            cube(1.0, [-1.0, 1.0, 5.0]),
            cube(1.0, [1.0, 1.0, 5.0]),
            cube(1.0, [-1.0, 0.0, 5.0]),
            cube(1.0, [1.0, 0.0, 5.0]),
            cube(1.0, [-1.0, -1.0, 5.0]),
            cube(1.0, [1.0, -1.0, 5.0]),
            cube(1.0, [-1.0, -2.0, 5.0]),
            cube(1.0, [1.0, -2.0, 5.0]),
            cube(1.0, [-1.0, -3.0, 5.0]),
            cube(1.0, [0.0, -3.0, 5.0]),
            cube(1.0, [1.0, -3.0, 5.0]),
            cube(1.0, [3.0, -3.0, 5.0]), // start L
            cube(1.0, [4.0, -3.0, 5.0]),
            cube(1.0, [5.0, -3.0, 5.0]),
            cube(1.0, [3.0, -2.0, 5.0]),
            cube(1.0, [3.0, -1.0, 5.0]),
            cube(1.0, [3.0, 0.0, 5.0]),
            cube(1.0, [3.0, 1.0, 5.0]),
            cube(1.0, [3.0, 2.0, 5.0]),
        ],
    };

    // let mut world: Structure = Structure {
    //     obj_list: vec![cube(1.0, [0.0, 0.0, 5.0])],
    // };

    // Declaring buttons
    let mut buttons: Vec<Button> = vec![
        Button {
            x_pos: 10.0,
            y_pos: 10.0,
            text: String::from("zoom in"),
            border_width: 3.0,
            text_colour: 0x00888888,
            bg_colour: 0x00222244,
            id: String::from("zoom_in"),
            action: buttons::Action::ZoomIn,
        },
        Button {
            x_pos: 10.0,
            y_pos: 30.0,
            text: String::from("zoom out"),
            border_width: 3.0,
            text_colour: 0x00888888,
            bg_colour: 0x00222244,
            id: String::from("zoom_out"),
            action: buttons::Action::ZoomOut,
        },
        Button {
            x_pos: (WIDTH as i32 - 100) as f32,
            y_pos: 10.0,
            text: String::from("X: 0"),
            border_width: 3.0,
            text_colour: RED,
            bg_colour: 0x00222244,
            id: String::from("x_lable"),
            action: buttons::Action::None,
        },
        Button {
            x_pos: (WIDTH as i32 - 100) as f32,
            y_pos: 30.0,
            text: String::from("Y: 0"),
            border_width: 3.0,
            text_colour: RED,
            bg_colour: 0x00222244,
            id: String::from("y_lable"),
            action: buttons::Action::None,
        },
        Button {
            x_pos: (WIDTH as i32 - 100) as f32,
            y_pos: 50.0,
            text: String::from("Z: 0"),
            border_width: 3.0,
            text_colour: RED,
            bg_colour: 0x00222244,
            id: String::from("z_lable"),
            action: buttons::Action::None,
        },
    ];

    let mut button_pressed = false;

    let mut zoom_level: f32 = 0.0;
    let mut x_move: f32 = 0.0;
    let mut y_move: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // Resets all pixels to black
        }

        let pressed_keys = window.get_keys_pressed(KeyRepeat::No);
        for key in pressed_keys {
            match key {
                Key::Up => y_move += 1.0,
                Key::Down => y_move -= 1.0,
                Key::Right => x_move += 1.0,
                Key::Left => x_move -= 1.0,
                _ => (),
            }
        }

        // Button logic
        {
            if window.get_mouse_down(MouseButton::Left) {
                if button_pressed == false {
                    let button_action: buttons::Action;

                    button_pressed = true;
                    button_action = buttons::button_pressed(&window, &buttons);

                    match button_action {
                        buttons::Action::ZoomIn => zoom_level = -1.0,
                        buttons::Action::ZoomOut => zoom_level = 1.0,
                        buttons::Action::None => print!(""),
                    }
                }
            } else {
                button_pressed = false;
            }
        }

        // Focal length in pixels
        let focal: f32 = 100.0;

        let x_cull_factor: f32 = 3.15152;
        let y_cull_factor: f32 = 1.80606;
        let cull_depth: f32 = 0.01;

        // Rotating the world
        world.rotate(
            3.1415 / 500.0,
            3.1415 / 500.0,
            3.1415 / 500.0,
            0.0,
            0.0,
            0.0,
        );

        // Displaying cube pos
        buttons[2].text = String::from(format!("X: {}", world.obj_list[0].center()[0].to_string()));
        buttons[3].text = String::from(format!("Y: {}", world.obj_list[0].center()[1].to_string()));
        buttons[4].text = String::from(format!("Z: {}", world.obj_list[0].center()[2].to_string()));

        for obj in &mut world.obj_list {
            // Applying zoom and movement to the obj
            {
                for vertex in &mut obj.vertex_data {
                    vertex.z += zoom_level;
                    vertex.x += x_move;
                    vertex.y += y_move;
                }
            }

            let obj_center = obj.center();

            // Culling objects that are off-screen
            if obj_center[2] > cull_depth
                && obj_center[0].abs() < obj_center[2] * x_cull_factor - 1.33333
                && obj_center[1].abs() < obj_center[2] * y_cull_factor - 0.933333
            {
                // // Drawing faces of the obj
                // {
                //     let triangles = obj.surface();

                //     for tri in triangles {
                //         let proj_pt1 = project_to_2d(tri[0].x, tri[0].y, tri[0].z, focal);
                //         let proj_pt2 = project_to_2d(tri[1].x, tri[1].y, tri[1].z, focal);
                //         let proj_pt3 = project_to_2d(tri[2].x, tri[2].y, tri[2].z, focal);

                //         draw_triangle(
                //             &mut buffer,
                //             proj_pt1[0] as i32,
                //             proj_pt1[1] as i32,
                //             proj_pt2[0] as i32,
                //             proj_pt2[1] as i32,
                //             proj_pt3[0] as i32,
                //             proj_pt3[1] as i32,
                //             BLUE,
                //         );
                //     }
                // }

                // Drawing faces of the obj
                {
                    for face in &obj.face_indices {
                        let proj_pt1 = project_to_2d(
                            obj.vertex_data[face[0] as usize].x,
                            obj.vertex_data[face[0] as usize].y,
                            obj.vertex_data[face[0] as usize].z,
                            focal,
                        );
                        let proj_pt2 = project_to_2d(
                            obj.vertex_data[face[1] as usize].x,
                            obj.vertex_data[face[1] as usize].y,
                            obj.vertex_data[face[1] as usize].z,
                            focal,
                        );
                        let proj_pt3 = project_to_2d(
                            obj.vertex_data[face[2] as usize].x,
                            obj.vertex_data[face[2] as usize].y,
                            obj.vertex_data[face[2] as usize].z,
                            focal,
                        );
                        let proj_pt4 = project_to_2d(
                            obj.vertex_data[face[3] as usize].x,
                            obj.vertex_data[face[3] as usize].y,
                            obj.vertex_data[face[3] as usize].z,
                            focal,
                        );

                        draw_rect(
                            &mut buffer,
                            proj_pt1[0] as i32,
                            proj_pt1[1] as i32,
                            proj_pt2[0] as i32,
                            proj_pt2[1] as i32,
                            proj_pt3[0] as i32,
                            proj_pt3[1] as i32,
                            proj_pt4[0] as i32,
                            proj_pt4[1] as i32,
                            BLUE,
                        );
                    }
                }

                // Drawing edges of the obj
                {
                    for edge in &obj.edge_indices {
                        let proj_pt1 = project_to_2d(
                            obj.vertex_data[edge[0] as usize].x,
                            obj.vertex_data[edge[0] as usize].y,
                            obj.vertex_data[edge[0] as usize].z,
                            focal,
                        );
                        let proj_pt2 = project_to_2d(
                            obj.vertex_data[edge[1] as usize].x,
                            obj.vertex_data[edge[1] as usize].y,
                            obj.vertex_data[edge[1] as usize].z,
                            focal,
                        );

                        draw_line(
                            &mut buffer,
                            proj_pt1[0] as i32,
                            proj_pt1[1] as i32,
                            proj_pt2[0] as i32,
                            proj_pt2[1] as i32,
                            GREEN,
                        );
                    }
                }

                // Drawing vertices of the obj
                {
                    for i in 0..obj.vertex_data.len() {
                        // Only project points in front of the camera.
                        if obj.vertex_data[i].z > 0.001 {
                            // Drawing verticies of cube
                            let proj_coords = project_to_2d(
                                obj.vertex_data[i].x,
                                obj.vertex_data[i].y,
                                obj.vertex_data[i].z,
                                focal,
                            );

                            dd_drawing::draw_pixel(
                                &mut buffer,
                                proj_coords[0] as i32,
                                proj_coords[1] as i32,
                                RED,
                            );
                        }
                    }
                }
            }
        }
        zoom_level = 0.0;
        x_move = 0.0;
        y_move = 0.0;

        // Rendering buttons
        {
            let button_buff = buttons::render_buttons(&buttons);

            for i in 0..button_buff.len() {
                if button_buff[i] != 1 {
                    buffer[i] = button_buff[i];
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
