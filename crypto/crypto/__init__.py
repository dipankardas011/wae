from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from .types import Result, Ok, Err, Some



class Crypto(Protocol):

    @abstractmethod
    def generate_random(self, length: int) -> str:
        raise NotImplementedError

