import re


VAR_PATT = re.compile("{{[\s]([A-Za-z0-0]*)[\s]}}")


def line_is_var(line: str) -> bool:
    return "{{" in line and "}}" in line


def get_indent(line: str) -> int:
    return len(line) - len(line.lstrip())


def get_var_name(line: str) -> str:
    return VAR_PATT.findall(line)[0]
