#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod protocols;
pub mod utlities;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum TwoPartyRSAError {
    GeneralError,
    InvalidPaillierKey,
    InvalidElGamalKey,
    InvalidDlogProof,
    InvalidCom,
    CandidateGenerationEncError,
    InvalidModProof,
}

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ProofError {
    DlogProofError,
    ElGamalProofError,
    EqError,
    RangeProofError,
    ModProofError,
    DHProofError,
}
