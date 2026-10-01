use minifb::{Key, MouseButton, Window, WindowOptions};

pub mod assets;

use crate::assets::buttons;
use crate::assets::dd_drawing;
use crate::assets::dd_drawing::draw_line;
use crate::assets::dd_drawing::draw_triangle;
use crate::assets::ddd_general::Structure;
use crate::assets::ddd_general::cube;
use crate::assets::ddd_general::project_to_2d;

use crate::assets::buttons::Button;

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

    // Defining 3D objects
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

    // Declaring buttons
    let buttons: Vec<Button> = vec![
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
    ];

    let mut button_pressed = false;

    let mut zoom_level: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // Resets all pixels to black
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

        // world.rotate(
        //     3.1415 / 250.0 * 22.0,
        //     3.1415 / 250.0 * 22.0,
        //     3.1415 / 250.0 * 22.0,
        //     0.0,
        //     0.0,
        //     0.0,
        // );
        world.rotate(
            3.1415 / 500.0,
            3.1415 / 500.0,
            3.1415 / 500.0,
            0.0,
            0.0,
            0.0,
        );
        for obj in &mut world.obj_list {
            {
                for vertex in &mut obj.vertex_data {
                    vertex.z += zoom_level;
                }
            }
            {
                // Drawing faces of cube
                let triangles = obj.surface();

                for tri in triangles {
                    let proj_pt1 = project_to_2d(tri[0].x, tri[0].y, tri[0].z, focal);
                    let proj_pt2 = project_to_2d(tri[1].x, tri[1].y, tri[1].z, focal);
                    let proj_pt3 = project_to_2d(tri[2].x, tri[2].y, tri[2].z, focal);

                    draw_triangle(
                        &mut buffer,
                        proj_pt1[0] as i32,
                        proj_pt1[1] as i32,
                        proj_pt2[0] as i32,
                        proj_pt2[1] as i32,
                        proj_pt3[0] as i32,
                        proj_pt3[1] as i32,
                        BLUE,
                    );
                }
            }

            {
                // Drawing edges of the cube
                let obj_edges = obj.edges();
                for edge in obj_edges {
                    let proj_pt1 = project_to_2d(edge[0].x, edge[0].y, edge[0].z, focal);
                    let proj_pt2 = project_to_2d(edge[1].x, edge[1].y, edge[1].z, focal);

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

            {
                // Drawing vertices of the cube
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
        zoom_level = 0.0;

        // rendering buttons
        let button_buff = buttons::render_buttons(&buttons);

        for i in 0..button_buff.len() {
            if button_buff[i] != 1 {
                buffer[i] = button_buff[i];
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
