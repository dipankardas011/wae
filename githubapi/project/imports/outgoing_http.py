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
    body: str

@dataclass
class RequestHeader:
    key: str
    value: str

@dataclass
class Reserror:
    msg: str


def get_request(method: str, headers: List[RequestHeader], url: str) -> Response:
    """
    Raises: `project.types.Err(project.imports.outgoing_http.Reserror)`
    """
    raise NotImplementedError

