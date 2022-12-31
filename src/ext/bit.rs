use std::{collections::HashSet};

use crate::engine;

use fixedbitset::FixedBitSet;

pub enum Operation {
    Exchange(usize, usize),
}

mod bit_operation {
    use super::Operation;
    use fixedbitset::FixedBitSet;

    pub fn apply(target: &mut FixedBitSet, op: &Operation) {
        match op {
            Operation::Exchange(a, b) => {
                let (a, b) = (*a, *b);
                let old_a = target[a];
                let old_b = target[b];
                target.set(a, old_b);
                target.set(b, old_a);
            },
        }
    }
}

pub struct BitTx {
    ops: Vec<Operation>,
}

impl BitTx {
    pub fn new() -> BitTx {
        BitTx { ops: vec![] }
    }

    pub fn append(&mut self, op: Operation) {
        self.ops.push(op);
    }

    fn touching_mask(&self) -> HashSet<usize> {
        let mut mask = HashSet::new();
        for op in self.ops.iter() {
            match op {
                Operation::Exchange(a, b) => {
                    mask.insert(*a); mask.insert(*b);
                },
            }
        }
        mask
    }
}

impl engine::Transaction for BitTx {
    type State = FixedBitSet;

    fn apply(&self, mut state: Self::State) -> Self::State {
        for op in self.ops.iter() {
            bit_operation::apply(&mut state, op)
        }
        state
    }

    fn is_collision_safe_with(&self, other: &Self) -> bool {
        let this_mask = self.touching_mask();
        let that_mask = other.touching_mask();
        
        this_mask.is_disjoint(&that_mask)
    }
}

pub trait BitReducer<Input> : engine::Reducer<FixedBitSet, Input, BitTx> {}
pub type BitEngine<Input, BR> = engine::Engine<FixedBitSet, Input, BitTx, BR>;
