use super::store::CounterState;
use web_sys::MouseEvent;
use yewdux::dispatch::Dispatch;

pub enum CounterActions {
    Increment,
    Decrement,
    Reset,
}

pub fn counter_increment(_: MouseEvent) -> () {
    let dispatch = Dispatch::<CounterState>::new();

    dispatch.apply(CounterActions::Increment);
}

pub fn counter_decrement(_: MouseEvent) -> () {
    let dispatch = Dispatch::<CounterState>::new();

    dispatch.apply(CounterActions::Decrement);
}

pub fn counter_reset(_: MouseEvent) -> () {
    let dispatch = Dispatch::<CounterState>::new();

    dispatch.apply(CounterActions::Reset);
}
