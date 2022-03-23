from .tool import cli, io, render

import sys
from os.path import abspath


CMD_NAME = "__main__.py"
HELP = f"""\
<<< help here >>>
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
    if "-h" in template_args or "--help" in template_args:
        handler = io.stream_disk(root_template)
        parser = cli.generate_parser(handler)
        args = parser.parse_args(template_args)
        exit(0)

    args = sys.argv[1:]
    var_map = cli.bundle_args(args)
    input_stream = io.stream_stdin()
    output = render.render(input_stream, var_map)
    sys.stdout.write("".join(list(output)))


if __name__ == "__main__":
    main()
