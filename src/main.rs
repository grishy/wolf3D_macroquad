use macroquad::prelude::*;

// more info: https://grantshandy.github.io/posts/raycasting/
// code reference: https://github.com/grantshandy/wasm4-raycaster/blob/main/src/lib.rs

const MAP_BIN: [u32; 32] = [
    0b111111111111111111111111111111,
    0b100000000001001001001000000001,
    0b100000000001001001000100000001,
    0b100000000001001001000010000001,
    0b100000000001001001000001000001,
    0b100000100001001001000000000001,
    0b100000100001001001000000010001,
    0b100000100001001001000000001001,
    0b100000100000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000000000011100000001,
    0b100000000000000000010100000001,
    0b100000000000000000011100000001,
    0b100000000000000000000000000001,
    0b100000000000000000000000000001,
    0b100000000000000000000000000001,
    0b100000000000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000000000000000000001,
    0b100000100000010000000011110001,
    0b100000000000111000000010000001,
    0b100000000000010000000010000001,
    0b100000000000000000000010001001,
    0b100000000000000000000010000001,
    0b100000000000000000000010000001,
    0b100000000000000000000010000001,
    0b100000000000000000000010000001,
    0b111111111111111111111111111111,
];

const MAP: [[usize; 30]; 30] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

const SIZE_BOCK: f32 = 24.0;
const SIZE_PLAYER: f32 = 12.0;
const PLAYER_SPEED: f32 = 0.1;
const focalLength: f32 = 0.8;

struct State {
    player_x: f32,
    player_y: f32,
    player_angle_radians: f32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Wolf3D Macroquad".to_owned(),
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state: State = State {
        player_x: 2.2,
        player_y: 2.2,
        player_angle_radians: 0.0,
    };

    loop {
        clear_background(macroquad::color::Color::from_rgba(224, 248, 207, 255));

        for (y, y_line) in MAP.iter().enumerate() {
            for (x, val) in y_line.iter().enumerate() {
                match val {
                    1 => {
                        let start_x = x as f32 * SIZE_BOCK;
                        let start_y = y as f32 * SIZE_BOCK;

                        draw_rectangle(
                            start_x,
                            start_y,
                            SIZE_BOCK,
                            SIZE_BOCK,
                            macroquad::color::Color::from_rgba(48, 104, 80, 255),
                        );
                    }
                    0 => {}
                    _ => panic!("wrong number in map"),
                }
            }
        }

        {
            let num_ray = macroquad::window::screen_width() as i32;
            // let num_ray = 20;

            for column in 0..=num_ray {
                // -0.5 < x < 0.5
                let x = column as f32 / num_ray as f32 - 0.5;
                let angle = game_state.player_angle_radians + x.atan2(focalLength);
                let length = 40.0;

                let (ray_x, ray_y) = castRay(&mut game_state, angle, length);

                draw_line(
                    game_state.player_x * SIZE_BOCK,
                    game_state.player_y * SIZE_BOCK,
                    (ray_x) * SIZE_BOCK,
                    (ray_y) * SIZE_BOCK,
                    1.0,
                    RED,
                );
            }
        }

        {
            let length = 3.0;
            let dx = length * game_state.player_angle_radians.cos();
            let dy = length * game_state.player_angle_radians.sin();

            draw_line(
                game_state.player_x * SIZE_BOCK,
                game_state.player_y * SIZE_BOCK,
                (game_state.player_x + dx) * SIZE_BOCK,
                (game_state.player_y + dy) * SIZE_BOCK,
                2.0,
                BLUE,
            );
        }

        draw_circle(
            game_state.player_x * SIZE_BOCK,
            game_state.player_y * SIZE_BOCK,
            SIZE_PLAYER,
            YELLOW,
        );

        {
            if is_key_down(KeyCode::Up) {
                walk(&mut game_state, PLAYER_SPEED);
            }
            if is_key_down(KeyCode::Down) {
                walk(&mut game_state, -PLAYER_SPEED);
            }
        }

        if is_key_down(KeyCode::Right) {
            game_state.player_angle_radians += 0.1;
        }
        if is_key_down(KeyCode::Left) {
            game_state.player_angle_radians -= 0.1;
        }
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0)
        }

        next_frame().await
    }
}

fn collision_check(x: f32, y: f32) -> bool {
    let x_r = x.floor() as usize;
    let y_r = y.floor() as usize;

    MAP[y_r][x_r] != 0
}

fn walk(state: &mut State, speed: f32) {
    let dx = speed * state.player_angle_radians.cos();
    let dy = speed * state.player_angle_radians.sin();

    let radius_plater_in_map = SIZE_PLAYER / SIZE_BOCK;

    let collision_x = if dx > 0.0 {
        radius_plater_in_map
    } else {
        -radius_plater_in_map
    };

    let collision_y = if dy > 0.0 {
        radius_plater_in_map
    } else {
        -radius_plater_in_map
    };

    if !collision_check(state.player_x + collision_x, state.player_y) {
        state.player_x += dx;
    };

    if !collision_check(state.player_x, state.player_y + collision_y) {
        state.player_y += dy;
    }
}

fn castRay(state: &mut State, angle: f32, range: f32) -> (f32, f32) {
    let sin = angle.sin();
    let cos = angle.cos();
    let tan = sin / cos;
    let cot = cos / sin;
    let cosNegative = cos < 0.0;
    let sinNegative = sin < 0.0;

    let mut currentX = state.player_x;
    let mut currentY = state.player_y;

    let mut distance = 0.0;

    while (distance < range) {
        // Collision with the nearest axis X
        let dxx = if cosNegative {
            (currentX - 1.0).ceil() - currentX
        } else {
            (currentX + 1.0).floor() - currentX
        };

        let dxy = dxx * tan;
        let lengthX2 = dxx * dxx + dxy * dxy;

        // Collision with the nearest axis Y
        let dyx = if sinNegative {
            (currentY - 1.0).ceil() - currentY
        } else {
            (currentY + 1.0).floor() - currentY
        };

        let dyy = dyx * cot;
        let lengthY2 = dyx * dyx + dyy * dyy;

        let mut collision = false;

        if (lengthX2 < lengthY2) {
            currentX = currentX + dxx;
            currentY = currentY + dxy;
            distance = distance + lengthX2.sqrt();

            let shift = if cosNegative { 1.0 } else { 0.0 };

            collision = collision_check(currentX - shift, currentY);
        } else {
            currentX = currentX + dyy;
            currentY = currentY + dyx;
            distance = distance + lengthY2.sqrt();

            let shift = if sinNegative { 1.0 } else { 0.0 };
            collision = collision_check(currentX, currentY - shift);
        }

        if (collision) {
            break;
        };
    }

    (currentX, currentY)
}

