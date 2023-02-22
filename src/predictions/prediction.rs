// License: GNU Affero General Public License v3 or later
// A copy of GNU AGPL v3 should have been included in this software package in LICENSE.txt.

use nrps_rs::predictors::predictions::{Prediction, StachPrediction};
use pyo3::prelude::*;

/// An SVM-based A domain substrate prediction
#[pyclass(name = "Prediction", module = "nrpys")]
pub struct PyPrediction {
    /// Name of the substrate
    #[pyo3(get)]
    pub name: String,
    /// Score the model generated
    #[pyo3(get)]
    pub score: f64,
}

impl From<&Prediction> for PyPrediction {
    fn from(value: &Prediction) -> Self {
        Self {
            name: value.name.to_string(),
            score: value.score,
        }
    }
}

/// A Stachelhaus-table-based A domain substrate prediction
#[pyclass(name = "StachPrediction", module = "nrpys")]
pub struct PyStachPrediction {
    /// Name of the substrate
    #[pyo3(get)]
    pub name: String,
    /// Proportion of Stachelhaus signature matched
    #[pyo3(get)]
    pub aa10_score: f64,
    /// Stachelhaus signature matched
    #[pyo3(get)]
    pub aa10_sig: String,
    /// Proportion of 8Å signature matched
    #[pyo3(get)]
    pub aa34_score: f64,
    /// 8Å signature matched
    #[pyo3(get)]
    pub aa34_sig: String,
}

impl From<&StachPrediction> for PyStachPrediction {
    fn from(value: &StachPrediction) -> Self {
        Self {
            name: value.name.to_string(),
            aa10_score: value.aa10_score,
            aa10_sig: value.aa10_sig.to_string(),
            aa34_score: value.aa34_score,
            aa34_sig: value.aa34_sig.to_string(),
        }
    }
}
