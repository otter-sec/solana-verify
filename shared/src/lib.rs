use std::{any::Any, cell::RefCell};

pub use any_ref;
use dyn_clone::DynClone;

pub trait Invariant<AccountInfo> : Any + DynClone
where
    AccountInfo: 'static,
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn check_invariant(&self);
    fn check_transition_invariant(
        &self,
        old: &dyn Invariant<AccountInfo>,
        remaining: &[AccountInfo],
    );
}
