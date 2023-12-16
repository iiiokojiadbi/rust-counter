use std::rc::Rc;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct CounterState {
    pub count: i32,
}

pub enum CounterActions {
    Increment,
    Decrement,
}

impl Reducer<CounterState> for CounterActions {
    fn apply(self, mut counter: Rc<CounterState>) -> Rc<CounterState> {
        let state = Rc::make_mut(&mut counter);
        match self {
            CounterActions::Increment => state.count += 1,
            CounterActions::Decrement => state.count -= 1,
        };

        counter
    }
}
