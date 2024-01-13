use leptos::*;

pub mod chess_board;

use crate::routes::index::chess_board::ChessBoard;
use engine::position::{Move, Position, Side};

#[component]
pub fn IndexPage() -> impl IntoView {
    let (position, set_position) = create_signal(Position::start());
    let (side, set_side) = create_signal(Side::White);

    let handle_move = move |mve: Move| {
        set_position.update(|pos| pos.make_move(&mve).unwrap());
    };

    view! {
        <div class="grid grid-cols-5">
            <div class="bg-gray-200 p-2">
                <h1 class="text-xl font-bold">"vs. computer"</h1>
            </div>
            <div class="col-span-3 flex justify-center">
                <ChessBoard position=position player_side=side handle_move={handle_move} />
            </div>
            <div class="bg-gray-200 p-2">
                <h3 class="text-xl font-bold">"move generation"</h3>
            </div>
        </div>
    }
}
