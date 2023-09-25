#[derive(Debug)]
pub enum PrologError {
    TermConversionError,
    NoTermVariantMatch,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for PrologError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for PrologError {}
