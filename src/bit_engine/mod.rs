use super::engine;

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
