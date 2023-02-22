// License: GNU Affero General Public License v3 or later
// A copy of GNU AGPL v3 should have been included in this software package in LICENSE.txt.

use nrps_rs::predictors::predictions::ADomain;
use pyo3::prelude::*;

use super::{PyPrediction, PyPredictionCategory, PyStachPrediction};

/// All the information required about an A domain
#[pyclass(name = "ADomain", module = "nrpys")]
pub struct PyADomain {
    a_domain: ADomain,
}

#[pymethods]
impl PyADomain {
    /// Create a new A domain
    #[new]
    fn new(name: String, aa34: String) -> Self {
        Self {
            a_domain: ADomain::new(name, aa34),
        }
    }

    #[getter]
    fn name(&self) -> String {
        self.a_domain.name.to_string()
    }

    #[getter]
    fn aa34(&self) -> String {
        self.a_domain.aa34.to_string()
    }

    #[getter]
    fn aa10(&self) -> String {
        self.a_domain.aa10.to_string()
    }

    /// Get the best SVM predictions for a category
    #[pyo3(signature = (category, count = 1))]
    fn get_best(&self, category: &PyPredictionCategory, count: usize) -> Vec<PyPrediction> {
        let mut predictions = Vec::new();

        for pred in self.a_domain.get_best_n(category.into(), count).iter() {
            predictions.push(pred.into())
        }

        predictions
    }

    /// Get the best Stachelhaus predictions
    #[pyo3(signature = (count = 1))]
    fn get_stachelhaus(&self, count: usize) -> Vec<PyStachPrediction> {
        let mut predictions = Vec::new();

        for pred in self.a_domain.stach_predictions.get_best_n(count).iter() {
            predictions.push(pred.into())
        }

        predictions
    }
}

impl PyADomain {
    pub fn from_adomain(a_domain: ADomain) -> Self {
        Self { a_domain }
    }
}
