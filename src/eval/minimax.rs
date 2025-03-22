use std::f32;

use crate::board::{Player, State, MAXIMIZER, MINIMIZER};

pub type TranspositionTable = std::collections::HashMap<TranspositionKey, TranspositionEntry>;
pub type TranspositionKey = u64;

pub struct TranspositionEntry {
    pub eval: f32,
    pub depth: usize,
    pub flag: TranspositionFlag
}

pub enum TranspositionFlag {
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
}

impl State {
    pub fn negamax(
        &self,
        depth: usize,

        // transposition table
        tt: &mut TranspositionTable,

        // alpha-beta pruning
        mut alpha: f32, // root: -inf
        mut beta: f32,  // root: +inf
    ) -> f32 {
        let alpha_original = alpha;

        let entry = tt.get(&self.hash_value());
        if let Some(entry) = entry {
            if entry.depth >= depth {
                match entry.flag {
                    TranspositionFlag::EXACT => return entry.eval,
                    TranspositionFlag::LOWERBOUND => alpha = alpha.max(entry.eval),
                    TranspositionFlag::UPPERBOUND => beta = beta.min(entry.eval),
                }

                if alpha >= beta {
                    return entry.eval;
                }
            }
        }

        if depth == 0 || self.is_terminal() {
            return self.evaluate();
        }
        
        let mut value = f32::NEG_INFINITY;
        for column in self.get_valid() {
            let child = self.dropped(column, self.player());
            let score = -child.negamax(depth - 1, tt, -beta, -alpha);   // note: negation + swap (wikipedia: negamax)
            value = value.max(score);

            alpha = alpha.max(value);
            if alpha >= beta {
                // break;
            }
        }

        let flag = if value <= alpha_original { TranspositionFlag::UPPERBOUND }
        else if value >= beta { TranspositionFlag::LOWERBOUND }
        else { TranspositionFlag::EXACT };

        tt.insert(self.hash_value(), TranspositionEntry {
            eval: value,
            depth,
            flag
        });

        value
    }

    pub fn best(
        &self,
        depth: usize,
        player: Player,
        tt: &mut TranspositionTable,
    ) -> Option<usize> {
        let mut best_eval = if player == MAXIMIZER { f32::NEG_INFINITY } else { f32::INFINITY };
        let mut best_column: Option<usize> = None;

        for column in self.get_valid() {
            let child = self.dropped(column, player);
            let score = child.negamax(depth, tt, f32::NEG_INFINITY, f32::INFINITY);  // note: negation
            if (player == MAXIMIZER && score > best_eval) || (player == MINIMIZER && score < best_eval) {
                best_eval = score;
                best_column = Some(column);
            }
        }

        println!();

        best_column
    }
}
