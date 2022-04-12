from . import io
from . import template

from typing import Generator


def render(
    root_template_stream: Generator, 
    variable_paths: dict
) -> Generator:
    for line in root_template_stream:
        if template.line_is_var(line):
            var = template.get_var_name(line)
            indent = " " * template.get_indent(line)
            var_path = variable_paths.get(var)
            if not var_path:
                yield line
                continue

            child_template = io.stream_disk(var_path)
            for line in child_template:
                yield indent + line

        else:
            yield line
