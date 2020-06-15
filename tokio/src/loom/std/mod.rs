mod atomic_usize;
mod causal_cell;

pub(crate) mod future {
    pub(crate) use crate::sync::AtomicWaker;
}

pub(crate) mod cell {
    pub(crate) use super::causal_cell::CausalCell;
}

pub(crate) mod sync {
    pub(crate) use std::sync::Arc;

    pub(crate) mod atomic {
        pub(crate) use crate::loom::std::atomic_usize::AtomicUsize;

        pub(crate) use std::sync::atomic::{AtomicPtr};
        pub(crate) use std::sync::atomic::spin_loop_hint;
    }
}

pub(crate) use std::thread;
