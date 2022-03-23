from sys import stdin
from time import sleep
from typing import Generator


def stream_disk(path: str) -> Generator:
    handler = open(path, "r")
    for line in handler:
        yield line
    handler.close()


def stream_stdin() -> Generator:
    for line in stdin:
        yield line
