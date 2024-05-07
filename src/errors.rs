use thiserror::Error;

#[cfg(feature = "wasm")]
use wasmer::MemoryAccessError;

#[derive(Error, Debug)]
pub enum BigIntOutOfRangeError {
    #[error("Cannot convert negative BigInt into type")]
    Negative,
    #[error("BigInt value is too large for type")]
    Overflow,
}

#[derive(Error, Debug)]
pub enum BigNumberErr {
    #[error("Parser Error")]
    Parser,
    #[error(transparent)]
    OutOfRange(#[from] BigIntOutOfRangeError),
    #[error("Number too big")]
    NumberTooBig,
    #[error(transparent)]
    ParseError(#[from] num_bigint::ParseBigIntError),
}

#[derive(Debug, Error)]
pub enum AscError {
    #[error("Size not fit")]
    SizeNotFit,
    #[error("Value overflow: {0}")]
    Overflow(u32),
    #[error("Error: {0}")]
    Plain(String),
    #[error("Bad boolean value: {0}")]
    IncorrectBool(usize),
    #[error("Size does not match")]
    SizeNotMatch,
    #[error("Maximum Recursion Depth reached!")]
    MaxRecursion,
    #[error(transparent)]
    BigNumberOutOfRange(#[from] BigNumberErr),
    #[cfg(feature = "wasm")]
    #[error(transparent)]
    WasmMemoryAccessError(#[from] MemoryAccessError),
}

impl From<AscError> for wasmer::RuntimeError {
    fn from(e: AscError) -> Self {
        wasmer::RuntimeError::new(e.to_string())
    }
}