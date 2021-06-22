extern crate pleco;
use rand::Rng;

use pleco::{Board};

enum StartColor {
    White,
    Black
}

enum PlayMode {
    Human,
    Computer
}

enum TurnEnt {
    Human,
    Computer
}

fn main() {
    match start_menu() {
        PlayMode::Computer => {computer_game();}
        PlayMode::Human => {human_game();}
    }
}

//Menu for choosing if the user wants to play against a computer or human (human not implemented yet)
fn start_menu() -> PlayMode {
    println!("Play against a human or computer? [1/2]");
    println!("1) Computer");
    println!("2) Human");

    let mut player_input = String::new();
    std::io::stdin().read_line(&mut player_input).unwrap();

    match player_input.trim_end() {
        "1" => PlayMode::Computer,
        "2" => PlayMode::Human,
        _   => {
            println!("Invalid option!");
            start_menu()
        }
    }
}

//Menu for choosing which color the user wants to play as
fn color_menu() -> StartColor {
    println!("What color do you want to play? [w/b]");

    let mut player_input = String::new();
    std::io::stdin().read_line(&mut player_input).unwrap();

    match player_input.trim_end() {
        "w" => StartColor::White,
        "b" => StartColor::Black,
        _   => {
            println!("Invalid option!");
            color_menu()
        }
    }
}

//Query move from user
fn query_move() -> String {
    println!("Your move: ");
    let mut player_input = String::new();
    std::io::stdin().read_line(&mut player_input).unwrap();
    player_input.trim_end().to_lowercase()
}

//start a computer game
fn computer_game() {
    let start_color = color_menu();
    let mut board = Board::start_pos();

    print_board(&board.fen());

    match start_color {
        StartColor::White => {
            while true {
                if make_move(&mut board, TurnEnt::Human) {break};
                if make_move(&mut board, TurnEnt::Computer) {break};
            }
        }
        StartColor::Black => {
            while true {
                if make_move(&mut board, TurnEnt::Computer) {break};
                if make_move(&mut board, TurnEnt::Human) {break};
            }
        }
    }
}

//start a game with two humans
fn human_game() {
    let mut board = Board::start_pos();
    print_board(&board.fen());

    while true {
        if make_move(&mut board, TurnEnt::Human) {break}
        if make_move(&mut board, TurnEnt::Human) {break}
    }
}

//Check if a player won or a stalemate was achieved
fn check_winning_conditions(board : &mut Board) -> bool {
    if board.stalemate() {
        println!("Stalemate!");
        true
    } else if board.checkmate() {
        println!("Checkmate! {} won!", board.turn().other_player());
        true
    } else {
        false
    }
}

//Make a move based on the current player
fn make_move(board : &mut Board, entity : TurnEnt) -> bool {
    println!("Current player: {}", board.turn());
    match entity {
        TurnEnt::Computer => {computer_move(board);}
        TurnEnt::Human    => {
            let player_move = query_move();
        
            let mut success = board.apply_uci_move(&player_move);
            while !success {
                println!("Invalid move!");
                success = board.apply_uci_move(&query_move());
            }
        }
    }
    print_board(&board.fen());
    check_winning_conditions(board)
}

//Let the computer make a move (right now only a random valid move)
fn computer_move(board : &mut Board) {
    let moves = board.generate_moves();
    let vec_moves = moves.vec();

    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..vec_moves.len());

    board.apply_move(vec_moves[rand]);
    println!("Computer move: {}", vec_moves[rand].stringify());
    print_board(&board.fen());
}

//Print the board to the console
fn print_board(position : &str) {
    let mut rows : Vec<&str> = position.split("/").collect();
    let temp : Vec<&str> = rows[7].split(" ").collect();
    rows[7] = temp[0];
    let mut i = 8;

    println!("");
    for s in rows {
        print!("{}  ", i);
        i -= 1;
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    let n = c.to_digit(10).unwrap();
                    for i in 0..n {
                        print!(" -");
                    }
                }
                _ => {
                    print!(" {}", c);
                }
            }
        }
        print!("\n");
    }

    println!("\n    a b c d e f g h \n");
}
