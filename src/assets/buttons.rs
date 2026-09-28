use minifb::{MouseButton, MouseMode, Window};

use crate::HEIGHT;
use crate::WIDTH;

use crate::assets::font;

#[derive(Clone)]
pub enum Action {
    Exit,
    None,
}

#[derive(Clone)]
pub struct Button {
    pub x_pos: f32,
    pub y_pos: f32,
    pub text: String,
    pub border_width: f32,
    pub text_colour: u32,
    pub bg_colour: u32,
    /// Cant be blank
    pub id: String,
    pub action: Action,
}

impl Button {
    pub fn contains(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if mouse_x > self.x_pos - self.border_width
            && mouse_x
                < self.x_pos + (self.text.len() * font::GLYPH_WIDTH - 1) as f32 + self.border_width
            && mouse_y > self.y_pos - self.border_width
            && mouse_y
                < self.y_pos + (self.text.len() * font::GLYPH_WIDTH - 1) as f32 + self.border_width
        {
            return true;
        } else {
            return false;
        }
    }
}

// This must be called repeatedly.
pub fn button_pressed(window: &Window, buttons: &Vec<Button>) -> Action {
    // only runs the code when the mouse is down
    if window.get_mouse_down(MouseButton::Left) {
        let mut mouse_pos: Vec<f32> = vec![0.0, 0.0];

        // gets the mouse position
        window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
            mouse_pos[0] = mouse.0;
            mouse_pos[1] = mouse.1;
        });

        for button in buttons {
            // Detecting button presses
            if button.contains(mouse_pos[0], mouse_pos[1]) {
                return button.action.clone();
            }
        }
    };

    return Action::None;
}

/// Render buttons
pub fn render_buttons(buttons: &Vec<Button>) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![1; WIDTH * HEIGHT];

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
