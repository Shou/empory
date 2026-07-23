#!/bin/bash

echo "PostgreSQL..."
docker-entrypoint.sh postgres -c config_file=/postgresql.conf &

echo "Waiting for PostgreSQL..."
PGPASSWORD="$POSTGRES_PASSWORD"
until pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" -h localhost; do sleep 1; done
echo "Init PostgreSQL with schema.sql"
psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -h localhost -f /nanoid.sql
psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -h localhost -f /init.sql
if [ "$ENVIRONMENT" = "dev" ]; then
    psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -h localhost -f /mock.sql
fi
echo "PostgreSQL is set up."
PGPASSWORD=""

tail -f /dev/null