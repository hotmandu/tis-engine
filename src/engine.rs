use std::marker::PhantomData;

use super::util::{PhantomUnsend, PhantomUnsync};

type UID = usize;

pub trait Transaction : Sized
{
    type State;

    fn apply(&self, state: Self::State) -> Self::State;

    /// Performs collision check with other transaction.  
    /// - `True` = 충돌 안남
    /// - `False` = 충돌 남
    /// 
    /// # Soundness
    /// `A.is_collision_safe_with(B) == B.is_collision_safe_with(A)` 는 항상 성립해야 한다.
    /// 
    /// 추가로, (A, B)가 충돌하지 않으면, 임의의 Transactions \[A, B, ...\]에 대해서
    /// 항상 그 apply 결과가 어떤 초기 state든지, apply 순서든지 상관 없이 일관성이 있어야 한다.
    fn is_collision_safe_with(&self, other: &Self) -> bool;
}

pub trait Reducer<State, Input, Tx: Transaction>
{
    fn develop(&self, state: &State, input: &Input) -> Option<Tx>;
}

struct Event<Input, Tx: Transaction>
{
    input: Input,
    transactions: Vec<(UID, Tx)>,
}

pub enum EngineResult<'a, State, Tx: Transaction>
{
    Ok(&'a State),
    TransactionConflict(&'a State, &'a Vec<(UID, Tx)>),
    ReducerCrashed(&'a State, Vec<UID>),
}

pub struct Engine<State, Input, Tx, R>
where
    Tx: Transaction,
    R: Reducer<State, Input, Tx>,
{
    state: State,
    reducers: Vec<R>,
    events: Vec<Event<Input, Tx>>,
    time: usize,

    _unsend: PhantomUnsend,
    _unsync: PhantomUnsync,
}

impl<State, Input, Tx, R> Engine<State, Input, Tx, R>
where
    Tx: Transaction,
    R: Reducer<State, Input, Tx>,
{
    pub fn new(state: State) -> Engine<State, Input, Tx, R> {
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
        let i = self.reducers.len();
        self.reducers.insert(i, reducer);
        i
    }

    pub fn observe<'a>(&'a self) -> &'a State {
        &self.state
    }

    pub fn step<'a>(&'a mut self, input: Input) -> EngineResult<'a, State, Tx> {
        self.time = self.time + 1;

        // 1. Calculate Event
        let mut ev: Event<Input, Tx> = Event { input, transactions: vec![] };
        // let mut crashed_reducers: Vec<usize> = vec![];
        
        let cnt_rdr = self.reducers.len();
        for i in 0..cnt_rdr {
            let rdr = self.reducers.get(i).unwrap() as &dyn Reducer<State, Input, Tx>;
            // TODO: Add try-catch?
            let opt_tx = rdr.develop(&self.state, &ev.input);
            if let Some(tx) = opt_tx {
                ev.transactions.push((i, tx));
            }
        }

        // 2. Add it to Log

        // 3. Return result
        todo!()
    }

    pub fn get_reducer<'a>(&'a self, index: usize) -> Option<&'a R> {
        self.reducers.get(index)
    }
}
