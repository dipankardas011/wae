from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Llm(Protocol):

    @abstractmethod
    def text_to_text(self) -> None:
        raise NotImplementedError

    @abstractmethod
    def text_to_image(self, prompt: str) -> str:
        """
        *String returned is a the location where to look for the image*/
        """
        raise NotImplementedError


