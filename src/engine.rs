use std::marker::PhantomData;

use super::util::{PhantomUnsend, PhantomUnsync};

type UID = usize;

pub trait Transaction<State, Aux>
{
    fn apply(&self, state: State) -> State;
    fn aux(&self) -> Aux;

    /// Performs collision check with other transaction.
    /// 
    /// # Soundness
    /// A.collision_check(B) == B.collision_check(A) 는 항상 성립해야 한다.
    /// 추가로, (A, B)가 충돌하지 않으면, 임의의 Transactions [A, B, ...]에 대해서
    /// 항상 그 apply 결과가 어떤 초기 state든지, apply 순서든지 상관 없이 일관성이 있어야 한다.
    fn collision_check(&self, other: &Self) -> bool;
}

pub trait Reducer<State, Input, Tx, Aux>
where
    Tx: Transaction<State, Aux> + Sized
{
    fn develop(state: State, input: Input) -> Option<Tx>;
}

struct SignedTx<Tx, State, Aux>
where
    Tx: Transaction<State, Aux> + Sized
{
    tx: Tx,
    reducer_id: UID,
}

struct Event<Input, Tx, State, Aux> {
    input: Input,
    transactions: Vec<SignedTx<Tx, State, Aux>>,
}

pub enum EngineResult<State, Tx, Aux> {
    Ok(State),
    TransactionConflict(State, Vec<SignedTx<Tx, State, Aux>>),
    ReducerCrashed(State, Vec<UID>),
}

pub struct Engine<State, Input, Tx, Aux, R>
where
    R: Reducer<State, Input, Tx, Aux>,
{
    state: State,
    reducers: Vec<R>,
    events: Vec<Event<Input, Tx, State, Aux>>,
    time: usize,

    _unsend: PhantomUnsend,
    _unsync: PhantomUnsync,
}

impl<State, Input, Tx, Aux, R> Engine<State, Input, Tx, Aux, R>
where
    R: Reducer<State, Input, Tx, Aux>,
{
    pub fn new(state: State) -> Engine<State, Input, Tx, Aux, R> {
        Self {
            state,
            reducers: vec![],
            events: vec![],
            time: 0,

            _unsend: PhantomData,
            _unsync: PhantomData,
        }
    }

    pub fn add_reducer(&mut self, reducer: R) -> UID {
        todo!()
    }

    pub fn observe(&self) -> State {
        self.state
    }

    pub fn step(&mut self, input: Input) -> EngineResult<State, Tx, Aux> {
        todo!()
    }

    pub fn get_reducer(index: usize) -> Option<R> {
        todo!()
    }
}
