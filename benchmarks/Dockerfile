FROM python:3.11.3
RUN useradd -ms /bin/bash  bencher
RUN pip install nlpaug
COPY . /app
RUN pip install /app/wheels/fasttextaug-0.1.0-cp38-abi3-manylinux_2_27_x86_64.whl
USER bencher
CMD [ "/bin/sh", "/app/entrypoint.sh" ]
