#![no_std]

use gtest::{Program, ProgramBuilder, System};
use game_session_io::*;

const USER1: u64 = 10;
const SESSION_PROGRAM_ID: u64 = 1;
const WORDLE_PROGRAM_ID: u64 = 2;

#[test]
fn test_game_start() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
        .with_id(SESSION_PROGRAM_ID)
        .build(&system);

    let wordle_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
        .with_id(WORDLE_PROGRAM_ID)
        .build(&system);

    let init_wordle_program_result = wordle_program.send_bytes(USER1, []);
    assert!(!init_wordle_program_result.main_failed());

    let init_session_program_result = session_program.send(USER1, wordle_program.id());
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "helle".into() });

     let state: Session = session_program.read_state(()).unwrap();
     assert_eq!(state.session_status, SessionStatus::Waiting);
}

#[test]
fn test_game_check() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
        .with_id(SESSION_PROGRAM_ID)
        .build(&system);

    let wordle_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
        .with_id(WORDLE_PROGRAM_ID)
        .build(&system);

    let init_wordle_program_result = wordle_program.send_bytes(USER1, []);
    assert!(!init_wordle_program_result.main_failed());

    let init_session_program_result = session_program.send(USER1, wordle_program.id());
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());


    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "homan".into() });


    let state: Session = session_program.read_state(()).unwrap();
    assert_eq!(state.session_status, SessionStatus::GameEnded { result: GameResult::Lose });
}

#[test]
fn test_game_win() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
        .with_id(SESSION_PROGRAM_ID)
        .build(&system);

    let wordle_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
        .with_id(WORDLE_PROGRAM_ID)
        .build(&system);

    let init_wordle_program_result = wordle_program.send_bytes(USER1, []);
    assert!(!init_wordle_program_result.main_failed());

    let init_session_program_result = session_program.send(USER1, wordle_program.id());
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());


    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "human".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "house".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "horse".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "human".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "house".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "horse".into() });
    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "horse".into() });

    let state: Session = session_program.read_state(()).unwrap();
    assert_eq!(state.session_status, SessionStatus::GameEnded { result: GameResult::Win });
}


#[test]
fn test_timeout() {
    let system = System::new();
    system.init_logger();

    let session_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
        .with_id(SESSION_PROGRAM_ID)
        .build(&system);

    let wordle_program: Program = ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
        .with_id(WORDLE_PROGRAM_ID)
        .build(&system);

    let init_wordle_program_result = wordle_program.send_bytes(USER1, []);
    assert!(!init_wordle_program_result.main_failed());

    let init_session_program_result = session_program.send(USER1, wordle_program.id());
    assert!(!init_session_program_result.main_failed());

    let start_result = session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    assert!(!start_result.main_failed());

    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "hello".into() });

    system.spend_blocks(210);

    session_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: "hello".into() });

    let state: Session = session_program.read_state(()).unwrap();
    assert_eq!(state.session_status, SessionStatus::GameEnded { result: GameResult::Lose });
}
