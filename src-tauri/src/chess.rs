use std::collections::HashMap;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

#[tauri::command]
pub fn check_legal_moves(game_fen: String) {
    //Setup the position
    let fen: Fen = match game_fen.parse() {
        Ok(v) => v,
        Err(e) => {
            println!("Error Happned parsing FEN {:?}", e);
            return
        },
    };
    let pos: Chess = match fen.into_position(CastlingMode::Standard) {
        Ok(v) => v,
        Err(pos_err) => {
            println!("There was an error creating the position {}", pos_err);
            return
        }
    };
    let mut hash_moves:HashMap<&shakmaty::Square, Vec<&shakmaty::Square>> = HashMap::new();
    let legal_moves = pos.legal_moves();
    for a_move in legal_moves.iter() {
        match a_move {
            shakmaty::Move::Normal {role, from, capture, to, promotion,} => {
                drop(role);drop(capture);drop(promotion);
                match hash_moves.get_mut(from) {
                    Some(possible_moves) => {
                        possible_moves.push(to);
                    },
                    None => {
                        hash_moves.insert(from, vec![to]);
                    }
                }
            },
            shakmaty::Move::EnPassant{from, to } => {
                println!("Croissant is possible from {} to {}", from, to);
            },
            shakmaty::Move::Castle {king, rook} => {
                println!("Caslting Rook {}", rook);
                println!("Isn't king just going to say e1? {}", king);
            },
            shakmaty::Move::Put {role, to} => {
                // Not currently possible because dumb
                println!("A put somehow: {:?},{}", role, to);
            }
        }
    }
    println!("Current Legal Moves: {:#?}", hash_moves);
    //return hash_moves
}
