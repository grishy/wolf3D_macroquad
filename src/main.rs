use macroquad::prelude::*;

// more info: https://grantshandy.github.io/posts/raycasting/
// code reference: https://github.com/grantshandy/wasm4-raycaster/blob/main/src/lib.rs
// my old code: https://github.com/grishy/SNO/blob/master/04/main.js

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
const FOCAL_LENGTH: f32 = 0.8;

struct State {
    player_x: f32,
    player_y: f32,
    player_angle_radians: f32,
    render: Renter,
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

fn draw_top(game_state: &mut State) {
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
        let num_ray = macroquad::window::screen_width() as i32 / 2;

        for column in 0..=num_ray {
            // -0.5 < x < 0.5
            let x = column as f32 / num_ray as f32 - 0.5;
            let angle = game_state.player_angle_radians + x.atan2(FOCAL_LENGTH);
            let length = 40.0;

            let (ray_x, ray_y) = cast_ray(game_state, angle, length);

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
}

fn draw_first(game_state: &mut State) {
    {
        let num_ray = macroquad::window::screen_width() as i32;

        for column in 0..=num_ray {
            // -0.5 < x < 0.5
            let x = column as f32 / num_ray as f32 - 0.5;
            let angle = game_state.player_angle_radians + x.atan2(FOCAL_LENGTH);
            let length = 40.0;

            let (ray_x, ray_y) = cast_ray(game_state, angle, length);

            let v_player = dvec2(game_state.player_x as f64, game_state.player_y as f64);
            let v_ray = dvec2(ray_x as f64, ray_y as f64);
            let distance = v_player.distance(v_ray) as f32 * x.atan2(FOCAL_LENGTH).cos();
            let wall_height = 1000.0 / distance;

            let middle = macroquad::window::screen_height() / 2.0;
            let color = 10.0 / distance * 255.0;

            draw_line(
                column as f32,
                middle - wall_height,
                column as f32,
                middle + wall_height,
                1.0,
                Color::from_rgba(255, 0, 0, color as u8),
            );
        }
    }
}

enum Renter {
    Top,
    FirstPerson,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state: State = State {
        player_x: 2.2,
        player_y: 2.2,
        player_angle_radians: 0.0,
        render: Renter::Top,
    };

    loop {
        clear_background(macroquad::color::Color::from_rgba(224, 248, 207, 255));

        match game_state.render {
            Renter::Top => draw_top(&mut game_state),
            Renter::FirstPerson => draw_first(&mut game_state),
        }

        {
            if is_key_down(KeyCode::Up) {
                walk(&mut game_state, PLAYER_SPEED);
            }
            if is_key_down(KeyCode::Down) {
                walk(&mut game_state, -PLAYER_SPEED);
            }
        }

        if is_key_down(KeyCode::Right) {
            game_state.player_angle_radians += 0.05;
        }
        if is_key_down(KeyCode::Left) {
            game_state.player_angle_radians -= 0.05;
        }
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0)
        }

        {
            if is_key_down(KeyCode::Z) {
                game_state.render = Renter::Top;
            }
            if is_key_down(KeyCode::X) {
                game_state.render = Renter::FirstPerson;
            }
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

fn cast_ray(state: &mut State, angle: f32, range: f32) -> (f32, f32) {
    let sin = angle.sin();
    let cos = angle.cos();
    let tan = sin / cos;
    let cot = cos / sin;
    let cos_negative = cos < 0.0;
    let sin_negative = sin < 0.0;

    let mut current_x = state.player_x;
    let mut current_y = state.player_y;

    let mut distance = 0.0;

    while distance < range {
        // Collision with the nearest axis X
        let dxx = if cos_negative {
            (current_x - 1.0).ceil() - current_x
        } else {
            (current_x + 1.0).floor() - current_x
        };

        let dxy = dxx * tan;
        let length_x2 = dxx * dxx + dxy * dxy;

        // Collision with the nearest axis Y
        let dyx = if sin_negative {
            (current_y - 1.0).ceil() - current_y
        } else {
            (current_y + 1.0).floor() - current_y
        };

        let dyy = dyx * cot;
        let length_y2 = dyx * dyx + dyy * dyy;


        if length_x2 < length_y2 {
            current_x = current_x + dxx;
            current_y = current_y + dxy;
            distance = distance + length_x2.sqrt();

            let shift = if cos_negative { 1.0 } else { 0.0 };

            if collision_check(current_x - shift, current_y) {
                break;
            };
        } else {
            current_x = current_x + dyy;
            current_y = current_y + dyx;
            distance = distance + length_y2.sqrt();

            let shift = if sin_negative { 1.0 } else { 0.0 };
            if collision_check(current_x, current_y - shift) {
                break;
            }
        }

    }

    (current_x, current_y)
}
