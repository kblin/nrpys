// License: GNU Affero General Public License v3 or later
// A copy of GNU AGPL v3 should have been included in this software package in LICENSE.txt.

use std::path::PathBuf;

use nrps_rs::config::Config;
use pyo3::prelude::*;

use crate::predictions::PyPredictionCategory;

/// Python version of NRPS-rs Config
#[pyclass(name = "Config", module = "nrpys")]
pub struct PyConfig {
    config: Config,
}

#[pymethods]
impl PyConfig {
    /// Create a configuration object with default settings
    #[new]
    fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    /// Path to directory containing the SVM models
    #[getter]
    fn model_dir(&self) -> &PathBuf {
        self.config.model_dir()
    }

    #[setter]
    fn set_model_dir(&mut self, model_dir: PathBuf) {
        self.config.set_model_dir(model_dir)
    }

    /// Path to the Stachelhaus signature file
    #[getter]
    fn stachelhaus_signatures(&self) -> &PathBuf {
        self.config.stachelhaus_signatures()
    }

    #[setter]
    fn set_stachelhaus_signatures(&mut self, stachelhaus_signatures: PathBuf) {
        self.config
            .set_stachelhaus_signatures(stachelhaus_signatures)
    }

    /// Get the active categories
    #[getter]
    fn categories(&self) -> Vec<PyPredictionCategory> {
        let mut categories = Vec::new();
        for cat in self.config.categories().iter() {
            categories.push(cat.into());
        }

        categories
    }

    /// Fungal mode
    #[getter]
    fn fungal(&self) -> bool {
        self.config.fungal
    }

    #[setter]
    fn set_fungal(&mut self, fungal: bool) {
        self.config.fungal = fungal
    }

    /// Skip v1 models
    #[getter]
    fn skip_v1(&self) -> bool {
        self.config.skip_v1
    }

    #[setter]
    fn set_skip_v1(&mut self, skip: bool) {
        self.config.skip_v1 = skip
    }

    /// Skip v2 models
    #[getter]
    fn skip_v2(&self) -> bool {
        self.config.skip_v2
    }

    #[setter]
    fn set_skip_v2(&mut self, skip: bool) {
        self.config.skip_v2 = skip
    }

    /// Skip v3 models
    #[getter]
    fn skip_v3(&self) -> bool {
        self.config.skip_v3
    }

    #[setter]
    fn set_skip_v3(&mut self, skip: bool) {
        self.config.skip_v3 = skip
    }

    /// Skip Stachelhaus lookup
    #[getter]
    fn skip_stachelhaus(&self) -> bool {
        self.config.skip_stachelhaus
    }

    #[setter]
    fn set_skip_stachelhaus(&mut self, skip: bool) {
        self.config.skip_stachelhaus = skip
    }
}

impl<'a> From<&'a PyConfig> for &'a Config {
    fn from(value: &'a PyConfig) -> Self {
        &value.config
    }
}
