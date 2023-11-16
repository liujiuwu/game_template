use std::time::Instant;
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50().build()?;
    main_loop(ctx, State::new())
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    score: i32,
    timer: Option<Instant>,
    snake: Snake,
    food: Food,
}

fn event_handler(state: &mut State, ctx: &mut BTerm) {
    ctx.cls();
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => state.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx, |state, ctx| event_handler(state, ctx)),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx, |state, ctx| event_handler(state, ctx)),
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            frame_time: 0.0,
            score: 0,
            timer: None,
            snake: Snake::new(),
            food: Food::with_default(),
        }
    }

    fn restart(&mut self) {
        *self = Self::new();
        self.mode = GameMode::Playing;
        self.timer = Some(Instant::now());
    }

    fn menu<F>(&mut self, ctx: &mut BTerm, event_handler: F)
        where F: Fn(&mut State, &mut BTerm) -> ()
    {
        event_handler(self, ctx);
        ctx.print_color_centered(8, CYAN, BLACK, "Welcome to xxx");
        ctx.print_color_centered(12, CYAN, BLACK, "(P) Player Game");
        ctx.print_color_centered(16, CYAN, BLACK, "(Q) Quit Game");
    }

    fn dead<F>(&mut self, ctx: &mut BTerm, event_handler: F)
        where F: Fn(&mut State, &mut BTerm) -> ()
    {
        event_handler(self, ctx);
        ctx.print_color_centered(8, RED, BLACK, "You are dead");
        ctx.print_color_centered(12, CYAN, BLACK, &format!("You earned {} points", self.score));
        ctx.print_color_centered(16, CYAN, BLACK, "(P) Player Game");
        ctx.print_color_centered(20, CYAN, BLACK, "(Q) Quit Game");
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            let new_head_point = self.snake.auto_move();
            if !self.is_valid_move(new_head_point) || self.snake.colides_with_body(new_head_point) {
                self.mode = GameMode::End;
            }

            self.snake.segments.insert(0, new_head_point);
            if self.snake.head() == self.food.point {
                self.generate_food();
                self.score += 1;
            } else {
                self.snake.segments.pop();
            }
        }


        if let Some(timer) = self.timer {
            ctx.print_centered(0, &format!("Time {:?}s Score {}", timer.elapsed().as_secs(), self.score));
        }


        self.snake.render(ctx);
        self.food.render(ctx);
    }

    fn is_valid_move(&self, point: Point) -> bool {
        point.x > 0 && point.x < SCREEN_WIDTH && point.y > 0 && point.y < SCREEN_HEIGHT
    }

    fn generate_food(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        loop {
            let (x, y) = (rng.range(1, SCREEN_WIDTH), rng.range(1, SCREEN_HEIGHT));
            if !self.snake.segments.contains(&Point::new(x, y)) {
                self.food = Food::new(x, y);
                break;
            }
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    segments: Vec<Point>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        Self {
            segments: vec![Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)],
            direction: Direction::Right,
        }
    }

    fn head(&self) -> Point {
        self.segments[0]
    }

    fn render(&mut self, ctx: &mut BTerm) {
        for segment in &self.segments {
            ctx.set(segment.x, segment.y, WHITE, BLACK, to_cp437('@'));
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => if self.direction != Direction::Down {
                    self.direction = Direction::Up
                }
                VirtualKeyCode::Down => if self.direction != Direction::Up {
                    self.direction = Direction::Down
                }
                VirtualKeyCode::Left => if self.direction != Direction::Right {
                    self.direction = Direction::Left
                }
                VirtualKeyCode::Right => if self.direction != Direction::Left {
                    self.direction = Direction::Right
                }
                _ => {}
            }
        }
    }

    fn auto_move(&self) -> Point {
        let head_point = self.head();
        match self.direction {
            Direction::Up => Point::new(head_point.x, head_point.y - 1),
            Direction::Down => Point::new(head_point.x, head_point.y + 1),
            Direction::Left => Point::new(head_point.x - 1, head_point.y),
            Direction::Right => Point::new(head_point.x + 1, head_point.y),
        }
    }

    fn colides_with_body(&self, new_head_point: Point) -> bool {
        self.segments[1..].contains(&new_head_point)
    }
}

struct Food {
    point: Point,
}

use std::ops::Deref;

impl Deref for Food {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}


impl Food {
    fn new(x: i32, y: i32) -> Self {
        Self {
            point: Point::new(x, y)
        }
    }

    fn with_default() -> Self {
        Self::new(SCREEN_WIDTH / 4, SCREEN_HEIGHT / 4)
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, GREEN, BLACK, to_cp437('*'))
    }
}