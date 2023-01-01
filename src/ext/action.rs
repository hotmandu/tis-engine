use std::{collections::HashSet, hash::Hash};

use crate::engine;

#[derive(PartialEq, Eq, Hash)]
enum AllowDeny<X>
where
    X: Eq + Hash + Clone,
{
    Allow(X),
    Deny(X),
}

pub struct ActionState<Action, State>
where
    Action: Eq + Hash + Clone,
{
    acts: HashSet<AllowDeny<Action>>,
    last_seen_timestamp: usize,
    state: State,
}

impl<Action, State> ActionState<Action, State>
where
    Action: Eq + Hash + Clone,
{
    pub fn new(initial_state: State) -> Self {
        Self {
            acts: HashSet::new(),
            last_seen_timestamp: 0,
            state: initial_state,
        }
    }

    pub fn allowed_actions(&self) -> Vec<Action> {
        let mut allowed: Vec<Action> = vec![];
        for ax in self.acts.iter() {
            if let AllowDeny::Allow(act) = ax {
                if !self.acts.contains(&AllowDeny::Deny(act.clone())) {
                    allowed.push(act.clone());
                }
            }
        }
        allowed
    }

    pub fn clear_actions(&mut self) {
        self.acts.clear();
    }
}

pub enum ActionTx<Action, STx: engine::Transaction>
where
    Action: Eq + Hash + Clone,
{
    StateTx(STx),
    ActionAllow(Action),
    ActionDeny(Action),
}

impl<Action, STx: engine::Transaction> engine::Transaction for ActionTx<Action, STx>
where
    Action: Eq + Hash + Clone,
{
    type State = ActionState<Action, STx::State>;
    type ErrorType = STx::ErrorType;

    fn apply(&self, state: &mut Self::State, time: usize) -> Result<(), Self::ErrorType> {
        if state.last_seen_timestamp != time {
            // Time advanced. Clear actions
            state.acts.clear();
            state.last_seen_timestamp = time;
        }
        match self {
            ActionTx::StateTx(stx) => stx.apply(&mut state.state, time),
            ActionTx::ActionAllow(act) => {
                state.acts.insert(AllowDeny::Allow(act.clone()));
                Ok(())
            }
            ActionTx::ActionDeny(act) => {
                state.acts.insert(AllowDeny::Deny(act.clone()));
                Ok(())
            }
        }
    }

    fn is_collision_safe_with(&self, other: &Self) -> bool {
        if let ActionTx::StateTx(stx_self) = self {
            if let ActionTx::StateTx(stx_other) = other {
                return stx_self.is_collision_safe_with(stx_other);
            }
        }
        true
    }
}

pub trait ActionReducer<State, Action, Input, StateTx>:
    engine::Reducer<State, Input, ActionTx<Action, StateTx>>
where
    Action: Eq + Hash + Clone,
    StateTx: engine::Transaction,
{
}

pub type ActionEngine<State, Action, Input, StateTx, AR> =
    engine::Engine<State, Input, ActionTx<Action, StateTx>, AR>;
