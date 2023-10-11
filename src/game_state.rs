use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    E, // empty
    O, // O
    X, // X
    D, // dead cell (not affect to other cells)
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellState::E => " ",
                CellState::O => "O",
                CellState::X => "X",
                CellState::D => "?",
            }
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct GridNotEndState {
    pub a: CellState,
    pub b: CellState,
    pub c: CellState,
    pub d: CellState,
    pub e: CellState,
    pub f: CellState,
    pub g: CellState,
    pub h: CellState,
    pub i: CellState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GridState {
    O,
    X,
    E(GridNotEndState),
}

#[derive(Clone, Copy)]
pub struct BoardNotEndState {
    pub a: GridState,
    pub b: GridState,
    pub c: GridState,
    pub d: GridState,
    pub e: GridState,
    pub f: GridState,
    pub g: GridState,
    pub h: GridState,
    pub i: GridState,
}

#[derive(Clone)]
pub enum BoardState {
    O,
    X,
    E(BoardNotEndState),
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! cell {
            ($self:expr, $board:ident, $grid:ident) => {
                match $self {
                    BoardState::O => CellState::O,
                    BoardState::X => CellState::X,
                    BoardState::E(board) => match board.$board {
                        GridState::O => CellState::O,
                        GridState::X => CellState::X,
                        GridState::E(grid) => grid.$grid,
                    },
                }
            };
        }
        macro_rules! grid {
            ($self:expr, $board:ident) => {
                [
                    cell!($self, $board, a),
                    cell!($self, $board, b),
                    cell!($self, $board, c),
                    cell!($self, $board, d),
                    cell!($self, $board, e),
                    cell!($self, $board, f),
                    cell!($self, $board, g),
                    cell!($self, $board, h),
                    cell!($self, $board, i),
                ]
            };
        }
        let board = [
            grid!(self, a),
            grid!(self, b),
            grid!(self, c),
            grid!(self, d),
            grid!(self, e),
            grid!(self, f),
            grid!(self, g),
            grid!(self, h),
            grid!(self, i),
        ];
        writeln!(f, "┏━━━┯━━━┯━━━┓")?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[0][0],
            board[0][1],
            board[0][2],
            board[1][0],
            board[1][1],
            board[1][2],
            board[2][0],
            board[2][1],
            board[2][2]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[0][3],
            board[0][4],
            board[0][5],
            board[1][3],
            board[1][4],
            board[1][5],
            board[2][3],
            board[2][4],
            board[2][5]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[0][6],
            board[0][7],
            board[0][8],
            board[1][6],
            board[1][7],
            board[1][8],
            board[2][6],
            board[2][7],
            board[2][8]
        )?;
        writeln!(f, "┠───┼───┼───┨")?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[3][0],
            board[3][1],
            board[3][2],
            board[4][0],
            board[4][1],
            board[4][2],
            board[5][0],
            board[5][1],
            board[5][2]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[3][3],
            board[3][4],
            board[3][5],
            board[4][3],
            board[4][4],
            board[4][5],
            board[5][3],
            board[5][4],
            board[5][5]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[3][6],
            board[3][7],
            board[3][8],
            board[4][6],
            board[4][7],
            board[4][8],
            board[5][6],
            board[5][7],
            board[5][8]
        )?;
        writeln!(f, "┠───┼───┼───┨")?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[6][0],
            board[6][1],
            board[6][2],
            board[7][0],
            board[7][1],
            board[7][2],
            board[8][0],
            board[8][1],
            board[8][2]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[6][3],
            board[6][4],
            board[6][5],
            board[7][3],
            board[7][4],
            board[7][5],
            board[8][3],
            board[8][4],
            board[8][5]
        )?;
        writeln!(
            f,
            "┃{}{}{}│{}{}{}│{}{}{}┃",
            board[6][6],
            board[6][7],
            board[6][8],
            board[7][6],
            board[7][7],
            board[7][8],
            board[8][6],
            board[8][7],
            board[8][8]
        )?;
        writeln!(f, "┗━━━┷━━━┷━━━┛")?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub enum Player {
    O,
    X,
}

#[derive(Clone, PartialEq)]
pub enum GridPosition {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    S,
}

#[derive(Clone)]
pub struct GameState {
    pub board: BoardState,
    pub turn: Player,
    pub position: GridPosition,
}

