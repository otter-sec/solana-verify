use std::cell::RefCell;

pub use any_ref;

pub trait Invariant<AccountInfo>
where
    AccountInfo: 'static,
{
    fn check_invariant(&self) -> bool;
    fn check_transition_invariant(
        &self,
        old: &dyn Invariant<AccountInfo>,
        remaining: &[AccountInfo],
    ) -> bool;
}

impl<AccountInfo, T> Invariant<AccountInfo> for Box<T>
where
    T: Invariant<AccountInfo>,
    AccountInfo: 'static,
{
    fn check_invariant(&self) -> bool {
        self.as_ref().check_invariant()
    }

    fn check_transition_invariant(
        &self,
        old: &dyn Invariant<AccountInfo>,
        remaining: &[AccountInfo],
    ) -> bool {
        self.as_ref().check_transition_invariant(old, remaining)
    }
}

impl<AccountInfo, T> Invariant<AccountInfo> for RefCell<T>
where
    T: Invariant<AccountInfo>,
    AccountInfo: 'static,
{
    fn check_invariant(&self) -> bool {
        self.borrow().check_invariant()
    }

    fn check_transition_invariant(
        &self,
        old: &dyn Invariant<AccountInfo>,
        remaining: &[AccountInfo],
    ) -> bool {
        self.borrow().check_transition_invariant(old, remaining)
    }
}
