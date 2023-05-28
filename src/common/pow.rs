use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use log::info;
use sha2::{Sha256, Digest};
use crate::common::pow::PowError::WrongSolutionError;

pub struct PowProvider {
    difficulty: u8,
}

pub struct PowSolution {
    challenge: Vec<u8>,
    nonce: u64,
}

pub struct PowSolver {
    challenge: Vec<u8>,
    difficulty: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum PowError {
    #[error("the solution is wrong")]
    WrongSolutionError,
    #[error("can't find solution")]
    SolutionNotFoundError,
    #[error("internal error: {0}")]
    SystemError(#[from] SystemTimeError),
}

impl PowProvider {
    pub fn new(difficulty: u8) -> PowProvider {
        return PowProvider {
            difficulty,
        };
    }
}

impl PowSolution {
    pub fn new(challenge: Vec<u8>, nonce: u64) -> PowSolution {
        return PowSolution {
            challenge,
            nonce,
        };
    }
}

impl PowSolver {
    pub fn new(challenge: Vec<u8>, difficulty: u8) -> PowSolver {
        return PowSolver {
            challenge,
            difficulty,
        };
    }

    pub fn find_nonce(&self) -> Result<u64, PowError> {
        for nonce in 0..=u64::MAX {
            if check_solution(nonce, &self.challenge, self.difficulty.clone()).is_ok() {
                info!("Found nonce {}", nonce);
                return Ok(nonce.clone());
            }
        }
        Err(PowError::SolutionNotFoundError)
    }
}

impl PowProvider {
    pub fn generate_challenge(&self) -> Result<Vec<u8>, PowError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(Into::<PowError>::into)?
            .as_secs();
        let mut result = timestamp.to_be_bytes().to_vec();
        result.push(self.difficulty.clone());
        Ok(result)
    }

    pub fn check_solution(&self, solution: &PowSolution) -> Result<(), PowError> {
        return check_solution(solution.nonce.clone(), &solution.challenge, self.difficulty.clone());
    }
}

pub fn hash(bytes: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = hasher.finalize();
    hash.to_vec()
}

pub fn check_solution(nonce: u64, challenge: &Vec<u8>, difficulty: u8) -> Result<(), PowError> {
    let mut vector = nonce.to_le_bytes().to_vec();
    vector.extend(challenge.clone());
    let hash = hash(vector);
    let zeros = vec![0u8; difficulty as usize];
    if hash.starts_with(zeros.as_slice()) {
        return Ok(());
    }
    return Err(WrongSolutionError);
}
