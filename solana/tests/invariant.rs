use onchor::prelude::invariant;

#[invariant(self.num == 1)]
struct InvariantTest {
    num: u64,
}

#[invariant()]
struct EmptyInvariantTest;

#[test]
fn test_invariant_true() {
    let t = InvariantTest { num: 1 };
    assert!(t._check_invariant());
}

#[test]
fn test_invariant_false() {
    let t = InvariantTest { num: 2 };
    assert!(!t._check_invariant());
}

#[test]
fn test_empty_always_true() {
    let t = EmptyInvariantTest;
    assert!(t._check_invariant());
}
