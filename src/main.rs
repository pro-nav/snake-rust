use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Pos {
    x: i32,
    y: i32,
}

fn generate_food(game_width: i32, game_height: i32, snake: &Vec<Pos>) -> Pos {
    let mut rng = thread_rng();
    let mut overlap = true;
    let mut food_pos = Pos { x: 0, y: 0 };

    while overlap {
        overlap = false;

        food_pos = Pos {
            x: rng.gen_range(1..(game_width / 10)) * 10,
            y: rng.gen_range(1..(game_height / 10)) * 10,
        };

        for segment_pos in snake.iter() {
            if food_pos.x == segment_pos.x && food_pos.y == segment_pos.y {
                overlap = true;
            }
        }
    }
    return food_pos;
}

fn main() -> Result<(), String> {
    // sdl initialization
    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust!", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let screen_area = Rect::new(0, 0, screen_width, screen_height);

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    // game variables
    let mut pause = false;
    let target_frame_duration = Duration::from_secs_f32(1.0 / 24.0);
    let mut direction = Direction::RIGHT;
    let mut snake: Vec<Pos> = Vec::new();
    snake.push(Pos { x: 10, y: 10 });
    let mut last_frame = Instant::now();
    let mut food = generate_food(screen_width as i32, screen_height as i32, &snake);

    while running {
        // handle events
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }

                Event::KeyDown { keycode, .. } => {
                    let p = keycode.unwrap();
                    match p {
                        Keycode::Q => {
                            running = false;
                        }
                        Keycode::Up => {
                            if direction != Direction::DOWN {
                                direction = Direction::UP
                            }
                        }
                        Keycode::Down => {
                            if direction != Direction::UP {
                                direction = Direction::DOWN
                            }
                        }
                        Keycode::Left => {
                            if direction != Direction::RIGHT {
                                direction = Direction::LEFT
                            }
                        }
                        Keycode::Right => {
                            if direction != Direction::LEFT {
                                direction = Direction::RIGHT
                            }
                        }
                        Keycode::Space => pause = !pause,
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        let currunt_frame = Instant::now();
        if !pause && (currunt_frame - last_frame > target_frame_duration) {
            let mut movement_x = 0;
            let mut movement_y = 0;
            match direction {
                Direction::DOWN => {
                    movement_y = 10;
                }
                Direction::UP => {
                    movement_y = -10;
                }
                Direction::LEFT => {
                    movement_x = -10;
                }
                Direction::RIGHT => {
                    movement_x = 10;
                }
            }

            // if let Some(head) = snake.get(0) {
            let head = Pos {
                x: snake[0].x,
                y: snake[0].y,
            };
            let mut delta_x = head.x + movement_x;
            if delta_x > screen_width as i32 - 10 {
                delta_x = 0;
            } else if delta_x < 0 {
                delta_x = screen_width as i32 - 10;
            }

            let mut delta_y = head.y + movement_y;
            if delta_y > screen_height as i32 - 10 {
                delta_y = 0;
            } else if delta_y < 0 {
                delta_y = screen_height as i32 - 10;
            }

            for segment_pos in snake.iter() {
                if segment_pos.x == delta_x && segment_pos.y == delta_y {
                    println!("Collision");
                    pause = true;
                }
            }

            snake.insert(
                0,
                Pos {
                    x: delta_x,
                    y: delta_y,
                },
            );

            if head.x == food.x && head.y == food.y {
                food = generate_food(screen_width as i32, screen_height as i32, &snake);
            } else {
                let _ = snake.pop();
            }

            last_frame = Instant::now()
        }

        // renderer
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _ = canvas.fill_rect(screen_area);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for segment_pos in snake.iter() {
            let segment = Rect::new(segment_pos.x, segment_pos.y, 10, 10);
            let _ = canvas.fill_rect(segment);
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let food_cell = Rect::new(food.x, food.y, 10, 10);
        let _ = canvas.fill_rect(food_cell);

        canvas.present();
    }

    Ok(())
}
