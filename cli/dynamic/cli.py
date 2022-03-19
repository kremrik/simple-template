import argparse
import re
from typing import IO


VAR_PATT = re.compile("{{[\s]([A-Za-z0-0]*)[\s]}}")


def generate_parser(handler: IO) -> argparse.ArgumentParser:
    variables_raw = []
    for line in handler:
        if _line_is_var(line):
            variables_raw.append(line)

    variables = []
    for line in variables_raw:
        v = _get_var_name(line)
        variables.append(v)

    parser = argparse.ArgumentParser()
    for v in variables:
        arg = f"--{v}"
        hlp = _make_var_help(v)
        parser.add_argument(arg, help=hlp)
    return parser


def _line_is_var(line: str) -> bool:
    return "{{" in line and "}}" in line


def _get_var_name(line: str) -> str:
    return VAR_PATT.findall(line)[0]


def _make_var_help(variable: str) -> str:
    return f"path to {variable}.html template file"
