FROM postgres:alpine

ENV POSTGRES_HOST_AUTH_METHOD=trust

COPY *.sql /docker-entrypoint-initdb.d/