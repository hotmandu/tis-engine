use crate::engine;

struct ActionState<Action, State> {
    acts: Vec<Action>,
    state: State,
}

impl<Action, State> ActionState<Action, State> {
    pub fn new(initial_state: State) -> Self {
        Self {
            acts: vec![],
            state: initial_state,
        }
    }
}