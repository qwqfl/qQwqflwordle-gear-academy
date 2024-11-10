#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, PartialEq};

pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<SessionAction, SessionEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Session>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
    CheckGameStatus { user: ActorId },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionEvent {
    GameStarted { user: ActorId },
    WordChecked { user: ActorId, correct_positions: Vec<u8>, contained_in_word: Vec<u8> },
    GameStatus(GameStatus),
    GameError(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Session {
    pub target_program_id: ActorId,
    pub session_status: SessionStatus,
    pub start_block_height: u32,
    pub attempts : u8,
}


#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameStatus {
    pub game_result: Option<GameResult>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum SessionStatus {
    Waiting,
    MessageSent,
    MessageReceived(Event),
    GameEnded { result: GameResult },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum Event {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}