use yew::prelude::*;

use crate::entities::counter::ui::Counter;
#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <Counter />
        </div>
    }
}
