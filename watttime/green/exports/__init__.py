from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some
from ..exports import watttime

class Watttime(Protocol):

    @abstractmethod
    def register(self, username: str, password: str, email: str) -> bool:
        raise NotImplementedError

    @abstractmethod
    def get_token(self) -> Optional[str]:
        raise NotImplementedError

    @abstractmethod
    def get_region(self, token: str, signal_type: str) -> Optional[str]:
        raise NotImplementedError

    @abstractmethod
    def get_forecast(self, token: str, region: str, signal_type: str) -> Optional[watttime.Forecast]:
        raise NotImplementedError

    @abstractmethod
    def get_current_co2_moer_index(self, token: str, region: str, signal_type: str) -> Optional[watttime.Co2MoerIndex]:
        raise NotImplementedError