fn grid_state(state: GridNotEndState) -> GridState {
    macro_rules! grid_state {
        ($state:expr, $a:ident, $b:ident, $c:ident) => {
            if $state.$a == CellState::O && $state.$a == $state.$b && $state.$a == $state.$c {
                return GridState::O;
            }
            if $state.$a == CellState::X && $state.$a == $state.$b && $state.$a == $state.$c {
                return GridState::X;
            }
        };
    }

    grid_state!(state, a, b, c);
    grid_state!(state, d, e, f);
    grid_state!(state, g, h, i);
    grid_state!(state, a, d, g);
    grid_state!(state, b, e, h);
    grid_state!(state, c, f, i);
    grid_state!(state, a, e, i);
    grid_state!(state, c, e, g);
    GridState::E(state)
}

fn is_dead_row(a: CellState, b: CellState, c: CellState) -> bool {
    let has_o = a == CellState::O
        || a == CellState::D
        || b == CellState::O
        || b == CellState::D
        || c == CellState::O
        || c == CellState::D;
    let has_x = a == CellState::X
        || a == CellState::D
        || b == CellState::X
        || b == CellState::D
        || c == CellState::X
        || c == CellState::D;
    has_o && has_x
}

pub fn calculate_dead_cell(state: GridNotEndState) -> GridState {
    let mut result = state;
    macro_rules! check_dead1 {
        ($state:expr, $cell:ident, $($cells:ident),*) => {
            let mut all_dead = true;
            $(
                all_dead = all_dead && $state.$cells != CellState::E;
            )*
            if all_dead {
                $state.$cell = CellState::D;
            }
        };
    }
    macro_rules! check_dead2 {
        ($state:expr, $cell:ident, $a:ident, $b:ident) => {{
            $state.$cell != CellState::E && is_dead_row($state.$cell, $state.$a, $state.$b)
        }};
    }
    check_dead1!(result, a, a, b, c, d, e, g, i);
    if check_dead2!(result, a, b, c)
        && check_dead2!(result, a, d, g)
        && check_dead2!(result, a, e, i)
    {
        result.a = CellState::D;
    }
    check_dead1!(result, b, a, b, c, e, h);
    if check_dead2!(result, b, a, c) && check_dead2!(result, b, e, h) {
        result.b = CellState::D;
    }
    check_dead1!(result, c, a, b, c, e, f, g, i);
    if check_dead2!(result, c, a, b)
        && check_dead2!(result, c, f, i)
        && check_dead2!(result, c, e, g)
    {
        result.c = CellState::D;
    }
    check_dead1!(result, d, a, d, e, f, g);
    if check_dead2!(result, d, e, f) && check_dead2!(result, d, a, g) {
        result.d = CellState::D;
    }
    check_dead1!(result, e, a, b, c, d, e, f, g, h, i);
    if check_dead2!(result, e, d, f)
        && check_dead2!(result, e, b, h)
        && check_dead2!(result, e, a, i)
        && check_dead2!(result, e, c, g)
    {
        result.e = CellState::D;
    }
    check_dead1!(result, f, c, d, e, f, i);
    if check_dead2!(result, f, d, e) && check_dead2!(result, f, c, i) {
        result.f = CellState::D;
    }
    check_dead1!(result, g, a, c, d, e, g, h, i);
    if check_dead2!(result, g, h, i)
        && check_dead2!(result, g, a, d)
        && check_dead2!(result, g, c, e)
    {
        result.g = CellState::D;
    }
    check_dead1!(result, h, b, e, g, h, i);
    if check_dead2!(result, h, g, i) && check_dead2!(result, h, b, e) {
        result.h = CellState::D;
    }
    check_dead1!(result, i, a, c, e, f, g, h, i);
    if check_dead2!(result, i, g, h)
        && check_dead2!(result, i, c, f)
        && check_dead2!(result, i, a, e)
    {
        result.i = CellState::D;
    }
    grid_state(result)
}

const BASE64_CHARS: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];

fn pack_bools_to_base64(bools: &[bool]) -> String {
    let mut result = String::new();
    let mut current_char = 0u8;
    let mut remaining_bits = 0;
    for &b in bools {
        let bit = if b { 1u8 } else { 0u8 };
        current_char |= bit << (5 - remaining_bits);
        remaining_bits += 1;
        if remaining_bits == 6 {
            result.push(char::from(BASE64_CHARS[current_char as usize]));
            current_char = 0;
            remaining_bits = 0;
        }
    }
    if remaining_bits > 0 {
        result.push(char::from(BASE64_CHARS[current_char as usize]));
    }
    result
}

