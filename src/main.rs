use macroquad::prelude::*;

const PADDING: f32 = 40.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED_START: f32 = 200.0;
const BALL_SPEED_MAX: f32 = 800.0;
const BALL_SPEED_INCREMENT: f32 = 50.0;
const COUNTDOWN_SECONDS: f32 = 3.0;

// The tree image is drawn as a square this fraction of the smaller
// screen dimension, so it scales as the window is resized.
const CENTER_IMAGE_SCREEN_FRACTION: f32 = 0.7;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut Vec2, up_key: KeyCode, down_key: KeyCode, dt: f32) {
    if is_key_down(up_key) {
        pos.y -= PLAYER_SPEED * dt;
    }
    if is_key_down(down_key) {
        pos.y += PLAYER_SPEED * dt;
    }
    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_height() - RACKET_HEIGHT_HALF,
    );
}

fn randomize_vec(vec: &mut Vec2, x: f32, y: f32) {
    vec.x = if rand::gen_range(0, 2) == 0 { x } else { -x };
    vec.y = if rand::gen_range(0, 2) == 0 { y } else { -y };
}

/// Rasterizes an SVG file into a macroquad Texture2D.
/// macroquad can only decode raster formats (PNG/JPEG/...), so the SVG is
/// rendered to pixels once at startup using resvg, then uploaded as a texture.
fn load_svg_texture(path: &str) -> Texture2D {
    let svg_data = std::fs::read(path)
        .unwrap_or_else(|e| panic!("failed to read {path}: {e}"));

    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &opt)
        .unwrap_or_else(|e| panic!("failed to parse {path}: {e}"));

    let size = tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
        .expect("invalid svg dimensions");

    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let texture = Texture2D::from_rgba8(size.width() as u16, size.height() as u16, pixmap.data());
    texture.set_filter(FilterMode::Linear);
    texture
}

struct MainState {
    player_1_pos: Vec2,
    player_2_pos: Vec2,
    ball_pos: Vec2,
    ball_vel: Vec2,
    ball_speed: f32,
    countdown: f32,
    player_1_score: i32,
    player_2_score: i32,
    center_texture: Texture2D,
}

impl MainState {
    fn new(center_texture: Texture2D) -> Self {
        let (screen_w, screen_h) = (screen_width(), screen_height());

        MainState {
            player_1_pos: Vec2::new(RACKET_WIDTH_HALF + PADDING, screen_h * 0.5),
            player_2_pos: Vec2::new(screen_w - RACKET_WIDTH_HALF - PADDING, screen_h * 0.5),
            ball_pos: Vec2::new(screen_w * 0.5, screen_h * 0.5),
            ball_vel: Vec2::ZERO,
            ball_speed: BALL_SPEED_START,
            countdown: COUNTDOWN_SECONDS,
            player_1_score: 0,
            player_2_score: 0,
            center_texture,
        }
    }

    fn update(&mut self, dt: f32) {
        let (screen_w, screen_h) = (screen_width(), screen_height());

        // Keep rackets pinned to the left/right edges if the window is resized.
        self.player_1_pos.x = RACKET_WIDTH_HALF + PADDING;
        self.player_2_pos.x = screen_w - RACKET_WIDTH_HALF - PADDING;

        move_racket(&mut self.player_1_pos, KeyCode::W, KeyCode::S, dt);
        move_racket(&mut self.player_2_pos, KeyCode::Up, KeyCode::Down, dt);

        // While the countdown is running, freeze the ball at center and
        // don't run any ball physics.
        if self.countdown > 0.0 {
            self.countdown -= dt;
            self.ball_pos = Vec2::new(screen_w * 0.5, screen_h * 0.5);
            if self.countdown <= 0.0 {
                self.countdown = 0.0;
                self.ball_vel = Vec2::ZERO;
                randomize_vec(&mut self.ball_vel, self.ball_speed, self.ball_speed);
            }
            return;
        }

        self.ball_pos += self.ball_vel * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos = Vec2::new(screen_w * 0.5, screen_h * 0.5);
            self.ball_speed = BALL_SPEED_START;
            self.countdown = COUNTDOWN_SECONDS;
            self.player_2_score += 1;
        }
        if self.ball_pos.x > screen_w {
            self.ball_pos = Vec2::new(screen_w * 0.5, screen_h * 0.5);
            self.ball_speed = BALL_SPEED_START;
            self.countdown = COUNTDOWN_SECONDS;
            self.player_1_score += 1;
        }

        // ball, Y bounce
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let intersects_player_1 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
            self.increase_ball_speed();
        }

        let intersects_player_2 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
            self.increase_ball_speed();
        }
    }

    /// Bumps the ball's speed up by BALL_SPEED_INCREMENT (capped at
    /// BALL_SPEED_MAX) while preserving its current direction.
    fn increase_ball_speed(&mut self) {
        self.ball_speed = (self.ball_speed + BALL_SPEED_INCREMENT).min(BALL_SPEED_MAX);
        let direction = self.ball_vel.normalize_or_zero();
        self.ball_vel = direction * self.ball_speed;
    }

    fn draw(&self) {
        clear_background(BLACK);

        let screen_w = screen_width();
        let screen_h = screen_height();

        // static center image, scaled to the window
        let image_size = screen_w.min(screen_h) * CENTER_IMAGE_SCREEN_FRACTION;
        draw_texture_ex(
            &self.center_texture,
            screen_w * 0.5 - image_size * 0.5,
            screen_h * 0.5 - image_size * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(image_size, image_size)),
                ..Default::default()
            },
        );

        // rackets
        draw_rectangle(
            self.player_1_pos.x - RACKET_WIDTH_HALF,
            self.player_1_pos.y - RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
            WHITE,
        );
        draw_rectangle(
            self.player_2_pos.x - RACKET_WIDTH_HALF,
            self.player_2_pos.y - RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
            WHITE,
        );

        // ball
        draw_circle(self.ball_pos.x, self.ball_pos.y, BALL_SIZE_HALF, WHITE);

        // countdown, shown near the score so the ball doesn't cover it
        if self.countdown > 0.0 {
            let count = self.countdown.ceil() as i32;
            let count_text = count.to_string();
            let font_size = 60u16;
            let dims = measure_text(&count_text, None, font_size, 1.0);
            draw_text(
                &count_text,
                screen_w * 0.5 - dims.width * 0.5,
                110.0 + dims.height * 0.5,
                font_size as f32,
                WHITE,
            );
        }

        // score
        let score_text = format!("{}         {}", self.player_1_score, self.player_2_score);
        let font_size = 40u16;
        let dims = measure_text(&score_text, None, font_size, 1.0);
        draw_text(
            &score_text,
            screen_w * 0.5 - dims.width * 0.5,
            40.0 + dims.height * 0.5,
            font_size as f32,
            WHITE,
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pong".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let center_texture = load_svg_texture("assets/tree-in-circle.svg");
    let mut state = MainState::new(center_texture);

    loop {
        let dt = get_frame_time();
        state.update(dt);
        state.draw();
        next_frame().await;
    }
}