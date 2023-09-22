use std::{fmt, f32::consts::PI, num};
//use colored::Colorize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
    Checkmate
}

#[derive(Clone, PartialEq, Eq)]
enum Colour {
    White, Black
}

#[derive(PartialEq, Eq)]
enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn
}

struct Piece {
    colour: Colour,
    piece_type: PieceType,
    two_step: u8,
}

impl Piece {
    fn new(colour: Colour, piece_type: PieceType, two_step: u8) -> Self {
        Piece {
            colour,
            piece_type,
            two_step,
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
        

        start_board[0] = Some(Piece::new(Colour::Black, PieceType::Rook, 0));
        start_board[1] = Some(Piece::new(Colour::Black, PieceType::Knight, 0));
        start_board[2] = Some(Piece::new(Colour::Black, PieceType::Bishop, 0));
        start_board[3] = Some(Piece::new(Colour::Black, PieceType::Queen, 0));
        start_board[4] = Some(Piece::new(Colour::Black, PieceType::King, 0));
        start_board[5] = Some(Piece::new(Colour::Black, PieceType::Bishop, 0));
        start_board[6] = Some(Piece::new(Colour::Black, PieceType::Knight, 0));
        start_board[7] = Some(Piece::new(Colour::Black, PieceType::Rook, 0));

        for i in 8..16 {
            start_board[i] = Some(Piece::new(Colour::Black, PieceType::Pawn, 1));
        }
        

        start_board[56] = Some(Piece::new(Colour::White, PieceType::Rook, 0));
        start_board[57] = Some(Piece::new(Colour::White, PieceType::Knight, 0));
        start_board[58] = Some(Piece::new(Colour::White, PieceType::Bishop, 0));
        start_board[59] = Some(Piece::new(Colour::White, PieceType::Queen, 0));
        start_board[60] = Some(Piece::new(Colour::White, PieceType::King, 0));
        start_board[61] = Some(Piece::new(Colour::White, PieceType::Bishop, 0));
        start_board[62] = Some(Piece::new(Colour::White, PieceType::Knight, 0));
        start_board[63] = Some(Piece::new(Colour::White, PieceType::Rook, 0));

        for i in 48..56 {
            start_board[i] = Some(Piece::new(Colour::White, PieceType::Pawn, 1));
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
        

        Some(GameState::InProgress)
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    pub fn fr_to_index(&self, _postion: &str) -> usize {
        let chars: Vec<char> = _postion.chars().collect();

        let letter = chars[0];
        let number = chars[1] as usize - '0' as usize;

        let letter_value = match letter {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => 0,
        };
        let number_value = match number {
            1 => 7,
            2 => 6,
            3 => 5,
            4 => 4,
            5 => 3,
            6 => 2,
            7 => 1,
            8 => 0,
            _ => 0,
        };

        let pos_index = number_value * 8 + letter_value;

        pos_index
    }

    pub fn index_to_fr(&self, _postion: usize) -> String {
        let letter = match _postion % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => 'a',
        };
        let number = 8 - (_postion / 8);

        let result = format!("{}{}", letter, number);

        result
    }

    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        let mut move_list: Vec<String> = Vec::new();

        let pos_index = self.fr_to_index(_postion); 

        if let Some(piece) = &self.board[pos_index] {
            match piece.piece_type {
                PieceType::Pawn => match piece.colour {
                    Colour::White => if piece.two_step == 1 {

                        if pos_index >= 8 {
                            if self.board[pos_index - 8].is_none() {
                                move_list.push(self.index_to_fr(pos_index - 8).to_string());
                            }
                        }
                        if pos_index >= 8*2 {
                            if self.board[pos_index - 8*2].is_none() {
                                move_list.push(self.index_to_fr(pos_index - 8*2).to_string());
                            }
                        }

                        if pos_index >= 9 {
                            if let Some(att_piece) = &self.board[pos_index - 9] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index - 9).to_string());
                                }
                            }
                        }

                        if pos_index >= 7 {
                            if let Some(att_piece) = &self.board[pos_index - 7] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index - 7).to_string());
                                }
                            }
                        }
                    } else {

                        if pos_index >= 8 {
                            if self.board[pos_index - 8].is_none() {
                                move_list.push(self.index_to_fr(pos_index - 8).to_string());
                            }
                        }

                        if pos_index >= 9 {
                            if let Some(att_piece) = &self.board[pos_index - 9] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index - 9).to_string());
                                }
                            }
                        }

                        if pos_index >= 7 {
                            if let Some(att_piece) = &self.board[pos_index - 7] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index - 7).to_string());
                                }
                            }
                        }
                    },
                    
                    Colour::Black => if piece.two_step == 1 {

                        if pos_index + 8 <= 63 {
                            if self.board[pos_index + 8].is_none() {
                                move_list.push(self.index_to_fr(pos_index + 8).to_string());
                            }
                        }
                        if pos_index + 8*2 <= 63 {
                            if self.board[pos_index + 8*2].is_none() {
                                move_list.push(self.index_to_fr(pos_index + 8*2).to_string());
                            }
                        }

                        if pos_index + 9 <= 63 {
                            if let Some(att_piece) = &self.board[pos_index + 9] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index + 9).to_string());
                                }
                            }
                        }

                        if pos_index + 7 <= 63 {
                            if let Some(att_piece) = &self.board[pos_index - 7] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index - 7).to_string());
                                }
                            }
                        }

                    } else {

                        if pos_index + 8 <= 63 {
                            if self.board[pos_index + 8].is_none() {
                                move_list.push(self.index_to_fr(pos_index + 8).to_string());
                            }
                        }

                        if pos_index + 9 <= 63 {
                            if let Some(att_piece) = &self.board[pos_index + 9] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index + 9).to_string());
                                }
                            }
                        }

                        if pos_index + 7 <= 63 {
                            if let Some(att_piece) = &self.board[pos_index - 7] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(pos_index - 7).to_string());
                                }
                            }
                        }
                    },
                }
                PieceType::King => println!("king was chosen"),
                PieceType::Queen => (),
                PieceType::Rook => (),
                PieceType::Bishop => (),
                PieceType::Knight => (),

            };
        };

        Some(move_list)
    }

    pub fn get_board_rep_str (&self) -> String {
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
                if let Some(piece) = &self.board[index] {
                    let piece_str = match piece.colour {
                        Colour::Black => match piece.piece_type {
                            PieceType::King => "B♔ ",
                            PieceType::Queen => "B♕ ",
                            PieceType::Rook => "B♖ ",
                            PieceType::Bishop => "B♗ ",
                            PieceType::Knight => "B♘ ",
                            PieceType::Pawn => "B♙ ",
                        },
                        Colour::White => match piece.piece_type {
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
        board_string
    }
}

/// Implement print routine for Game.
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        let board_string = self.get_board_rep_str();
        println!("{:?}", self.get_possible_moves("e7"));
        println!("{}", self.fr_to_index("f3").to_string());

        write!(f, "{}", board_string)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use crate::PieceType;

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

    #[test]
    fn recog_empty_space() {
        let game = Game::new();
        let mut slay: bool = false;
        if game.board[32].is_none() {
            slay = true;
        }
        assert_eq!(slay, true);
    }

    
}