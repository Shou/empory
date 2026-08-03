FROM postgres:18

WORKDIR /

RUN apt update && apt install -y postgresql-18-cron
COPY postgresql.conf /postgresql.conf
COPY schema.sql /init.sql
COPY dev.sql /mock.sql
# install nanoid() - in the future we probably want the C lib version for speed.
# NOTE we do NOT want nanoid.sql to be recreated, defaults get WIPED
COPY nanoid.sql /docker-entrypoint-initdb.d/nanoid.sql
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
