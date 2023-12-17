use super::actions::CounterActions;
use std::rc::Rc;
use yewdux::store::{Reducer, Store};

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct CounterState {
    pub count: i32,
}

impl Reducer<CounterState> for CounterActions {
    fn apply(self, mut counter: Rc<CounterState>) -> Rc<CounterState> {
        let state = Rc::make_mut(&mut counter);

        match self {
            CounterActions::Increment => state.count += 1,
            CounterActions::Decrement => state.count -= 1,
            CounterActions::Reset => state.count = 0,
        };

        counter
    }
}
