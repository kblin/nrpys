from enum import Enum


class PredictionCategory(Enum):
    ThreeClusterV3 = "ThreeClusterV3"
    LargeClusterV3 = "LargeClusterV3"
    SmallClusterV3 = "SmallClusterV3"
    SingleV3 = "SingleV3"
    Stachelhaus = "Stachelhaus"
    ThreeClusterV2 = "ThreeClusterV2"
    ThreeClusterFungalV2 = "ThreeClusterFungalV2"
    LargeClusterV2 = "LargeClusterV2"
    SmallClusterV2 = "SmallClusterV2"
    SingleV2 = "SingleV2"
    LargeClusterV1 = "LargeClusterV1"
    SmallClusterV1 = "SmallClusterV1"


class Prediction:
    name: str
    score: float


class StachPrediction:
    name: str
    aa10_score: float
    aa10_sig: str
    aa34_score: float
    aa34_sig: str


class ADomain:
    name: str
    aa34: str
    aa10: str

    def __new__(cls: type["ADomain"]) -> "ADomain": ...

    def get_best(self, category: PredictionCategory,
                 count: int = 1) -> list[Prediction]: ...

    def get_stachelhaus(self, count: int = 1) -> list[StachPrediction]: ...


class Config:
    model_dir: str
    stachelhaus_signatures: str
    categories: list[PredictionCategory]
    skip_v1: bool
    skip_v2: bool
    skip_v3: bool
    skip_stachelhaus: bool

    def __new__(cls: type["Config"]) -> "Config": ...


def run(config: Config, names: list[str],
        signatures: list[str]) -> list[ADomain]: ...
