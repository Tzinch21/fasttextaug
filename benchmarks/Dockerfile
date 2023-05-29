FROM python:3.11.3
RUN useradd -ms /bin/bash  bencher
RUN pip install nlpaug fasttextaug
COPY . /app
USER bencher
CMD [ "/bin/sh", "/app/entrypoint.sh" ]
