from . import cli, io, render

import sys
from os.path import abspath


HELP = """
  << INSERT HELP HERE >>
"""


def main():
    if len(sys.argv) == 1:
        print(HELP)
        exit(1)

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
