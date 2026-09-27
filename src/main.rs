use minifb::{Key, Window, WindowOptions};

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        let rect = draw_rect(20, 40, 40, 20, BLUE);
        for i in 0..rect.len() {
            buffer[rect[i].0] = rect[i].1;
        }

        let line = draw_line(40, 40, 10, 20, GREEN);
        for i in 0..(line.len()) {
            buffer[line[i].0] = line[i].1;
        }

        let triangle = draw_triangle(150, 50, 200, 150, 50, 170, RED);
        for i in 0..(triangle.len()) {
            buffer[triangle[i].0] = triangle[i].1;
        }

        let line2 = draw_line(199, 148, 199, 130, BLUE);
        for i in 0..(line2.len()) {
            buffer[line2[i].0] = line2[i].1;
        }

        let (index, col) = draw_pixel(150, 50, GREEN);
        buffer[index] = col;

        let (index, col) = draw_pixel(200, 150, GREEN);
        buffer[index] = col;

        let (index, col) = draw_pixel(50, 170, GREEN);
        buffer[index] = col;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

/// Draws a point using a colour.
///
/// # Example usage:
/// ```
/// const BLUE: u32 = 0x000000ff;
///
/// let (index, col) = draw_pixel(20, 20, BLUE);
/// buffer[index] = col;
/// ```
/// This would draw a point at ```20, 20```\
/// with the colour blue.
///
/// # Returns:
/// A tuple of ```(usize, u32)```
/// - **usize**: pos in buffer
/// - **u32**: colour
fn draw_pixel(x: usize, y: usize, col: u32) -> (usize, u32) {
    // println!("{:?}", y);
    return (x + y * WIDTH, col);
}

/// Draws a rectangle between two points using a colour.
///
/// # Example usage:
/// ```
/// const BLUE: u32 = 0x000000ff;
///
/// let rect = draw_rect(20, 20, 40, 40, BLUE);
/// for i in 0..rect.len() {
///     buffer[rect[i].0] = rect[i].1;
/// }
/// ```
/// This would draw a rectangle between points\
/// ```20, 20``` and ```40, 40``` with the colour blue.
///
/// # Returns:
/// A vector of tuple of ```(usize, u32)```
/// - **usize**: pos in buffer
/// - **u32**: colour
fn draw_rect(x1: usize, y1: usize, x2: usize, y2: usize, col: u32) -> Vec<(usize, u32)> {
    let mut v_out: Vec<(usize, u32)> =
        vec![(0 as usize, 0); (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)];

    for x in 0..(x1.abs_diff(x2) + 1) {
        for y in 0..(y1.abs_diff(y2) + 1) {
            v_out[x + y * (x1.abs_diff(x2) + 1)] = draw_pixel(
                x + minimumi32(x1 as i32, x2 as i32) as usize,
                y + minimumi32(y1 as i32, y2 as i32) as usize,
                col,
            )
        }
    }
    return v_out;
}

/// Draws a line between two points using a colour.
///
/// # Example usage:
/// ```
/// const BLUE: u32 = 0x000000ff;
///
/// let line = draw_line(40, 20, 20, 40, BLUE);
/// for i in 0..(line.len()) {
///     buffer[line[i].0] = line[i].1;
/// }
/// ```
/// This would draw a line between points\
/// ```40, 20``` and ```20, 40``` with the colour blue.
///
/// # Returns:
/// A vector of tuple of ```(usize, u32)```
/// - **usize**: pos in buffer
/// - **u32**: colour
fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, col: u32) -> Vec<(usize, u32)> {
    let mut v_out: Vec<(usize, u32)> = vec![(0 as usize, 0); x1.abs_diff(x2) + 1];

    for x in 0..x1.abs_diff(x2) + 1 {
        let mut y = (((y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32)) * x as f32).round() as i32;

        // println!("{:?}", (y + minimumi32(y1 as i32, y2 as i32)));

        if (y + minimumi32(y1 as i32, y2 as i32)) < 0 {
            y = y + (y + minimumi32(y1 as i32, y2 as i32)).abs();
        }

        v_out[x] = draw_pixel(
            (x as i32 + minimumi32(x1 as i32, x2 as i32)) as usize,
            (y + minimumi32(y1 as i32, y2 as i32)) as usize,
            col,
        )
    }
    return v_out;
}

