use super::store::CounterState;

pub fn select_count(state: &CounterState) -> i32 {
    state.count.into()
}
