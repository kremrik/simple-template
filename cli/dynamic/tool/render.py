from . import io
from . import template

from typing import Generator


def render(root_path: str, variable_paths: dict) -> Generator:
    root_template = io.stream(root_path)

    for line in root_template:
        if template.line_is_var(line):
            var = template.get_var_name(line)
            indent = " " * template.get_indent(line)
            var_path = variable_paths[var]
            child_template = io.stream(var_path)
            for line in child_template:
                yield indent + line
        else:
            yield line