fn unpack_base64_to_bools(encoded: &str) -> Vec<bool> {
    let mut bools = Vec::new();
    let mut current_char = 0u8;
    let mut remaining_bits = 0;
    for &c in encoded.as_bytes() {
        let index = BASE64_CHARS.iter().position(|&x| x == c as u8).unwrap_or(0);
        current_char = (current_char << 6) | (index as u8);
        remaining_bits += 6;
        while remaining_bits >= 1 {
            let bit = ((current_char >> (remaining_bits - 1)) & 1) == 1;
            bools.push(bit);
            remaining_bits -= 1;
        }
    }
    bools
}

fn pack_grid_position_to_bools(position: &GridPosition) -> [bool; 4] {
    match *position {
        GridPosition::A => [false, false, false, false],
        GridPosition::B => [false, false, false, true],
        GridPosition::C => [false, false, true, false],
        GridPosition::D => [false, false, true, true],
        GridPosition::E => [false, true, false, false],
        GridPosition::F => [false, true, false, true],
        GridPosition::G => [false, true, true, false],
        GridPosition::H => [false, true, true, true],
        GridPosition::I => [true, false, false, false],
        GridPosition::S => [true, false, false, true],
    }
}

fn unpack_grid_position_from_bools(source: &mut Vec<bool>) -> GridPosition {
    match (
        source.pop().unwrap(),
        source.pop().unwrap(),
        source.pop().unwrap(),
        source.pop().unwrap(),
    ) {
        (false, false, false, false) => GridPosition::A,
        (false, false, false, true) => GridPosition::B,
        (false, false, true, false) => GridPosition::C,
        (false, false, true, true) => GridPosition::D,
        (false, true, false, false) => GridPosition::E,
        (false, true, false, true) => GridPosition::F,
        (false, true, true, false) => GridPosition::G,
        (false, true, true, true) => GridPosition::H,
        (true, false, false, false) => GridPosition::I,
        (true, false, false, true) => GridPosition::S,
        _ => panic!("invalid pack"),
    }
}

fn pack_cell_state_to_bools(state: &CellState) -> [bool; 2] {
    match *state {
        CellState::E => [false, false],
        CellState::O => [false, true],
        CellState::X => [true, false],
        CellState::D => [true, true],
    }
}

fn unpack_cell_state_from_bools(source: &mut Vec<bool>) -> CellState {
    match (source.pop().unwrap(), source.pop().unwrap()) {
        (false, false) => CellState::E,
        (false, true) => CellState::O,
        (true, false) => CellState::X,
        (true, true) => CellState::D,
    }
}

fn pack_grid_state_to_bools(state: &GridState) -> Vec<bool> {
    match *state {
        GridState::O => vec![true, false],
        GridState::X => vec![true, true],
        GridState::E(cells) => {
            let mut result = vec![false];
            result.extend(pack_cell_state_to_bools(&cells.a));
            result.extend(pack_cell_state_to_bools(&cells.b));
            result.extend(pack_cell_state_to_bools(&cells.c));
            result.extend(pack_cell_state_to_bools(&cells.d));
            result.extend(pack_cell_state_to_bools(&cells.e));
            result.extend(pack_cell_state_to_bools(&cells.f));
            result.extend(pack_cell_state_to_bools(&cells.g));
            result.extend(pack_cell_state_to_bools(&cells.h));
            result.extend(pack_cell_state_to_bools(&cells.i));
            result
        }
    }
}

fn unpack_grid_state_from_bools(source: &mut Vec<bool>) -> GridState {
    match source.pop().unwrap() {
        true => {
            if source.pop().unwrap() {
                GridState::X
            } else {
                GridState::O
            }
        }
        false => GridState::E(GridNotEndState {
            a: unpack_cell_state_from_bools(source),
            b: unpack_cell_state_from_bools(source),
            c: unpack_cell_state_from_bools(source),
            d: unpack_cell_state_from_bools(source),
            e: unpack_cell_state_from_bools(source),
            f: unpack_cell_state_from_bools(source),
            g: unpack_cell_state_from_bools(source),
            h: unpack_cell_state_from_bools(source),
            i: unpack_cell_state_from_bools(source),
        }),
    }
}

fn pack_game_state_to_bools(game_state: &GameState) -> Vec<bool> {
    let mut result = Vec::new();
    result.push(game_state.turn == Player::X);
    result.extend(pack_grid_position_to_bools(&game_state.position));
    match &game_state.board {
        BoardState::O => result.extend(vec![true, false]),
        BoardState::X => result.extend(vec![true, true]),
        BoardState::E(board_state) => {
            result.push(false);
            result.extend(pack_grid_state_to_bools(&board_state.a));
            result.extend(pack_grid_state_to_bools(&board_state.b));
            result.extend(pack_grid_state_to_bools(&board_state.c));
            result.extend(pack_grid_state_to_bools(&board_state.d));
            result.extend(pack_grid_state_to_bools(&board_state.e));
            result.extend(pack_grid_state_to_bools(&board_state.f));
            result.extend(pack_grid_state_to_bools(&board_state.g));
            result.extend(pack_grid_state_to_bools(&board_state.h));
            result.extend(pack_grid_state_to_bools(&board_state.i));
        }
    }
    result
}

