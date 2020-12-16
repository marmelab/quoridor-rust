table! {
    game (id) {
        id: String,
        over: bool,
        pawn_turn: u8,
        pawns: Vec<Pawn>,
        fences: Vec<Fence>,
        board: Board,
    }
}

table! {
    score (id) {
        id -> Int32,
        game_id -> String
    }
}

joinable!(game -> score (game_id));

allow_tables_to_appear_in_same_query!(game, scores);
