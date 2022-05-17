use shakmaty::{Chess, Position};

// -> ArrayVec<shakmaty::Move> {
//pub fn check_legal_moves() -> shakmaty::MoveList {
#[tauri::command]
pub fn check_legal_moves() {
    let pos = Chess::default();
    let mut legal_moves = pos.legal_moves();
    for i in legal_moves.iter_mut() {
        println!("{}", i)
    }
}
