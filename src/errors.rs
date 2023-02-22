// License: GNU Affero General Public License v3 or later
// A copy of GNU AGPL v3 should have been included in this software package in LICENSE.txt.

use nrps_rs::errors::NrpsError;
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NrpysError {
    #[error("NRPS-rs error")]
    NrpsRsError(#[from] NrpsError),
    #[error("Need names and signatures of same length, but got `{0}` vs. `{1}`")]
    LengthMismatch(u64, u64),
}

impl From<NrpysError> for PyErr {
    fn from(value: NrpysError) -> Self {
        match value {
            NrpysError::NrpsRsError(err) => PyOSError::new_err(err.to_string()),
            NrpysError::LengthMismatch(_, _) => PyValueError::new_err(value.to_string()),
        }
    }
}
