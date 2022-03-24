from .tool import cli, io, render

import sys


CMD_NAME = "__main__.py"
HELP = f"""\
<<< help here >>>
""".strip()


def main():
    args = sys.argv[1:]

    if len(args) == 0:
        print(HELP)
        exit(1)

    if "-h" in args or "--help" in args:
        handler = io.stream_stdin()
        parser = cli.generate_parser(handler)
        args = parser.parse_args(args)
        exit(0)

    args = sys.argv[1:]
    var_map = cli.bundle_args(args)
    input_stream = io.stream_stdin()
    output = render.render(input_stream, var_map)
    sys.stdout.write("".join(list(output)))


if __name__ == "__main__":
    main()
