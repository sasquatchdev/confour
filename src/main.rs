use board::Board;
use detect::get_winner;
use display::{HEIGHT, MARGIN, SIZE, WIDTH};

use macroquad::{input::{is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton}, miniquad::window::set_window_size, window::next_frame};

pub mod board;
pub mod detect;
pub mod display;

#[macroquad::main("ConFour")]
async fn main() {
    set_window_size(WIDTH as u32, HEIGHT as u32);

    let mut board = Board::new();

    loop {
        tick(&mut board).await;
        draw(&mut board).await;
        next_frame().await;
    }
}


async fn tick(board: &mut Board) {
    if get_winner(board.state()).is_some() || board.state().is_full() {
        if is_key_down(KeyCode::R) {
            *board = Board::new();
        }
        return;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(col) = mouse_column() {
            let player = board.state().player();
            board.state_mut().drop(col, player);
        }
    }
}

async fn draw(board: &mut Board) {
    if let Some(winner) = get_winner(board.state()) {
        display::draw_game_over(Some(winner)).await;
        return;
    } else if board.state().is_full() {
        display::draw_game_over(None).await;
        return;
    }

    let col = mouse_column();
    if let Some(col) = col {
        display::draw_highlight(board.state(), col).await;
    }

    display::draw_board(board.state()).await;
}

fn mouse_column() -> Option<usize> {
    let (x, _) = mouse_position();
    let col = ((x - MARGIN) / SIZE) as usize;
    
    match x {
        x if x < MARGIN => return None,
        x if x > WIDTH - MARGIN => return None,
        _ => (),
    }

    Some(col)
}
