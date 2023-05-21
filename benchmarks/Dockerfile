FROM python:3.11.3
RUN pip install perftester nlpaug
COPY . /app
RUN pip install /app/wheels/fasttextaug-0.1.0-cp38-abi3-manylinux_2_27_x86_64.whl
CMD [ "python", "/app/tests/benchmark.py" ]
