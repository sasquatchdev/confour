use macroquad::{color::{self, Color}, shapes::{self, draw_rectangle}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};
use crate::{board::{Cell, Player, State, COLS, MAXIMIZER, ROWS}, detect::sequences_all};

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

pub async fn draw_game_over(state: &State, winner: Option<Player>) {
    draw_board(&state).await;
    
    if let Some(player) = winner {
        let seqs = sequences_all(state, player)
            .into_iter()
            .filter(|seq| seq.len() >= 4);

        for seq in seqs {
            let start = seq[0];
            let end = seq[seq.len() - 1];

            let x1 = start.1 as f32 * SIZE + MARGIN + SIZE / 2.0;
            let y1 = start.0 as f32 * SIZE + MARGIN + SIZE / 2.0;

            let x2 = end.1 as f32 * SIZE + MARGIN + SIZE / 2.0;
            let y2 = end.0 as f32 * SIZE + MARGIN + SIZE / 2.0;

            shapes::draw_line(x1, y1, x2, y2, 12.0, if player == MAXIMIZER {
                color::YELLOW
            } else {
                color::RED
            });
        }
    }
    
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(255, 255, 255, 128));
    
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
