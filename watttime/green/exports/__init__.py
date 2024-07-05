from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Watttime(Protocol):

    @abstractmethod
    def register(self, username: str, password: str, email: str) -> bool:
        raise NotImplementedError

    @abstractmethod
    def get_token(self) -> Optional[str]:
        raise NotImplementedError

    @abstractmethod
    def get_region(self, token: str) -> Optional[str]:
        raise NotImplementedError


