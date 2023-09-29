use std::{fmt, f32::consts::PI, num, convert::Infallible};
//use colored::Colorize;

static mut CURR_LEGAL_MOVES: Vec<String> = vec![];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
    Checkmate
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Colour {
    White, Black
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn
}

struct Piece {
    colour: Colour,
    piece_type: PieceType,
    two_step: u8,
}

impl Clone for Piece {
    fn clone(&self) -> Self {
        Piece {
            colour: self.colour,
            piece_type: self.piece_type,
            two_step: self.two_step,
        }
    }
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

#[derive(Clone)]
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
        
        //start_board[24] = Some(Piece::new(Colour::White, PieceType::Pawn, 1));

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
        let act_colour: Colour;
        let n_act_colour: Colour;

        match self.active_colour {
            Colour::Black => {
                act_colour = Colour::Black;
                n_act_colour = Colour::White;
            }
            Colour::White => {
                act_colour = Colour::White;
                n_act_colour = Colour::Black;
            }
        }

        if self.get_game_state() == GameState::InProgress {

            let poss_moves = self.get_possible_moves(_from).unwrap();
            if let Some(pp) = &self.board[self.fr_to_index(&_from)] {
                if poss_moves.contains(&_to.to_string()) && pp.colour == act_colour {
                    let from_index = self.fr_to_index(_from);
                    let to_index = self.fr_to_index(_to);
                    

                    if let Some(piece) = self.board[from_index].take() {
                        self.board[to_index] = Some(piece);
                        self.board[from_index] = None;
                    }

                    if self.check_check(act_colour).is_none() {
                        self.active_colour = n_act_colour;
                        unsafe { CURR_LEGAL_MOVES = vec![] }
                        return Some(GameState::InProgress);
                    } else if self.check_check(act_colour).is_some() {
                        if self.check_check(act_colour).unwrap().len() == 0 {
                            return Some(GameState::Checkmate);
                        } else {
                            unsafe { CURR_LEGAL_MOVES = self.check_check(act_colour).unwrap() }
                            self.active_colour = n_act_colour;
                            return Some(GameState::Check);
                        }
                    }
                } else {
                    unsafe { CURR_LEGAL_MOVES = vec![] }
                    return Some(GameState::InProgress);
                }
        }

        } else if self.get_game_state() == GameState::Check {
            let poss_moves = unsafe { &CURR_LEGAL_MOVES };

            if let Some(pp) = &self.board[self.fr_to_index(&_from)] {
                if poss_moves.contains(&_to.to_string()) && pp.colour == act_colour {

                    let from_index = self.fr_to_index(_from);
                    let to_index = self.fr_to_index(_to);

                    if let Some(piece) = self.board[from_index].take() {
                        self.board[to_index] = Some(piece);
                        self.board[from_index] = None;
                    }

                    if self.check_check(act_colour).is_none() {
                        self.active_colour = n_act_colour;
                        unsafe { CURR_LEGAL_MOVES = vec![] }
                        return Some(GameState::InProgress);
                    } else if self.check_check(act_colour).is_some() {
                        if self.check_check(act_colour).unwrap().len() == 0 {
                            return Some(GameState::Checkmate);
                        } else {
                            unsafe { CURR_LEGAL_MOVES = self.check_check(act_colour).unwrap() }
                            self.active_colour = n_act_colour;
                            return Some(GameState::Check);
                        }
                    }

                } else {
                    return Some(GameState::Check);
                }
            }
        } else if self.get_game_state() == GameState::Checkmate {
            return Some(GameState::Checkmate);
        }

