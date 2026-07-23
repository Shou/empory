birdsht
=======

Evil microblogging platform made by a human in Rust/axum + TypeScript/React.


## Running the project

```bash
pnpm run --filter=front dev --host
cd back; cargo run
cd docker; docker compose build && docker compose --env-file ../.env.dev up
```

This will improve, see: [planned](#Planned)

## Project structure

- back/ - Rust web backend / APIs
- front/ - TypeScript web frontend
- docker/ - docker config, PostgreSQL, RustFS
- infra/ - terraform config for RustFS / S3
- tests/ - Postman, Playwright tests

## Planned

To achieve production-grade, high-availability, horizontal scaling, etc targets for a fictional level of popularity and industry standard ways of writing software, we have the following planned:

- Use OpenAPI or TypeSpec for cross-language API definitions
- DB table partitioning - (cronjob? partman?)
- DB replication and HA failover
- CI/CD, build status
- Backups, point-in-time recovery
- Full-text search via Elasticsearch
- SSL certs
- Deploy to prod (AWS/EKS), use k8s
- Set up reverse proxy, load balancer
- PgBouncer for connection pooling
- Redis cache so you can read tweets blazing fast
- CDN
- Replace simple Rust queue with Kafka/RabbitMQ
- Mitigate against API replay attacks

## AI disclosure

I use Mistral Vibe to rubber duck, for documentation, examples, debugging. No direct code generation - it doesn't have access to my codebase. Why not? Because I'm using this project to learn new things, AI generating everything defeats the purpose - I was inspired to do this project while reading Designing Data Intensive Applications (2nd ed).

