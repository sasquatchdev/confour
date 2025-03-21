use board::Board;
use display::{HEIGHT, MARGIN, SIZE, WIDTH};

use macroquad::{input::mouse_position, miniquad::window::set_window_size, window::next_frame};

pub mod board;
pub mod detect;
pub mod display;

#[macroquad::main("ConFour")]
async fn main() {
    set_window_size(WIDTH as u32, HEIGHT as u32);

    let mut board = Board::new();

    loop {
        // tick(&mut board).await;
        draw(&mut board).await;
        next_frame().await;
    }
}


async fn _tick(_board: &mut Board) {
    todo!()
}

async fn draw(board: &mut Board) {
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
