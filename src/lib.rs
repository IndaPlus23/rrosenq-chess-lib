use std::fmt;
use colored::Colorize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
    Checkmate
}

#[derive(Clone)]
enum Colour {
    White, Black
}

enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn
}

struct Piece {
    colour: Colour,
    piece_type: PieceType,
}

impl Piece {
    fn new(colour: Colour, piece_type: PieceType) -> Self {
        Piece {
            colour,
            piece_type,
        }
    }
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [Option<Piece>; 64],
    active_colour: Colour,
    //...
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut start_board: [Option<Piece>; 64] = [
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
        ];
        

        start_board[0] = Some(Piece::new(Colour::Black, PieceType::Rook));
        start_board[1] = Some(Piece::new(Colour::Black, PieceType::Knight));
        start_board[2] = Some(Piece::new(Colour::Black, PieceType::Bishop));
        start_board[3] = Some(Piece::new(Colour::Black, PieceType::Queen));
        start_board[4] = Some(Piece::new(Colour::Black, PieceType::King));
        start_board[5] = Some(Piece::new(Colour::Black, PieceType::Bishop));
        start_board[6] = Some(Piece::new(Colour::Black, PieceType::Knight));
        start_board[7] = Some(Piece::new(Colour::Black, PieceType::Rook));

        for i in 8..16 {
            start_board[i] = Some(Piece::new(Colour::Black, PieceType::Pawn));
        }

        start_board[56] = Some(Piece::new(Colour::White, PieceType::Rook));
        start_board[57] = Some(Piece::new(Colour::White, PieceType::Knight));
        start_board[58] = Some(Piece::new(Colour::White, PieceType::Bishop));
        start_board[59] = Some(Piece::new(Colour::White, PieceType::Queen));
        start_board[60] = Some(Piece::new(Colour::White, PieceType::King));
        start_board[61] = Some(Piece::new(Colour::White, PieceType::Bishop));
        start_board[62] = Some(Piece::new(Colour::White, PieceType::Knight));
        start_board[63] = Some(Piece::new(Colour::White, PieceType::Rook));

        for i in 48..56 {
            start_board[i] = Some(Piece::new(Colour::White, PieceType::Pawn));
        }

        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: start_board,
            active_colour: Colour::White,
            //...
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        let mut board_string = String::new();
        let act_colour = match self.active_colour {
            Colour::Black => "Black",
            Colour::White => "White"
        };
        board_string.push('\n');
        board_string.push_str(&format!("Active turn: {} \n", act_colour));

        for rank in 0..8 {
            for file in 0..8 {
                let index = file + rank * 8;
                if let Some(Piece) = &self.board[index] {
                    let piece_str = match Piece.colour {
                        Colour::Black => match Piece.piece_type {
                            PieceType::King => "B♔ ",
                            PieceType::Queen => "B♕ ",
                            PieceType::Rook => "B♖ ",
                            PieceType::Bishop => "B♗ ",
                            PieceType::Knight => "B♘ ",
                            PieceType::Pawn => "B♙ ",
                        },
                        Colour::White => match Piece.piece_type {
                            PieceType::King => "W♚ ",
                            PieceType::Queen => "W♛ ",
                            PieceType::Rook => "W♜ ",
                            PieceType::Bishop => "W♝ ",
                            PieceType::Knight => "W♞ ",
                            PieceType::Pawn => "W♟ ",
                        },
                    };
                    board_string.push_str(&piece_str);
                } else {
                    board_string.push_str(" * ");
                }
            }
            board_string.push('\n');
        }

        write!(f, "{}", board_string)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}