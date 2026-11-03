# Rusty Pong

A classic two-player Pong game written in Rust with [macroquad](https://macroquad.rs). Two rackets, one ball, and a rising pace: every hit makes the ball faster, so rallies get tense quickly.

## Features

- **Local two-player gameplay** on a single keyboard.
- **Speed-up on every hit**: the ball starts at 200 px/s and gains 50 px/s with each paddle contact, up to a cap of 800 px/s.
- **Countdown before each serve**: a 3-second countdown gives both players time to get ready after every point.
- **Random serve direction**, so no side gets a predictable start.
- **Resizable window**: rackets stay pinned to the screen edges and the layout adapts as you resize.
- **SVG background art**: a tree-in-circle image is rasterized at startup with [resvg](https://github.com/RazrFalcon/resvg) and scaled to the window.

## Controls

| Player | Move up | Move down |
| --- | --- | --- |
| Player 1 (left) | `W` | `S` |
| Player 2 (right) | `↑` | `↓` |

## Rules

- Bounce the ball back with your racket to keep the rally going.
- If the ball gets past your racket, the other player scores a point.
- After each point the ball returns to the center, its speed resets, and a 3-second countdown begins.
- The score is shown at the top of the screen. There is no win limit, so play to whatever target you like.

## Getting started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Platform dependencies for macroquad. On Linux you may need ALSA and X11/OpenGL development packages, see the [macroquad docs](https://github.com/not-fl3/macroquad#linux).

### Run

```bash
git clone <your-repo-url>
cd rusty-pong
cargo run --release
```

Run the game from the project root so that `assets/tree-in-circle.svg` can be found.

## Project layout

```
rusty-pong/
├── Cargo.toml
├── assets/
│   └── tree-in-circle.svg
└── src/
    └── main.rs
```

## Configuration

Gameplay is tuned by constants at the top of `src/main.rs`:

| Constant | Default | Description |
| --- | --- | --- |
| `PLAYER_SPEED` | 600 | Racket movement speed (px/s) |
| `BALL_SPEED_START` | 200 | Ball speed at serve (px/s) |
| `BALL_SPEED_MAX` | 800 | Maximum ball speed (px/s) |
| `BALL_SPEED_INCREMENT` | 50 | Speed gained per racket hit (px/s) |
| `COUNTDOWN_SECONDS` | 3 | Pause before each serve |
| `RACKET_HEIGHT` / `RACKET_WIDTH` | 100 / 20 | Racket dimensions (px) |
| `BALL_SIZE` | 30 | Ball diameter (px) |
| `CENTER_IMAGE_SCREEN_FRACTION` | 0.7 | Background image size relative to the smaller window dimension |

## How it works

- **Game state**: a single `MainState` struct holds racket and ball positions, scores, the countdown, and the background texture.
- **Collisions**: ball-vs-racket checks are axis-aligned rectangle overlap tests. The ball bounces off the top and bottom edges by flipping its vertical velocity.
- **Speed control**: on each racket hit, the ball's direction is kept and its velocity is rescaled to the new speed.
- **SVG loading**: macroquad only decodes raster formats, so the SVG is parsed and rendered to a pixel buffer with `resvg`, then uploaded as a `Texture2D`.
