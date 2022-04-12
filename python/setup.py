import tool

from setuptools import find_packages, setup


DESCRIPTION = ""


setup(
    name="htmlayout",
    version=tool.__version__,
    author="Kyle Emrick",
    url="https://github.com/kremrik/htmlayout",
    description=DESCRIPTION,
    packages=find_packages(exclude=("docs")),
    include_package_data=True,
    scripts=["bin/htmlayout-d"]
)
