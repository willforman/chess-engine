use leptos::*;

use engine::position::Move;

#[component]
pub fn Moves(moves: ReadSignal<Vec<Move>>) -> impl IntoView {
    view! {
        <div class="flex-initial flex flex-col bg-gray-200 p-2 h-full">
            <h1 class="text-xl font-bold">"vs. computer"</h1>
            <ul class="overflow-auto">
                {move || moves().iter()
                    .map(|mve| {
                        view! {
                            <li>
                                <p>{format!("{:?}", mve)}</p>
                            </li>
                        }
                    }).collect_view()
                }
            </ul>
        </div>
    }
}
