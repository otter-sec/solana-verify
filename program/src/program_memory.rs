use crate::vec::sparse::*;

#[allow(clippy::comparison_chain)]
pub fn sol_memcmp(a: &[u8], b: &[u8], _size: usize) -> i32 {
    // TODO: other values are also possible
    if a == b {
        0
    } else if a < b {
        -1
    } else if b < a {
        1
    } else {
        unreachable!()
    }
}

pub trait MaybeAccountBuf<'a> {
    fn mb_data(self) -> &'a mut [u8];
    fn mb_size(&self) -> usize;
    fn mb_is_acct(&self) -> bool;
}
impl<'a> MaybeAccountBuf<'a> for &'a mut [u8] {
    fn mb_data(self) -> &'a mut [u8] {
        self
    }
    fn mb_size(&self) -> usize {
        self.len()
    }
    fn mb_is_acct(&self) -> bool {
        false
    }
}
impl<'a, T> MaybeAccountBuf<'a> for SparseSlice<T> {
    fn mb_data(self) -> &'a mut [u8] {
        // We don't actually write into accounts
        &mut [] as &mut [u8; 0]
    }
    fn mb_size(&self) -> usize {
        self.len()
    }
    fn mb_is_acct(&self) -> bool {
        true
    }
}

pub fn sol_memset<'a, T: MaybeAccountBuf<'a>>(dst: T, c: u8, size: usize) {
    if dst.mb_is_acct() {
        assert!(c == 0);
    }
    dst.mb_data()[..size].fill(c);
}
