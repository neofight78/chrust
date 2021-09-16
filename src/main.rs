#![allow(unused)]

mod board;
use std::env;

use board::*;
use board::{cell::color::Color, chessboard::Board};

mod player;
use player::aiplayer::AIPlayer;
use player::chessplayer::ChessPlayer;
use player::humanplayer::HumanPlayer;

mod game;
use game::chessgame::ChessGame;

use crate::board::cell::chesspiece::ChessPiece;
use crate::frontend::server::build_and_run_frontend;

mod chessmove;

mod ai;

mod visualize;
use visualize::visualizer::Visualizer;

mod state;
use state::programstate::{ProgramState, get_args};
use state::viztype::VizType;

mod frontend;
use frontend::server;

mod ext;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate clap;
use clap::{App, Arg};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_state = get_args()?;
    let board = Board::load_from_file("game_start")?;

    let human_player = HumanPlayer {
        name: "kasparov".to_string(),
        color: Color::White,
    };

    let ai_player = AIPlayer {
        name: "rusty".to_string(),
        color: Color::Black,
    };

    let mut game = ChessGame::new(
        human_player,
        ai_player,
        board,
        program_state.human_plays,
        program_state.tick_speed,
    ); 

    match program_state.viz_type {
        VizType::TERM => {
            let winner = game.start_game()?;
            println!("Winner: {}", winner);
            Ok(())
        },
        VizType::GUI => {
            let mut viz = Visualizer::new(game);
            viz.start_viz();
            Ok(())
        },
        VizType::WEB => {
            build_and_run_frontend(game).await;
            Ok(())
        },
    }
}
