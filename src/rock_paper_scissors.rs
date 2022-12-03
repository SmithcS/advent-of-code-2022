use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum GameOutcome {
    LOSE,
    WIN,
    DRAW
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS
}

pub fn calculate_game_score(game_strategy: &Vec<String>) -> i32 {
    let mut total_score: i32 = 0;
    let char_to_move_map = char_to_moves();
    let shape_score_map = shape_scores();
    let game_outcomes_score_map = game_outcomes_scores();

    for recommendation in game_strategy.iter() {
        let recommendation_chars: Vec<char> = recommendation.chars().collect();
        let opponent_choice = char_to_move_map.get(&recommendation_chars[0]).unwrap();
        let suggested_choice = char_to_move_map.get(&recommendation_chars[2]).unwrap();

        let outcome = determine_game_outcome(suggested_choice, opponent_choice);
        let outcome_score = game_outcomes_score_map.get(&outcome).unwrap();
        let shape_score = shape_score_map.get(suggested_choice).unwrap();

        total_score += outcome_score + shape_score;
    }

    return total_score;
}

fn determine_game_outcome(p1_move: &Move, p2_move: &Move) -> GameOutcome {
    match p1_move {
        &Move::ROCK => {
            if p2_move == &Move::ROCK { return GameOutcome::DRAW; }
            if p2_move == &Move::PAPER { return  GameOutcome::LOSE; }
            if p2_move == &Move::SCISSORS { return  GameOutcome::WIN; }
        },
        &Move::PAPER => {
            if p2_move == &Move::ROCK { return GameOutcome::WIN; }
            if p2_move == &Move::PAPER { return  GameOutcome::DRAW; }
            if p2_move == &Move::SCISSORS { return  GameOutcome::LOSE; }
        },
        &Move::SCISSORS => {
            if p2_move == &Move::ROCK { return GameOutcome::LOSE; }
            if p2_move == &Move::PAPER { return  GameOutcome::WIN; }
            if p2_move == &Move::SCISSORS { return  GameOutcome::DRAW; }
        }
    }
    GameOutcome::DRAW
}

fn char_to_moves() -> HashMap<char, Move> {
    let mut scores = HashMap::new();
    scores.insert('X', Move::ROCK);
    scores.insert('Y', Move::PAPER);
    scores.insert('Z', Move::SCISSORS);
    scores.insert('A', Move::ROCK);
    scores.insert('B', Move::PAPER);
    scores.insert('C', Move::SCISSORS);
    scores
}

fn shape_scores() -> HashMap<Move, i32> {
    let mut scores = HashMap::new();
    scores.insert(Move::ROCK, 1);
    scores.insert(Move::PAPER, 2);
    scores.insert(Move::SCISSORS, 3);
    scores
}

fn game_outcomes_scores() -> HashMap<GameOutcome, i32> {
    let mut scores = HashMap::new();
    scores.insert(GameOutcome::LOSE, 0);
    scores.insert(GameOutcome::WIN, 6);
    scores.insert(GameOutcome::DRAW, 3);
    scores
}