/// Returns the smaller of two values.
fn minimumi32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

/// Returns the larger of two values.
fn maximumi32(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

/// Gets the ```x``` and ```y``` pos that a buffer position is referencing
fn get_xy_from_buffer_pos(buff: usize, w: usize) -> Vec<usize> {
    let y = (buff as f32 / w as f32).floor() as usize;
    let x = (buff as i32 - y as i32 * w as i32) as usize;
    return vec![x, y];
}

fn get_line_xy_pts(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<Vec<usize>> {
    let line = draw_line(x1, y1, x2, y2, 1);
    let mut line_out: Vec<Vec<usize>> = vec![vec![0, 0]; line.len()];

    for i in 0..line.len() {
        line_out[i] = get_xy_from_buffer_pos(line[i].0, WIDTH);
    }

    return line_out;
}

/// Draws a triangle between three points using a colour.
///
/// # Example usage:
/// ```
/// const BLUE: u32 = 0x000000ff;
///
/// ```
/// This would draw a triangle between points\
/// ```20, 20``` and ```40, 40``` with the colour blue.
///
/// # Returns:
/// A vector of tuple of ```(usize, u32)```
/// - **usize**: pos in buffer
/// - **u32**: colour
fn draw_triangle(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    x3: usize,
    y3: usize,
    col: u32,
) -> Vec<(usize, u32)> {
    let mut v_out: Vec<(usize, u32)> = vec![];

    let x_min = minimumi32(minimumi32(x1 as i32, x2 as i32), x3 as i32);
    let x_max = maximumi32(maximumi32(x1 as i32, x2 as i32), x3 as i32);

    let line_12 = get_line_xy_pts(x1, y1, x2, y2);
    let line_23 = get_line_xy_pts(x2, y2, x3, y3);
    let line_31 = get_line_xy_pts(x3, y3, x1, y1);

    let mut outline_pts: Vec<Vec<usize>> = vec![];

    outline_pts.extend(line_12.iter().cloned());
    outline_pts.extend(line_23.iter().cloned());
    outline_pts.extend(line_31.iter().cloned());

    for x in 0..x_max.abs_diff(x_min) {
        let ext_pts = find_matching_x_from_2d_arr(outline_pts.clone(), x as usize + x_min as usize);
        if ext_pts.len() > 1 {
            let scan_line = draw_line(
                ext_pts[0][0],
                ext_pts[0][1],
                ext_pts[1][0],
                ext_pts[1][1],
                col,
            );
            println!("{:?}", ext_pts);
            println!("##################");
            // println!("{:?}", ext_pts);
            v_out.extend(scan_line.iter().cloned());
        }
    }
    return v_out;
}

/// Finds points that have an x-value that matches what is being searched for.
///
/// # Example:
/// ```
/// let x_pts = find_matching_x_from_2d_arr(
/// vec![
///     vec![10, 0],
///     vec![11, 33],
///     vec![10, 50],
///     vec![30, 5],
///     vec![20, 0],
/// ],
/// 10,
/// );
///
/// println!("{:?}", x_pts);
/// ```
///
/// This would print out ```[[10, 0], [10, 50]]```.
fn find_matching_x_from_2d_arr(pts: Vec<Vec<usize>>, x: usize) -> Vec<Vec<usize>> {
    let mut v_out: Vec<Vec<usize>> = vec![];
    for i in 0..pts.len() {
        if pts[i][0] == x {
            let current_pt = pts[i].clone();
            v_out.push(current_pt);
        }
    }
    return v_out;
}