fn unpack_game_state_from_bools(source: &mut Vec<bool>) -> GameState {
    GameState {
        turn: if source.pop().unwrap() {
            Player::X
        } else {
            Player::O
        },
        position: unpack_grid_position_from_bools(source),
        board: match source.pop().unwrap() {
            true => {
                if source.pop().unwrap() {
                    BoardState::X
                } else {
                    BoardState::O
                }
            }
            false => BoardState::E(BoardNotEndState {
                a: unpack_grid_state_from_bools(source),
                b: unpack_grid_state_from_bools(source),
                c: unpack_grid_state_from_bools(source),
                d: unpack_grid_state_from_bools(source),
                e: unpack_grid_state_from_bools(source),
                f: unpack_grid_state_from_bools(source),
                g: unpack_grid_state_from_bools(source),
                h: unpack_grid_state_from_bools(source),
                i: unpack_grid_state_from_bools(source),
            }),
        },
    }
}

pub fn pack_game_state_to_base64(game_state: &GameState) -> String {
    pack_bools_to_base64(&pack_game_state_to_bools(game_state))
}

pub fn unpack_game_state_from_base64(encoded: &str) -> GameState {
    let mut bools = unpack_base64_to_bools(encoded);
    bools.reverse();
    unpack_game_state_from_bools(&mut bools)
}

pub fn sanitize_grid_state(state: &GridState) -> GridState {
    match *state {
        GridState::O => GridState::O,
        GridState::X => GridState::X,
        GridState::E(grid) => calculate_dead_cell(grid),
    }
}

fn sanitize_board_state(state: &BoardState) -> BoardState {
    match *state {
        BoardState::O => BoardState::O,
        BoardState::X => BoardState::X,
        BoardState::E(board) => {
            let board = BoardNotEndState {
                a: sanitize_grid_state(&board.a),
                b: sanitize_grid_state(&board.b),
                c: sanitize_grid_state(&board.c),
                d: sanitize_grid_state(&board.d),
                e: sanitize_grid_state(&board.e),
                f: sanitize_grid_state(&board.f),
                g: sanitize_grid_state(&board.g),
                h: sanitize_grid_state(&board.h),
                i: sanitize_grid_state(&board.i),
            };
            macro_rules! check_end {
                ($a:ident, $b:ident, $c:ident) => {
                    if matches!(board.$a, GridState::O | GridState::X)
                        && board.$a == board.$b
                        && board.$a == board.$c
                    {
                        if board.$a == GridState::O {
                            return BoardState::O;
                        } else {
                            return BoardState::X;
                        }
                    }
                };
            }
            check_end!(a, b, c);
            check_end!(d, e, f);
            check_end!(g, h, i);
            check_end!(a, d, g);
            check_end!(b, e, h);
            check_end!(c, f, i);
            check_end!(a, e, i);
            check_end!(c, e, g);
            BoardState::E(board)
        }
    }
}

pub fn sanitize_game_state(state: &GameState) -> GameState {
    let mut state = state.clone();
    state.board = sanitize_board_state(&state.board);
    state.position = match state.board {
        BoardState::O => GridPosition::S,
        BoardState::X => GridPosition::S,
        BoardState::E(board) => {
            if false
                || (state.position == GridPosition::A
                    && (board.a == GridState::O || board.a == GridState::X))
                || (state.position == GridPosition::B
                    && (board.b == GridState::O || board.b == GridState::X))
                || (state.position == GridPosition::C
                    && (board.c == GridState::O || board.c == GridState::X))
                || (state.position == GridPosition::D
                    && (board.d == GridState::O || board.d == GridState::X))
                || (state.position == GridPosition::E
                    && (board.e == GridState::O || board.e == GridState::X))
                || (state.position == GridPosition::F
                    && (board.f == GridState::O || board.f == GridState::X))
                || (state.position == GridPosition::G
                    && (board.g == GridState::O || board.g == GridState::X))
                || (state.position == GridPosition::H
                    && (board.h == GridState::O || board.h == GridState::X))
                || (state.position == GridPosition::I
                    && (board.i == GridState::O || board.i == GridState::X))
            {
                GridPosition::S
            } else {
                state.position
            }
        }
    };
    state
}
