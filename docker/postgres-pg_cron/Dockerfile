FROM postgres:18

WORKDIR /

RUN apt update && apt install postgresql-18-cron
COPY postgresql.conf /postgresql.conf
COPY schema.sql /init.sql
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
