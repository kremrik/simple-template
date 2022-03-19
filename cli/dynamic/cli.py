from . import template

import argparse
from typing import Generator


def generate_parser(handler: Generator) -> argparse.ArgumentParser:
    variables_raw = []
    for line in handler:
        if template.line_is_var(line):
            variables_raw.append(line)

    variables = []
    for line in variables_raw:
        v = template.get_var_name(line)
        variables.append(v)

    parser = argparse.ArgumentParser()
    for v in variables:
        arg = f"--{v}"
        hlp = _make_var_help(v)
        parser.add_argument(arg, help=hlp)
    return parser


def _make_var_help(variable: str) -> str:
    return f"path to {variable}.html template file"
