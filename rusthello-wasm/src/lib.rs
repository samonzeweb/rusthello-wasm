mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn on_load() {
    utils::set_panic_hook();
    log!("rusthello WASM initialized");
}

use rusthello::Game;
use rusthello::Player;
use rusthello::{AlphaBeta, VirtualPlayer};

#[wasm_bindgen]
pub enum WPlayer {
    Black = 1,
    White = 2,
}

impl From<Player> for WPlayer {
    fn from(player: Player) -> Self {
        match player {
            Player::Black => WPlayer::Black,
            Player::White => WPlayer::White,
        }
    }
}

impl Into<Player> for WPlayer {
    fn into(self) -> Player {
        match self {
            WPlayer::Black => Player::Black,
            WPlayer::White => Player::White,
        }
    }
}

impl WPlayer {
    fn from_option_player(p: Option<Player>) -> Option<WPlayer> {
        match p {
            Some(p) => Some(WPlayer::from(p)),
            None => None,
        }
    }
}

#[wasm_bindgen]
pub struct Coordinates {
    pub x: u8,
    pub y: u8,
}

#[wasm_bindgen]
pub struct WGame {
    game: Game,
    human: Player,
    terminator: Box<dyn VirtualPlayer>,
}

#[wasm_bindgen]
impl WGame {
    pub fn new(p: WPlayer, depth: u8) -> Self {
        Self {
            game: Game::new(),
            human: p.into(),
            terminator: Box::new(AlphaBeta::new(depth)),
        }
    }

    pub fn player(&self) -> Option<WPlayer> {
        WPlayer::from_option_player(self.game.player())
    }

    pub fn opponent_is_blocked(&self) -> bool {
        self.game.opponent_is_blocked()
    }

    pub fn game_over(&self) -> bool {
        self.game.game_over()
    }

    pub fn winner(&self) -> Option<WPlayer> {
        WPlayer::from_option_player(self.game.winner())
    }

    pub fn count_black(&self) -> u8 {
        let (black, _) = self.game.count_pieces();
        black
    }

    pub fn count_white(&self) -> u8 {
        let (_, white) = self.game.count_pieces();
        white
    }

    pub fn player_play(&mut self, x: u8, y: u8) -> Result<(), JsValue> {
        self.game.play(self.human, x, y)?;
        Ok(())
    }

    pub fn computer_play(&mut self) -> Result<Coordinates, JsValue> {
        match self.game.player() {
            None => return Err(JsValue::from_str("Nobody (human or not) can play.")),
            Some(p) if p == self.human => {
                return Err(JsValue::from_str(
                    "It's the turn of the humain player, not the computer one.",
                ))
            }
            _ => {} // it's ok
        }

        let computer_move = self
            .terminator
            .compute_move(self.game.board(), self.human.opponent());

        let (x, y) = match computer_move {
            Some((x, y)) => (x, y),
            None => return Err(JsValue::from_str("Computer didn't find any move.")),
        };

        match self.game.play(self.human.opponent(), x, y) {
            Err(message) => Err(JsValue::from_str(&message)),
            Ok(()) => Ok(Coordinates { x, y }),
        }
    }

    pub fn get_piece(&self, x: u8, y: u8) -> Result<Option<WPlayer>, JsValue> {
        let piece = self.game.board().get_piece(x, y)?;
        Ok(WPlayer::from_option_player(piece))
    }

    pub fn is_move_valid(&self, player: WPlayer, x: u8, y: u8) -> Result<bool, JsValue> {
        match self.game.board().is_move_valid(player.into(), x, y) {
            Err(message) => Err(JsValue::from_str(&message)),
            Ok(result) => Ok(result),
        }
    }

    pub fn log(&self) {
        log!("{}", self.game.board());
    }
}
