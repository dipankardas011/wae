from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Releases(Protocol):

    @abstractmethod
    def get_latest_release(self, org: str, proj: str) -> str:
        raise NotImplementedError

    @abstractmethod
    def get_contributors(self, org: str, proj: str) -> str:
        raise NotImplementedError

    @abstractmethod
    def get_stars(self, org: str, proj: str) -> int:
        raise NotImplementedError


