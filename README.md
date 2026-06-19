birdshit
========

Evil microblogging platform made by a human in Rust/axum + TypeScript/React.


## Running the project

Right now:

1. cd front && pnpm run dev --host
2. cd back && cargo run
3. docker-compose up

I'll make it less annoying in the near future...

## Project structure

- back/ - Rust web backend / APIs
- front/ - TypeScript web frontend
- db/ - schemas
- tests/ - Postman, Playwright tests

## functional reqs

1. Auth. User registration, login.
2. Posts. Text-only posts. <512 char
3. Feed. View of ALL posts. Followed users come later.
4. Profile. View a user's post specifically.
5. Followed users. Now we'll need derived SQL tables, queues, etc.

## non-functional reqs

1. Backend: Rust + framework.
2. Frontend: TypeScript + React + ??
3. Database: PostgreSQL.
4. API: RESTful. Shared API schema + validation? OpenAPI... or?

## AI disclosure

I use Mistral Vibe to rubber duck, for documentation, examples, debugging. No direct code generation - it doesn't have access to my codebase. Why not? Because I'm using this project to learn Rust and other tools, AI generating everything defeats the purpose - I was inspired to do this project while reading Designing Data Intensive Applications (2nd ed).