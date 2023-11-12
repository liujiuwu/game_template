use std::time::{Duration, Instant};
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


        }

        if let Some(timer) = self.timer {
            ctx.print_centered(0, &format!("Time {:?}s Score {}", timer.elapsed().as_secs(), self.score));

            // 这里计时器，方便用于游戏实现初期的基本调试，实现后这段代码将移除
            if timer.elapsed() >= Duration::from_secs(5) {
                self.mode = GameMode::End;
            } else {}

            //TODO 游戏实现的地方
        }
    }
}