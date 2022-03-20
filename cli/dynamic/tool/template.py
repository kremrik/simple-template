import re
from typing import Optional


VAR_NAME = re.compile("{{[\s]([A-Za-z0-9]*).*}}")
VAR_COMMENT = re.compile(r"{{.*/\*([A-Za-z0-9 ]*)\*/[\s]}}")


def line_is_var(line: str) -> bool:
    return "{{" in line and "}}" in line


def get_indent(line: str) -> int:
    stripped_line = line.lstrip()
    if not stripped_line.startswith("{"):
        return 0

    return len(line) - len(stripped_line)


def get_var_name(line: str) -> str:
    return VAR_NAME.findall(line)[0]


def get_var_comment(line: str) -> Optional[str]:
    find = VAR_COMMENT.findall(line)
    if not find:
        return None

    comment = find[0]
    return comment.strip()
