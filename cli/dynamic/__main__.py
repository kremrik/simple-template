from .cli import generate_parser

import sys


HELP = """
  << INSERT HELP HERE >>
"""


def main():
    if len(sys.argv) == 1:
        print(HELP)
        exit(1)

    init_arg = sys.argv[1]
    template_args = sys.argv[2:]

    with open(init_arg, "r") as f:
        parser = generate_parser(f)

    parser.parse_args(template_args)


if __name__ == "__main__":
    main()
