FROM python:3.10-slim

WORKDIR /usr/src/app

COPY . .

RUN pip install .

CMD bash tests/functional/tests.sh
