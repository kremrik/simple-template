from typing import Generator


def stream(path: str) -> Generator:
    handler = open(path, "r")
    for line in handler:
        yield line
    handler.close()
