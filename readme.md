# GraphQL vs Rest API

This is just a repo to compare a simple GraphQL API and a Rest API.

You can find the results of 5 runs for each implementation [here](./results/readme.md)

## Running

```bash
# Build the containers
docker-compose build
# Run the benchmark for graphql api
docker-compose run runner-graphql-api
# Run the benchmark for rest api
docker-compose run runner-rest-api
```