        Some(self.get_game_state())
    }

    fn make_move_nbs(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        let mut gamestate = GameState::InProgress;

        if self.get_game_state() == GameState::InProgress {

            let poss_moves = self.get_possible_moves(_from).unwrap();
            if poss_moves.contains(&_to.to_string()) {
                let from_index = self.fr_to_index(_from);
                let to_index = self.fr_to_index(_to);

                let act_colour: Colour;
                let n_act_colour: Colour;
                

                if let Some(piece) = self.board[from_index].take() {
                    self.board[to_index] = Some(piece);
                    self.board[from_index] = None;
                }

                match self.active_colour {
                    Colour::Black => {
                        act_colour = Colour::Black;
                        n_act_colour = Colour::White;
                    }
                    Colour::White => {
                        act_colour = Colour::White;
                        n_act_colour = Colour::Black;
                    }
                }
                return Some(gamestate);
            }

        }

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

    pub fn step_real(&self, _pos: usize, _diffy: isize, _diffx: isize) -> Option<usize> {

        let pos_in_x = _pos % 8;
        let pos_in_y = _pos / 8;

        if (pos_in_x as isize) + _diffx >= 0 && (pos_in_x as isize) + _diffx < 8 {
            if (pos_in_y as isize) + _diffy >= 0 && (pos_in_y as isize) + _diffy < 8 {
                let new_index = ((pos_in_y as isize + _diffy) * 8 + (pos_in_x as isize + _diffx)).abs() as usize;
                return Some(new_index);
            }
        }

        return None;
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

                        if self.step_real(pos_index, -1, 0).is_some() {
                            if self.board[self.step_real(pos_index, -1, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 0).unwrap()).to_string());
                                if self.step_real(pos_index, -2, 0).is_some() {
                                    if self.board[self.step_real(pos_index, -2, 0).unwrap()].is_none() {
                                        move_list.push(self.index_to_fr(self.step_real(pos_index, -2, 0).unwrap()).to_string());
                                    }
                                }
                            }
                        }

                        if self.step_real(pos_index, -1, -1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, -1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -1, -1).unwrap()).to_string());
                                }
                            }
                        }

                        if self.step_real(pos_index, -1, 1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, 1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 1).unwrap()).to_string());
                                }
                            }
                        }
                    } else {

                        if self.step_real(pos_index, -1, 0).is_some() {
                            if self.board[self.step_real(pos_index, -1, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 0).unwrap()).to_string());
                            }
                        }

                        if self.step_real(pos_index, -1, -1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, -1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -1, -1).unwrap()).to_string());
                                }
                            }
                        }

                        if self.step_real(pos_index, 1, 1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, 1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 1).unwrap()).to_string());
                                }
                            }
                        }
                    },
                    
                    Colour::Black => if piece.two_step == 1 {

                        if self.step_real(pos_index, 1, 0).is_some() {
                            if self.board[self.step_real(pos_index, 1, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 0).unwrap()).to_string());
                                if self.step_real(pos_index, 2, 0).is_some() {
                                    if self.board[self.step_real(pos_index, 2, 0).unwrap()].is_none() {
                                        move_list.push(self.index_to_fr(self.step_real(pos_index, 2, 0).unwrap()).to_string());
                                    }
                                }
                            }
                        }

                        if self.step_real(pos_index, 1, -1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, -1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 1, -1).unwrap()).to_string());
                                }
                            }
                        }

                        if self.step_real(pos_index, 1, 1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, 1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 1).unwrap()).to_string());
                                }
                            }
                        }
                    } else {

                        if self.step_real(pos_index, 1, 0).is_some() {
                            if self.board[self.step_real(pos_index, 1, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 0).unwrap()).to_string());
                            }
                        }

                        if self.step_real(pos_index, 1, -1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, -1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 1, -1).unwrap()).to_string());
                                }
                            }
                        }

                        if self.step_real(pos_index, 1, 1).is_some() {
                            if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, 1).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 1).unwrap()).to_string());
                                }
                            }
                        }
                    },
                }
                PieceType::King => {
                    if self.step_real(pos_index, -1, 0).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, 0).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 0).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -1, 0).is_some() {
                        if self.board[self.step_real(pos_index, -1, 0).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 0).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, -1, 1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, 1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -1, 1).is_some() {
                        if self.board[self.step_real(pos_index, -1, 1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 0, 1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 0, 1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 0, 1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 0, 1).is_some() {
                        if self.board[self.step_real(pos_index, 0, 1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 0, 1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 1, 1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, 1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 1, 1).is_some() {
                        if self.board[self.step_real(pos_index, 1, 1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 1, 0).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, 0).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 0).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 1, 0).is_some() {
                        if self.board[self.step_real(pos_index, 1, 0).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 0).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 1, -1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, -1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, -1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 1, -1).is_some() {
                        if self.board[self.step_real(pos_index, 1, -1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 1, -1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 0, -1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 0, -1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 0, -1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 0, -1).is_some() {
                        if self.board[self.step_real(pos_index, 0, -1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 0, -1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, -1, -1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, -1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, -1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -1, -1).is_some() {
                        if self.board[self.step_real(pos_index, -1, -1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -1, -1).unwrap()).to_string());
                        }
                    }
                },
                PieceType::Queen => {
                    for i in 1..=8 {
                        if self.step_real(pos_index, i, i).is_some() {
                            if self.board[self.step_real(pos_index, i, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, i, -i).is_some() {
                            if self.board[self.step_real(pos_index, i, -i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, -i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, -i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, -i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, -i, i).is_some() {
                            if self.board[self.step_real(pos_index, -i, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -i, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, -i, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -i, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, -i, -i).is_some() {
                            if self.board[self.step_real(pos_index, -i, -i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -i, -i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, -i, -i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -i, -i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, i, 0).is_some() {
                            if self.board[self.step_real(pos_index, i, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, 0).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in (-8..=-1).rev() {
                        if self.step_real(pos_index, i, 0).is_some() {
                            if self.board[self.step_real(pos_index, i, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, 0).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, 0, i).is_some() {
                            if self.board[self.step_real(pos_index, 0, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, 0, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in (-8..=-1).rev() {
                        if self.step_real(pos_index, 0, i).is_some() {
                            if self.board[self.step_real(pos_index, 0, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, 0, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                },
                PieceType::Rook => {
                    for i in 1..=8 {
                        if self.step_real(pos_index, i, 0).is_some() {
                            if self.board[self.step_real(pos_index, i, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, 0).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in (-8..=-1).rev() {
                        if self.step_real(pos_index, i, 0).is_some() {
                            if self.board[self.step_real(pos_index, i, 0).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, 0).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, 0).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, 0, i).is_some() {
                            if self.board[self.step_real(pos_index, 0, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, 0, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in (-8..=-1).rev() {
                        if self.step_real(pos_index, 0, i).is_some() {
                            if self.board[self.step_real(pos_index, 0, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, 0, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, 0, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                },
                PieceType::Bishop => {
                    for i in 1..=8 {
                        if self.step_real(pos_index, i, i).is_some() {
                            if self.board[self.step_real(pos_index, i, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, i, -i).is_some() {
                            if self.board[self.step_real(pos_index, i, -i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, i, -i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, i, -i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, i, -i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, -i, i).is_some() {
                            if self.board[self.step_real(pos_index, -i, i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -i, i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, -i, i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -i, i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    for i in 1..=8 {
                        if self.step_real(pos_index, -i, -i).is_some() {
                            if self.board[self.step_real(pos_index, -i, -i).unwrap()].is_none() {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -i, -i).unwrap()));
                            } else if let Some(att_piece) = &self.board[self.step_real(pos_index, -i, -i).unwrap()] {
                                if att_piece.colour != piece.colour {
                                    move_list.push(self.index_to_fr(self.step_real(pos_index, -i, -i).unwrap()).to_string());
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                },
                PieceType::Knight => {
                    if self.step_real(pos_index, -2, -1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -2, -1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -2, -1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -2, -1).is_some() {
                        if self.board[self.step_real(pos_index, -2, -1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -2, -1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, -2, 1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -2, 1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -2, 1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -2, 1).is_some() {
                        if self.board[self.step_real(pos_index, -2, 1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -2, 1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, -1, -2).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, -2).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, -2).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -1, -2).is_some() {
                        if self.board[self.step_real(pos_index, -1, -2).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -1, -2).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 1, -2).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, -2).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, -2).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 1, -2).is_some() {
                        if self.board[self.step_real(pos_index, 1, -2).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 1, -2).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, -1, 2).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, -1, 2).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 2).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, -1, 2).is_some() {
                        if self.board[self.step_real(pos_index, -1, 2).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, -1, 2).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 1, 2).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 1, 2).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 2).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 1, 2).is_some() {
                        if self.board[self.step_real(pos_index, 1, 2).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 1, 2).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 2, -1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 2, -1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 2, -1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 2, -1).is_some() {
                        if self.board[self.step_real(pos_index, 2, -1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 2, -1).unwrap()).to_string());
                        }
                    }

                    if self.step_real(pos_index, 2, 1).is_some() {
                        if let Some(att_piece) = &self.board[self.step_real(pos_index, 2, 1).unwrap()] {
                            if att_piece.colour != piece.colour {
                                move_list.push(self.index_to_fr(self.step_real(pos_index, 2, 1).unwrap()).to_string());
                            }
                        } 
                    }
                    if self.step_real(pos_index, 2, 1).is_some() {
                        if self.board[self.step_real(pos_index, 2, 1).unwrap()].is_none() {
                            move_list.push(self.index_to_fr(self.step_real(pos_index, 2, 1).unwrap()).to_string());
                        }
                    }

                },

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
        board_string.push_str(&format!("Gamestate: {:?} \n", self.get_game_state()));

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

    fn check_check (&self, lastturn: Colour) -> Option<Vec<String>> {

        // None for InProgress, Vector with items for check with legal moves, empty Vector for Checkmate

        let mut king_loc1: String = "a0".to_string();

        let mut poss_moves1: Vec<String> = vec![];

        let mut legal_moves: Vec<String> = vec![];

        for i in 0..64 {
            if let Some(piece) = &self.board[i] {
                if piece.colour != lastturn {
                    king_loc1 = self.index_to_fr(i);
                    break;
                }
            }
        }

        for i in 0..64 {
            if let Some(piece) = &self.board[i] {
                if piece.colour == lastturn {
                    poss_moves1.append(&mut self.get_possible_moves(&self.index_to_fr(i)).unwrap());
                }
            }
        }

        if poss_moves1.contains(&king_loc1) {
            for i in 0..64 {
                if let Some(piece) = &self.board[i] {
                    if piece.colour != lastturn {
                        let poss_moves2: Vec<String> = self.get_possible_moves(&self.index_to_fr(i)).unwrap();

                        for y in poss_moves2 {
                            let mut temp_game = self.clone();
                            temp_game.make_move_nbs(&temp_game.index_to_fr(i), &y);

                            let mut king_loc2: String = "a0".to_string();

                            for h in 0..64 {
                                if let Some(piece) = &temp_game.board[h] {
                                    if piece.colour != lastturn {
                                        king_loc2 = temp_game.index_to_fr(h);
                                        break;
                                    }
                                }
                            }

                            for g in 0..64 {
                                if let Some(piece2) = &temp_game.board[g] {
                                    if piece2.colour == lastturn {
                                        if !temp_game.get_possible_moves(&temp_game.index_to_fr(g)).unwrap().contains(&king_loc2) {
                                            legal_moves.push(y.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return Some(legal_moves);
        } else {
            return None;
        }
    }

}

/// Implement print routine for Game.
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let board_string = self.get_board_rep_str();
        println!("{:?}", self.get_possible_moves("d1"));
        println!("{}", self.index_to_fr(62).to_string());

        write!(f, "{}", board_string)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use crate::CURR_LEGAL_MOVES;
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

    #[test]
    fn checkmatecheck() {
        let mut game = Game::new();
        let mut slay: bool = false;
        game.make_move("f2", "f3");
        game.make_move("e7", "e6");
        game.make_move("g2", "g4");
        game.make_move("d8", "h4");
        println!("{}", game.get_board_rep_str());
        println!("{:?}", unsafe {&CURR_LEGAL_MOVES});
        assert_eq!(game.get_game_state(), GameState::Checkmate);
    }

    
}