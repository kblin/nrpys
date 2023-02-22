// License: GNU Affero General Public License v3 or later
// A copy of GNU AGPL v3 should have been included in this software package in LICENSE.txt.

use nrps_rs::predictors::predictions::PredictionCategory;
use pyo3::prelude::*;

#[pyclass(name = "PredictionCategory", module = "nrpys")]
pub enum PyPredictionCategory {
    ThreeClusterV3,
    LargeClusterV3,
    SmallClusterV3,
    SingleV3,
    Stachelhaus,
    ThreeClusterV2,
    ThreeClusterFungalV2,
    LargeClusterV2,
    SmallClusterV2,
    SingleV2,
    LargeClusterV1,
    SmallClusterV1,
}

impl From<&PredictionCategory> for PyPredictionCategory {
    fn from(value: &PredictionCategory) -> Self {
        match value {
            PredictionCategory::ThreeClusterV3 => PyPredictionCategory::ThreeClusterV3,
            PredictionCategory::LargeClusterV3 => PyPredictionCategory::LargeClusterV3,
            PredictionCategory::SmallClusterV3 => PyPredictionCategory::SmallClusterV3,
            PredictionCategory::SingleV3 => PyPredictionCategory::SingleV3,
            PredictionCategory::Stachelhaus => PyPredictionCategory::Stachelhaus,
            PredictionCategory::ThreeClusterV2 => PyPredictionCategory::ThreeClusterV2,
            PredictionCategory::ThreeClusterFungalV2 => PyPredictionCategory::ThreeClusterFungalV2,
            PredictionCategory::LargeClusterV2 => PyPredictionCategory::LargeClusterV2,
            PredictionCategory::SmallClusterV2 => PyPredictionCategory::SmallClusterV2,
            PredictionCategory::SingleV2 => PyPredictionCategory::SingleV2,
            PredictionCategory::LargeClusterV1 => PyPredictionCategory::LargeClusterV1,
            PredictionCategory::SmallClusterV1 => PyPredictionCategory::SmallClusterV1,
        }
    }
}

impl From<&PyPredictionCategory> for &PredictionCategory {
    fn from(value: &PyPredictionCategory) -> Self {
        match value {
            PyPredictionCategory::ThreeClusterV3 => &PredictionCategory::ThreeClusterV3,
            PyPredictionCategory::LargeClusterV3 => &PredictionCategory::LargeClusterV3,
            PyPredictionCategory::SmallClusterV3 => &PredictionCategory::SmallClusterV3,
            PyPredictionCategory::SingleV3 => &PredictionCategory::SingleV3,
            PyPredictionCategory::Stachelhaus => &PredictionCategory::Stachelhaus,
            PyPredictionCategory::ThreeClusterV2 => &PredictionCategory::ThreeClusterV2,
            PyPredictionCategory::ThreeClusterFungalV2 => &PredictionCategory::ThreeClusterFungalV2,
            PyPredictionCategory::LargeClusterV2 => &PredictionCategory::LargeClusterV2,
            PyPredictionCategory::SmallClusterV2 => &PredictionCategory::SmallClusterV2,
            PyPredictionCategory::SingleV2 => &PredictionCategory::SingleV2,
            PyPredictionCategory::LargeClusterV1 => &PredictionCategory::LargeClusterV1,
            PyPredictionCategory::SmallClusterV1 => &PredictionCategory::SmallClusterV1,
        }
    }
}
