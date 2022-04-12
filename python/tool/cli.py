from . import template

import argparse
from typing import Generator, List


def generate_parser(handler: Generator) -> argparse.ArgumentParser:
    variables_raw = []
    for line in handler:
        if template.line_is_var(line):
            variables_raw.append(line)

    variables = []
    for line in variables_raw:
        var = template.get_var_name(line)
        comment = template.get_var_comment(line)
        variables.append((var, comment))

    parser = argparse.ArgumentParser(
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    for var, comment in variables:
        arg = f"--{var}"
        hlp = comment or _default_help(var)
        parser.add_argument(arg, help=hlp)
    return parser


def bundle_args(args: List[str]) -> dict:
    # naive implementation, assumes there is a matching
    # value for each param
    kvs = [args[i:i+2] for i in range(0, len(args), 2)]

    # also naive, requires param to have leading "--"
    cleaned_kvs = [(k.replace("--", ""), v) for k, v in kvs]
    return dict(cleaned_kvs)


def _default_help(variable: str) -> str:
    return f"path to {variable}.html template file"
