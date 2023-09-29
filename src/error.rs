use crate::sp::SP_term_ref;

#[derive(Debug)]
pub enum PrologError {
    TermConversionError,
    NoTermVariantMatch,
    AtomNotFound(String),
    CloseQueryError(String),
    UnexpectedReturnCode(i32),
    NoExceptionTerm(SP_term_ref),
    UncussefulUnificationError(i32, i32),
    ConsFunctorError,
    QueryOpenUnsuccessful,
    PredicateNotFound,
    NextSolutionError(String),
    NoMoreSolutions,
    CutQueryError(String),
    CutQueryFailure,
    DefineCPredicateError,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for PrologError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for PrologError {}
