use macroquad::{color::{self, Color}, shapes, text::{draw_text, measure_text}, window::{clear_background, screen_height, screen_width}};
use crate::board::{Cell, Player, State, COLS, ROWS};

pub const SIZE: f32 = 100.0;

pub const MARGIN: f32 = 50.0;

pub const WIDTH: f32 = COLS as f32 * SIZE + 2.0 * MARGIN;
pub const HEIGHT: f32 = ROWS as f32 * SIZE + 2.0 * MARGIN;

pub async fn draw_board(state: &State) {
    for row in 0 .. ROWS {
        for col in 0 .. COLS {
            draw_cell(state, row, col).await;
        }
    }
}

pub async fn draw_cell(state: &State, row: usize, col: usize) {
    let cell = state[(row, col)];
    let color = match cell {
        Cell::Empty => color::WHITE,
        Cell::Player { player } => match player {
            Player::Red => color::RED,
            Player::Yellow => color::YELLOW,
        }
    };

    let size = 100.0;

    let x = col as f32 * size + MARGIN + size / 2.0;
    let y = row as f32 * size + MARGIN + size / 2.0;

    shapes::draw_circle(x, y, size / 2.25, color);
}

pub async fn draw_highlight(state: &State, col: usize) {
    let x = col as f32 * SIZE + MARGIN;
    let y = 0.0;

    let color = if state.player() == Player::Red {
        Color::from_rgba(255, 0, 0, 128)
    } else {
        Color::from_rgba(255, 255, 0, 128)
    };

    shapes::draw_rectangle(
        x, y, 
        SIZE, HEIGHT, 
        color
    )
}

pub async fn draw_game_over(winner: Option<Player>) {
    clear_background(color::WHITE);

    let text = match winner {
        Some(Player::Red) => "Red wins!",
        Some(Player::Yellow) => "Yellow wins!",
        None => "It's a draw!",
    };

    let font_size = 48.0;
    let text_size = measure_text(text, None, font_size as _, 1.0);

    draw_text(
        text, 
        screen_width() / 2. - text_size.width / 2., 
        screen_height() / 2. - text_size.height / 2., 
        font_size, 
        color::BLACK
    );

    let text = "Press [r] to restart";
    let font_size = 24.0;
    let text_size = measure_text(text, None, font_size as _, 1.0);

    draw_text(
        text, 
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. + text_size.height / 2. + 25.0,
        font_size, 
        color::BLACK
    );
}
