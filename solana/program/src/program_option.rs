pub type COption<T> = Option<T>;

// #[derive(Copy, Clone, Debug, PartialEq)]
// #[repr(C)]
// pub enum COption<T : Copy> {
//     None,
//     Some(T),
// }

// impl<T> From<Option<T>> for COption<T>
// where T: Copy
// {
//     fn from(o: Option<T>) -> Self {
//         match o {
//             Option::Some(x) => Self::Some(x),
//             Option::None => Self::None
//         }
//     }
// }
