pub trait DecodeError<E> {
    fn decode_custom_error_to_enum(_custom: u32) -> Option<E>
// where
    //     E: FromPrimitive,
    {
        // TODO(ahaberlandt)
        None
        // E::from_u32(custom)
    }
    fn type_of() -> &'static str;
}
