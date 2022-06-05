use std::sync::Mutex;
use std::collections::HashMap;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position, Move, Square, Role};

pub struct GameState(pub Mutex<Chess>);

pub fn create_state() -> Chess {
    let pos = Chess::default();
    return pos
}

#[tauri::command]
pub fn play_move(state: tauri::State<'_, GameState>) {
    let mut state_guard = state.0.lock().unwrap();
    let mut game = state_guard.clone();

    game = match game.play(&Move::Normal {
        role: Role::Pawn,
        from: Square::E2,
        to: Square::E4,
        capture: None,
        promotion: None,
    }) {
        Ok(v) => v,
        Err(e) => {
            println!("Error playing a move {}", e);
            return
        }
    };
    *state_guard = game;
}

#[tauri::command]
pub fn check_legal_moves(game_fen: String) -> HashMap<String, Vec<String>>  {
    // let mut hash_moves:HashMap<shakmaty::Square, Vec<shakmaty::Square>> = HashMap::new();
    let mut hash_moves:HashMap<String, Vec<String>> = HashMap::new();
    //Setup position since state is not kept
    let fen: Fen = match game_fen.parse() {
        Ok(v) => v,
        Err(e) => {
            println!("Error Happned parsing FEN {:?}", e);
            return hash_moves
        },
    };
    let pos: Chess = match fen.into_position(CastlingMode::Standard) {
        Ok(v) => v,
        Err(pos_err) => {
            println!("There was an error creating the position {}", pos_err);
            return hash_moves
        }
    };
    let legal_moves = pos.legal_moves();
    println!("Current Move: {}", pos.turn());
    for a_move in legal_moves.iter() {
        match a_move {
            shakmaty::Move::Normal {role, from, capture, to, promotion,} => {
                drop(role);drop(capture);drop(promotion);
                let string_from: String = convert_square_to_char(from);
                let string_to: String = convert_square_to_char(to);
                match hash_moves.get_mut(&string_from) {
                    Some(possible_moves) => {
                        possible_moves.push(string_to);
                    },
                    None => {
                        hash_moves.insert(string_from, vec![string_to]);
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
    return hash_moves
}

fn convert_square_to_char(s: &shakmaty::Square) -> String {
    let (file, rank) = s.coords();
    let f_char = file.char();
    let r_char = rank.char();
    let mut string = String::from(f_char);
    string.push(r_char);
    return string
}
