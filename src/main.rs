use macroquad::prelude::*;

const MAP: [[usize; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

const SIZE_BOCK: f32 = 64.0;

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
};

struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}

#[macroquad::main("Wolf3D Macroquad")]
async fn main() {
    loop {
        clear_background(macroquad::color::Color::from_rgba(224, 248, 207, 255));

        for (x, x_line) in MAP.iter().enumerate() {
            for (y, val) in x_line.iter().enumerate() {
                match val {
                    1 => {
                        let start_x = x as f32 * SIZE_BOCK;
                        let start_y = y as f32 * SIZE_BOCK;

                        draw_rectangle(start_x, start_y, SIZE_BOCK, SIZE_BOCK, GREEN);
                    }
                    0 => {}
                    _ => panic!("wrong number in map"),
                }

                // let v = point_in_wall(x, y);
                // println!("{x},{y}= {v}");
            }
        }
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        next_frame().await
    }
}
