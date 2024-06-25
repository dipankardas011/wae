from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Releases(Protocol):

    @abstractmethod
    def fetch_latest(self, org: str, proj: str) -> str:
        raise NotImplementedError


