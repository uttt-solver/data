extern crate diesel;
extern crate dotenvy;

pub mod models;
pub mod schema;

mod game_state;

use crate::game_state::{BoardState, Player};

use self::models::Branch;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use game_state::{
    pack_game_state_to_base64, sanitize_game_state, sanitize_grid_state,
    unpack_game_state_from_base64, CellState, GameState, GridPosition, GridState,
};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_branches(state: &String) -> Vec<String> {
    let mut result = Vec::new();
    let old_game_state = unpack_game_state_from_base64(state);
    match old_game_state.board {
        BoardState::O | BoardState::X => return result,
        BoardState::E(old_board) => {
            macro_rules! insert {
                ($board:ident, $grid:ident, $position:ident) => {
                    match old_board.$board {
                        GridState::O | GridState::X => (),
                        GridState::E(old_grid) => match old_grid.$grid {
                            CellState::O | CellState::X | CellState::D => (),
                            CellState::E => {
                                let mut new_grid = old_grid.clone();
                                new_grid.$grid = if old_game_state.turn == Player::O {
                                    CellState::O
                                } else {
                                    CellState::X
                                };
                                let mut new_board = old_board.clone();
                                new_board.$board = sanitize_grid_state(&GridState::E(new_grid));
                                let new_game_state = GameState {
                                    turn: if old_game_state.turn == Player::O {
                                        Player::X
                                    } else {
                                        Player::O
                                    },
                                    position: GridPosition::$position,
                                    board: BoardState::E(new_board),
                                };
                                result.push(pack_game_state_to_base64(&sanitize_game_state(
                                    &new_game_state,
                                )));
                            }
                        },
                    };
                };
                ($board:ident, $position:ident) => {
                    if old_game_state.position == GridPosition::$position
                        || old_game_state.position == GridPosition::S
                    {
                        insert!($board, a, A);
                        insert!($board, b, B);
                        insert!($board, c, C);
                        insert!($board, d, D);
                        insert!($board, e, E);
                        insert!($board, f, F);
                        insert!($board, g, G);
                        insert!($board, h, H);
                        insert!($board, i, I);
                    }
                };
            }
            insert!(a, A);
            insert!(b, B);
            insert!(c, C);
            insert!(d, D);
            insert!(e, E);
            insert!(f, F);
            insert!(g, G);
            insert!(h, H);
            insert!(i, I);
            result
        }
    }
}

fn main() {
    use self::schema::branches::dsl::*;
    use self::schema::{branches, branches_next, branches_prev};

    let connection = &mut establish_connection();

    loop {
        let results = branches
            .filter(done.eq(false))
            .order(distance)
            .limit(1)
            .select(Branch::as_select())
            .load(connection)
            .expect("Error loading branches");

        if results.len() == 0 {
            break;
        }

        let branch_to_update = &results[0];
        let new_branches = create_branches(&branch_to_update.state);
        let transaction_result: Result<String, _> = connection.transaction(|conn| {
            for new_branch in &new_branches {
                diesel::insert_into(branches::table)
                    .values((
                        branches::state.eq(new_branch),
                        branches::distance.eq(branch_to_update.distance + 1),
                    ))
                    .on_conflict_do_nothing()
                    .execute(conn)?;
                diesel::insert_into(branches_next::table)
                    .values((
                        branches_next::current.eq(&branch_to_update.state),
                        branches_next::next.eq(new_branch),
                    ))
                    .on_conflict_do_nothing()
                    .execute(conn)?;
                diesel::insert_into(branches_prev::table)
                    .values((
                        branches_prev::current.eq(new_branch),
                        branches_prev::prev.eq(&branch_to_update.state),
                    ))
                    .on_conflict_do_nothing()
                    .execute(conn)?;
            }

            diesel::update(branches::table.find(&branch_to_update.state))
                .set(branches::done.eq(true))
                .execute(conn)?;

            Ok::<String, Error>(branch_to_update.state.clone())
        });

        match transaction_result {
            Ok(result) => {
                println!("Processed {} successfully", result);
            }
            Err(err) => {
                eprintln!("Transaction failed: {}", err);
            }
        }
    }
}
