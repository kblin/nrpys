// License: GNU Affero General Public License v3 or later
// A copy of GNU AGPL v3 should have been included in this software package in LICENSE.txt.

pub mod config;
pub mod errors;
pub mod predictions;

use nrps_rs;
use nrps_rs::predictors::predictions::ADomain;
use predictions::PyADomain;
use pyo3::prelude::*;

/// Runs the A domain substrate predictions
#[pyfunction]
fn run(
    config: &config::PyConfig,
    names: Vec<String>,
    signatures: Vec<String>,
) -> PyResult<Vec<PyADomain>> {
    if names.len() != signatures.len() {
        return Err(errors::NrpysError::LengthMismatch(
            names.len() as u64,
            signatures.len() as u64,
        )
        .into());
    }

    let mut domains: Vec<ADomain> = Vec::with_capacity(names.len());
    for (name, aa34) in names.iter().zip(signatures.iter()) {
        domains.push(ADomain::new(name.to_string(), aa34.to_string()));
    }

    let res = nrps_rs::run(config.into(), &mut domains);
    if let Err(err) = res {
        return Err(errors::NrpysError::NrpsRsError(err).into());
    }

    let results: Vec<PyADomain> = domains
        .iter()
        .map(|dom| PyADomain::from_adomain(dom.clone()))
        .collect();

    Ok(results)
}

#[pymodule]
fn nrpys(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<config::PyConfig>()?;
    m.add_class::<predictions::PyADomain>()?;
    m.add_class::<predictions::PyPredictionCategory>()?;
    m.add_class::<predictions::PyPrediction>()?;
    m.add_class::<predictions::PyStachPrediction>()?;

    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
