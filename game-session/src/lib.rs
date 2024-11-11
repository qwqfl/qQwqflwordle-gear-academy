#![no_std]

use gstd::{debug, exec, msg, prelude::*};
use game_session_io::*;

static mut SESSION: Option<Session> = None;

#[no_mangle]
extern "C" fn init() {
    debug!("===INIT===");
    let target_program_id = msg::load().expect("Unable to decode Init");

    let session = Session {
        target_program_id,
        session_status: SessionStatus::Waiting,
        start_block_height: exec::block_height(),
        attempts: 5,
    };

    save_session(session);
}

#[no_mangle]
extern "C" fn handle() {
    let mut session = load_session();
    let action: SessionAction = msg::load().expect("Unable to decode `Action`");

    match &session.session_status {
        SessionStatus::Waiting => {
            handle_waiting_state(&mut session, action);
        }
        SessionStatus::MessageSent => {
            debug!("===MESSAGE SENT===");
            msg::reply(SessionEvent::GameError("Message has already been sent, restart the game".into()), 0)
                .expect("Error in sending a reply");
        }
        SessionStatus::MessageReceived(ref event) => {
            let event = event.clone();
            handle_message_received(&mut session, &event);
        }
        SessionStatus::GameEnded { ref result } => {
            debug!("===GAME ENDED: {:?}===", result);
        }
    }

    save_session(session);
    debug!("===HANDLE ENDED===");
}

fn handle_waiting_state(session: &mut Session, action: SessionAction) {
    match action {
        SessionAction::StartGame { user } => {
            debug!("===WAITING AND START GAME===");

            let current_block_height = exec::block_height();
            session.start_block_height = current_block_height;

            msg::send(session.target_program_id, Action::StartGame { user }, 0)
                .expect("Error in sending a message");
            session.session_status = SessionStatus::MessageSent;
            exec::wait();
        }
        SessionAction::CheckWord { user, word } => {
            if matches!(session.session_status, SessionStatus::GameEnded { .. }) {
                msg::reply(SessionEvent::GameError("Game has already ended".into()), 0)
                    .expect("Error in sending a reply");
                return;
            }

            debug!("===CHECK WORD FOR USER: {:?}===", user);

            if session.attempts == 0 {
                msg::reply(SessionEvent::GameError("No more attempts left, game over".into()), 0)
                    .expect("Error in sending a reply");
                session.session_status = SessionStatus::GameEnded { result: GameResult::Lose };
                save_session(session.clone());
                return;
            }

            let current_game_status = get_game_status(&session);
            debug!("===CHECK WORD FOR current_game_status: {:?}===", current_game_status);

            if let Some(_) = current_game_status.game_result {
                msg::reply(SessionEvent::GameStatus(current_game_status), 0)
                    .expect("Unable to reply");
            } else {
                msg::send(session.target_program_id, Action::CheckWord { user, word }, 0)
                    .expect("Error in sending a message");

                session.attempts -= 1;
                session.session_status = SessionStatus::MessageSent;
                save_session(session.clone());
                debug!("====handle_waiting_state=== :{:?}", session.attempts);

                exec::wait();
            }
        }
        SessionAction::CheckGameStatus { user: _ } => {
            debug!("===CHECK GAME STATUS===");

            if has_game_lost(&session) {
                session.session_status = SessionStatus::GameEnded { result: GameResult::Lose };
                save_session(session.clone());
                msg::reply(SessionEvent::GameStatus(GameStatus { game_result: Some(GameResult::Lose) }), 0)
                    .expect("Unable to reply");
                debug!("===GAME LOST, 200 BLOCKS PASSED, UPDATING STATUS TO LOSE===");
            } else {
                let current_game_status = get_game_status(&session);
                msg::reply(SessionEvent::GameStatus(current_game_status), 0)
                    .expect("Unable to reply");
            }
        }
    }
}



fn handle_message_received(session: &mut Session, event: &Event) {
    debug!("===MESSAGE RECEIVED===");

    if matches!(session.session_status, SessionStatus::GameEnded { .. }) {
        debug!("Game has already ended, ignoring the message.");
        return;
    }

    match event {
        Event::GameStarted { user } => {
            debug!("===GAME STARTED FOR USER: {:?}===", user);
            msg::send_delayed(exec::program_id(), SessionAction::CheckGameStatus { user: *user }, 0, 200)
                .expect("Failed to send delayed message");
            msg::reply(SessionEvent::GameStarted { user: *user }, 0)
                .expect("Error in sending a reply");
        }
        Event::WordChecked { user, correct_positions, contained_in_word } => {
            debug!("===WORD CHECKED ANSWER: {:?}===", correct_positions);

            if correct_positions.len() == 5 {
                session.session_status = SessionStatus::GameEnded { result: GameResult::Win };
                save_session(session.clone());
                debug!("===GAME WON, UPDATING STATUS TO WIN===");

                msg::reply(SessionEvent::GameStatus(GameStatus { game_result: Some(GameResult::Win) }), 0)
                    .expect("Error in sending a reply");
            } else {
                msg::reply(SessionEvent::WordChecked {
                    user: *user,
                    correct_positions: correct_positions.clone(),
                    contained_in_word: contained_in_word.clone(),
                }, 0).expect("Error in sending a reply");
            }
        }
    }

    if !matches!(session.session_status, SessionStatus::GameEnded { .. }) {
        session.session_status = SessionStatus::Waiting;
        save_session(session.clone());
    }

    debug!("===SESSION STATE AFTER HANDLING: {:?}", session.session_status);
}


#[no_mangle]
extern "C" fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };

    let event: Event = msg::load().expect("Unable to decode `Event`");

    session.session_status = SessionStatus::MessageReceived(event);
    debug!("===WAKING UP MESSAGE WITH ID: {:?}===", reply_to);
    exec::wake(reply_to).expect("Failed to wake up the message");

}

#[no_mangle]
extern "C" fn state() {
    let session = load_session();
    msg::reply(session, 0).expect("Unable to get the state");
}

fn get_game_status(session: &Session) -> GameStatus {
    match &session.session_status {
        SessionStatus::GameEnded { result } => GameStatus {
            game_result: Some(result.clone()),
        },
        _ => GameStatus {
            game_result: None,
        },
    }
}

fn save_session(session: Session) {
    unsafe {
        SESSION = Some(session);
    }
}

fn load_session() -> Session {
    unsafe {
        SESSION.as_mut().expect("The session is not initialized").clone()
    }
}

fn has_game_lost(session: &Session) -> bool {
    let current_block_height = exec::block_height();
    let block_difference = current_block_height.saturating_sub(session.start_block_height);

    block_difference >= 200
}

