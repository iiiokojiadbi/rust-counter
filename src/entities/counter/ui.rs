use super::{
    actions::{counter_decrement, counter_increment, counter_reset},
    selectors::select_count,
};
use yew::{function_component, html, Html};
use yewdux::functional::use_selector;

#[function_component]
pub fn Counter() -> Html {
    let count = use_selector(select_count);

    html! {
        <>
          <p>{ *count }</p>
          <button onclick={counter_increment}>{"+1"}</button>
          <button onclick={counter_decrement}>{"-1"}</button>
          <button onclick={counter_reset}>{"reset count"}</button>
        </>
    }
}
