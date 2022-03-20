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
        var = template.get_var_name(line)
        comment = template.get_var_comment(line)
        print("#######", line, comment)
        variables.append((var, comment))

    parser = argparse.ArgumentParser()
    for var, comment in variables:
        arg = f"--{var}"
        hlp = comment or _default_help(var)
        parser.add_argument(arg, help=hlp)
    return parser


def _default_help(variable: str) -> str:
    return f"path to {variable}.html template file"
