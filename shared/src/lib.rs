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

impl<AccountInfo, T> Invariant<AccountInfo> for Box<T>
where
    T: Invariant<AccountInfo> + Clone,
    AccountInfo: 'static,
{
    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_mut().as_any_mut()
    }

    fn check_invariant(&self) {
        self.as_ref().check_invariant()
    }

    fn check_transition_invariant(
        &self,
        old: &dyn Invariant<AccountInfo>,
        remaining: &[AccountInfo],
    ) {
        self.as_ref().check_transition_invariant(old, remaining)
    }
}

// impl<AccountInfo, T> Invariant<AccountInfo> for RefCell<T>
// where
//     T: Invariant<AccountInfo>,
//     AccountInfo: 'static,
// {
//     fn as_any(&self) -> &dyn Any {
//         self.borrow().as_any()
//     }

//     fn check_invariant(&self) {
//         self.borrow().check_invariant()
//     }

//     fn check_transition_invariant(
//         &self,
//         old: &dyn Invariant<AccountInfo>,
//         remaining: &[AccountInfo],
//     ) {
//         self.borrow().check_transition_invariant(old, remaining)
//     }
// }
