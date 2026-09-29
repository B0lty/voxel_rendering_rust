use crate::WIDTH;

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
pub fn draw_pixel(x: usize, y: usize, col: u32) -> (usize, u32) {
    return (x + y * WIDTH, col);
}

/// Draws a rectangle between four points using a colour.
///
/// # Example usage:
/// ```
/// const BLUE: u32 = 0x000000ff;
///
/// let rect = draw_rect(20, 20, 20, 40, 40, 40, 40, 20, BLUE);
/// for i in 0..rect.len() {
///     buffer[rect[i].0] = rect[i].1;
/// }
/// ```
/// This would draw a rectangle between points\
/// ```20, 20```, ```20, 40```, ```40, 40``` and ```40, 20``` with the colour blue.
///
/// # Returns:
/// A vector of tuple of ```(usize, u32)```
/// - **usize**: pos in buffer
/// - **u32**: colour
pub fn draw_rect(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    x3: usize,
    y3: usize,
    x4: usize,
    y4: usize,
    col: u32,
) -> Vec<(usize, u32)> {
    let mut tri_1 = draw_triangle(x1, y1, x2, y2, x3, y3, col);
    let tri_2 = draw_triangle(x1, y1, x3, y3, x4, y4, col);

    tri_1.extend(tri_2.iter().cloned());

    return tri_1;
}

/// Draws a line between two points using a colour. This\
/// is an implementation of Bresenham's line algorithm.
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
pub fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, col: u32) -> Vec<(usize, u32)> {
    let mut v_out: Vec<(usize, u32)> = Vec::new();

    let (x1, y1, x2, y2) = (x1 as i32, y1 as i32, x2 as i32, y2 as i32);

    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    let (mut x, mut y) = (x1, y1);

    loop {
        v_out.push(draw_pixel(x as usize, y as usize, col));

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }

    return v_out;
}

/// Returns the smaller of two values.
pub fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

/// Returns the larger of two values.
pub fn max_i32(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

/// Gets the ```x``` and ```y``` pos that a buffer position is referencing.
fn get_xy_from_buffer_pos(buff: usize) -> Vec<usize> {
    let y = (buff as f32 / WIDTH as f32).floor() as usize;
    let x = (buff as i32 - y as i32 * WIDTH as i32) as usize;
    return vec![x, y];
}

/// Returns a vector of vectors of ```x, y``` points that form a line between two points. Used for separate\
/// computation, not direct drawing.
fn get_line_xy_pts(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<Vec<usize>> {
    let line = draw_line(x1, y1, x2, y2, 1);
    let mut line_out: Vec<Vec<usize>> = vec![vec![0, 0]; line.len()];

    for i in 0..line.len() {
        line_out[i] = get_xy_from_buffer_pos(line[i].0);
    }

    return line_out;
}

/// Draws a triangle between three points using a colour.
///
/// # Example usage:
/// ```
/// const BLUE: u32 = 0x000000ff;
///
/// let triangle = draw_triangle(150, 50, 200, 150, 50, 170, BLUE);
/// for i in 0..(triangle.len()) {
///     buffer[triangle[i].0] = triangle[i].1;
/// }
/// ```
/// This would draw a triangle between points\
/// ```20, 20```, ```40, 40``` and ```30, 30``` with the colour blue.
///
/// # Returns:
/// A vector of tuple of ```(usize, u32)```
/// - **usize**: pos in buffer
/// - **u32**: colour
pub fn draw_triangle(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    x3: usize,
    y3: usize,
    col: u32,
) -> Vec<(usize, u32)> {
    let mut v_out: Vec<(usize, u32)> = vec![];

    let x_min = min_i32(min_i32(x1 as i32, x2 as i32), x3 as i32);
    let x_max = max_i32(max_i32(x1 as i32, x2 as i32), x3 as i32);
    let x_range = x_min.abs_diff(x_max);

    let line_12 = get_line_xy_pts(x1, y1, x2, y2);
    let line_23 = get_line_xy_pts(x2, y2, x3, y3);
    let line_31 = get_line_xy_pts(x3, y3, x1, y1);

    let mut outline_pts: Vec<Vec<usize>> = vec![];

    outline_pts.extend(line_12.iter().cloned());
    outline_pts.extend(line_23.iter().cloned());
    outline_pts.extend(line_31.iter().cloned());

    for x in 0..x_range {
        let ext_pts = find_matching_x_from_2d_arr(outline_pts.clone(), x as usize + x_min as usize);
        if ext_pts.len() > 1 {
            let mut y_min: usize = ext_pts[0][1];
            let mut y_max: usize = ext_pts[0][1];
            let mut y_min_index = 0;
            let mut y_max_index = 0;

            for i in 0..ext_pts.len() {
                if ext_pts[i][1] < y_min {
                    y_min_index = i;
                    y_min = ext_pts[i][1];
                };
                if ext_pts[i][1] > y_max {
                    y_max_index = i;
                    y_max = ext_pts[i][1];
                };
            }

            let scan_line = draw_line(
                ext_pts[y_min_index][0],
                ext_pts[y_min_index][1],
                ext_pts[y_max_index][0],
                ext_pts[y_max_index][1],
                col,
            );
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
