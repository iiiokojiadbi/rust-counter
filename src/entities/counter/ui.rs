use yew::prelude::*;
use yewdux::prelude::*;

use crate::entities::counter::store::{CounterActions, CounterState};

#[function_component]
pub fn Counter() -> Html {
    let (counter, dispatch) = use_store::<CounterState>();
    let increment = dispatch.apply_callback(|_| CounterActions::Increment);
    let decrement = dispatch.apply_callback(|_| CounterActions::Decrement);

    html! {
        <>
          <p>{ counter.count }</p>
          <button onclick={increment}>{"+1"}</button>
          <button onclick={decrement}>{"-1"}</button>
        </>
    }
}
