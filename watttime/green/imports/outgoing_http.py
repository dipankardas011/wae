from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


@dataclass
class Response:
    status_code: int
    headers: str
    body: bytes

@dataclass
class RequestHeader:
    key: str
    value: str

@dataclass
class Reserror:
    msg: str


def get_request(method: str, headers: List[RequestHeader], url: str, body: Optional[bytes]) -> Response:
    """
    Raises: `green.types.Err(green.imports.outgoing_http.Reserror)`
    """
    raise NotImplementedError

