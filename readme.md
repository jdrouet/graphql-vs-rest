# GraphQL vs Rest API

This is just a repo to compare a simple GraphQL API and a Rest API.

## Running

```bash
# Build the containers
docker-compose build
# Run the benchmark for graphql api
docker-compose run runner-graphql-api
# Run the benchmark for rest api
docker-compose run runner-rest-api
```