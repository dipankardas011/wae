from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


@dataclass
class PointData:
    point_time: str
    value: float

@dataclass
class Metadata:
    data_point_period_seconds: int
    region: str
    warnings: List[str]
    signal_type: str
    model: str
    units: str
    generated_at_period_seconds: int
    generated_at: str

@dataclass
class Forecast:
    data: List[PointData]
    meta: Metadata

