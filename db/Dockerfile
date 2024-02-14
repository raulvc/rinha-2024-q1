FROM ghcr.io/tursodatabase/libsql-server:latest

USER root
RUN apt-get update && \
    apt-get install -y pip && \
    pip install libsql-client

USER sqld

COPY init.sql /var/lib/sqld/init.sql
COPY setup.sh /var/lib/sqld/setup.sh

ENV SQLD_DB_PATH=/var/lib/sqld/rinha

RUN bash /var/lib/sqld/setup.sh

CMD ["/bin/sqld"]