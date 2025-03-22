use std::{sync::{Arc, Mutex}, thread};

use board::{Board, MAXIMIZER, MINIMIZER};
use display::{HEIGHT, MARGIN, SIZE, WIDTH};

use macroquad::{input::{is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton}, miniquad::window::set_window_size, window::next_frame};

pub mod board;
pub mod detect;
pub mod display;
pub mod eval;

pub const DEPTH: usize = 6;

#[macroquad::main("ConFour")]
async fn main() {
    set_window_size(WIDTH as u32, HEIGHT as u32);

    let board = Arc::new(Mutex::new(Board::new()));

    let board_eval = Arc::clone(&board);
    let board_draw = Arc::clone(&board);

    thread::spawn(move || {
        async_std::task::block_on(async {
            loop {
                {
                    let should_eval;
                    {
                        let state = board_eval.lock().unwrap().state().clone();
                        should_eval = state.player() == MINIMIZER;
                    }
    
                    if should_eval {
                        let col = eval_omove(&board).await;
                        let mut board = board_eval.lock().unwrap();
                        let state = board.state_mut();
                        state.drop(col.unwrap(), MINIMIZER);
                    }
                }
            }
        })
    });

    loop {
        tick(&board_draw).await;
        draw(&board_draw).await;
        next_frame().await;
    }
}

async fn eval_omove(board: &Mutex<Board>) 
    -> Option<usize>
{
    let state;
    {
        // Having this inner scope allows the
        // mutex to be "unlocked" before the
        // actual evaluation takes place. This
        // makes board accessible to the main
        // thread while the evaluation is running.
        state = board.lock().unwrap().state().clone();
    }

    let best = state.best(DEPTH, MINIMIZER);

    best
}

async fn tick(board: &Mutex<Board>) {
    let mut board = board.lock().unwrap();
    let state = board.state_mut();

    if state.get_winner().is_some() || state.is_full() {
        if is_key_down(KeyCode::R) {
            *board = Board::new();
        }
        return;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(col) = mouse_column() {
            let player = state.player();
            if player == MAXIMIZER {
                state.drop(col, player);
            }
        }
    }
}

async fn draw(board: &Mutex<Board>) {
    let state;
    {
        state = board.lock().unwrap().state().clone();
    }

    if state.is_terminal() {
        let winner = state.get_winner();
        display::draw_game_over(&state, winner).await;

        return;
    }

    let col = mouse_column();
    if let Some(col) = col {
        if state.player() == MAXIMIZER {
            display::draw_highlight(&state, col).await;
        }
    }

    display::draw_board(&state).await;
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
