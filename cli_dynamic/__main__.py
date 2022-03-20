from .tool import cli, io, render

import sys
from os.path import abspath


CMD_NAME = "__main__.py"
HELP = f"""\
usage: {CMD_NAME} [-h] [template] [template args]

positional arguments:
  template

optional arguments:
  -h, --help  show this help message and exit
  [template args]  optional arguments related to specific template
""".strip()


def main():
    if len(sys.argv) == 1:
        print(HELP)
        exit(1)

    if sys.argv[1] in ("-h", "--help"):
        print(HELP)
        exit(0)

    root_template = sys.argv[1]
    template_args = sys.argv[2:]

    handler = io.stream(root_template)
    parser = cli.generate_parser(handler)
    args = parser.parse_args(template_args)

    var_map = {
        var_name: abspath(var_path)
        for var_name, var_path in args.__dict__.items()
    }

    output = render.render(root_template, var_map)
    sys.stdout.write("".join(list(output)))


if __name__ == "__main__":
    main()
