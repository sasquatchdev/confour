use std::f32;

use crate::board::{Player, State, MAXIMIZER, MINIMIZER};

impl State {
    pub fn negamax(
        &self,
        depth: usize,
        mut alpha: f32, // root: -inf
        beta: f32,  // root: +inf
    ) -> f32 {
        if depth == 0 || self.is_terminal() {
            return self.evaluate();
        }
        
        let mut value = f32::NEG_INFINITY;
        for column in self.get_valid() {
            let child = self.dropped(column, self.player());
            let score = -child.negamax(depth - 1, -beta, -alpha);   // note: negation + swap (wikipedia: negamax)
            value = value.max(score);

            alpha = alpha.max(value);
            if alpha >= beta {
                // break;
            }
        }

        value
    }

    pub fn best(
        &self,
        depth: usize,
        player: Player,
    ) -> Option<usize> {
        let mut best_eval = if player == MAXIMIZER { f32::NEG_INFINITY } else { f32::INFINITY };
        let mut best_column: Option<usize> = None;

        for column in self.get_valid() {
            let child = self.dropped(column, player);
            let score = child.negamax(depth, f32::NEG_INFINITY, f32::INFINITY);  // note: negation
            if (player == MAXIMIZER && score > best_eval) || (player == MINIMIZER && score < best_eval) {
                best_eval = score;
                best_column = Some(column);
            }
        }

        println!();

        best_column
    }
}
