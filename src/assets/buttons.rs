use crate::HEIGHT;
use crate::WIDTH;
use crate::assets::font;

#[derive(Clone)]
pub struct Button {
    pub x_pos: f32,
    pub y_pos: f32,
    pub text: String,
    pub border_width: f32,
    pub text_colour: u32,
    pub bg_colour: u32,
}

// fn main() {
//     // Declaring buttons
//     let buttons: Vec<Button> = vec![Button {
//         x_pos: 10.0,
//         y_pos: 10.0,
//         text: String::from("Hello World."),
//         border_width: 3.0,
//         text_colour: 0x00888888,
//         bg_colour: 0x00222244,
//     }];

//     let mut button_pressed: bool = false;
//     let mut mouse_pos: Vec<f32> = vec![0.0, 0.0];

//     // gets the mouse position
//     window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
//         mouse_pos[0] = mouse.0 as f32;
//         mouse_pos[1] = mouse.1 as f32;
//     });

//     // only runs the code when the mouse is down
//     if window.get_mouse_down(MouseButton::Left) {
//         // Detecting button presses on buttons, should be moved into a loop in the future
//         if is_pt_in_rect(
//             mouse_pos.clone(),
//             vec![buttons[0].0.clone(), buttons[0].1.clone()],
//             vec![
//                 (buttons[0].2.len() * font::GLYPH_WIDTH - 1) as f32,
//                 font::GLYPH_HEIGHT as f32,
//             ],
//         ) && (circle_pos_arr.len() + 1 <= 30)
//         {
//             if button_pressed == false {
//                 let mut new_pt: Vec<Vec<f32>> = vec![vec![10.0, 10.0]];
//                 circle_pos_arr.append(&mut new_pt);
//                 button_pressed = true;
//             }
//         }
//     } else {
//         button_pressed = false;
//     };
// }

fn is_pt_in_rect(pt: Vec<f32>, rect_pos: Vec<f32>, rect_dims: Vec<f32>) -> bool {
    for x in rect_pos[0] as i32..(rect_dims[0] + rect_pos[0]) as i32 {
        for y in rect_pos[1] as i32..(rect_dims[1] + rect_pos[1]) as i32 {
            if pt == vec![x as f32, y as f32] {
                return true;
            }
        }
    }
    return false;
}

pub fn render_buttons(buttons: &Vec<Button>) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![1; WIDTH * HEIGHT];

    // Render buttons
    for button in buttons {
        let text = button.text.clone();
        let mut text_x_offset = button.x_pos.clone() as usize;
        let text_y_offset = button.y_pos.clone() as usize;

        for x in (-1 * button.border_width as i32)
            ..((button.text.len() * font::GLYPH_WIDTH) as i32 + button.border_width as i32)
        {
            for y in (-1 * button.border_width as i32)
                ..(font::GLYPH_HEIGHT as i32 + button.border_width as i32)
            {
                if (x + text_x_offset as i32) < WIDTH as i32
                    && (y + text_y_offset as i32) < HEIGHT as i32
                {
                    let index =
                        ((y + text_y_offset as i32) * WIDTH as i32) + x + text_x_offset as i32;
                    buffer[index as usize] = button.bg_colour;
                }
            }
        }

        for c in text.chars() {
            let letter: [[i32; 7]; 9] = font::get_letter_glyph(c);

            for y in 0..letter.len() {
                for x in 0..letter[y].len() {
                    if letter[y][x] == 1 {
                        if x + text_x_offset < WIDTH && y + text_y_offset < HEIGHT {
                            let index = ((y + text_y_offset) * WIDTH) + x + text_x_offset;
                            buffer[index] = button.text_colour;
                        }
                    }
                }
            }
            text_x_offset += font::GLYPH_WIDTH;
        }
    }

    return buffer;
}
