# GraphQL API Example

Integrating GraphQL with RustAPI using `async-graphql` for flexible, type-safe APIs.

> 📖 **Related**: [RustAPI Cookbook](https://tuntii.github.io/RustAPI/) · [async-graphql Docs](https://async-graphql.github.io/async-graphql/en/index.html)

## Prerequisites

- Rust 1.70+
- Understanding of GraphQL concepts
- Completed [crud-api](../crud-api/) example

## Overview

## Features

- **GraphQL queries** — Query data with flexible field selection
- **GraphQL mutations** — Modify data through mutations
- **Type-safe resolvers** — Compile-time validation of schema
- **GraphQL Playground** — Interactive query builder
- **Schema introspection** — Auto-generated documentation

## Running

```bash
cargo run -p graphql-api
```

Then visit:
- **GraphQL Playground**: http://127.0.0.1:8080/graphql
- **API Info**: http://127.0.0.1:8080/

## Example Queries

### Get all books
```graphql
{
  books {
    id
    title
    author
    year
  }
}
```

### Get a specific book
```graphql
{
  book(id: 1) {
    id
    title
    author
    year
  }
}
```

### Search books
```graphql
{
  searchBooks(query: "Rust") {
    id
    title
    author
  }
}
```

## Example Mutations

### Add a new book
```graphql
mutation {
  addBook(
    title: "Zero to Production in Rust"
    author: "Luca Palmieri"
    year: 2022
  ) {
    id
    title
    author
    year
  }
}
```

## Schema

```graphql
type Book {
  id: ID!
  title: String!
  author: String!
  year: Int!
}

type Query {
  book(id: ID!): Book
  books: [Book!]!
  searchBooks(query: String!): [Book!]!
}

type Mutation {
  addBook(title: String!, author: String!, year: Int!): Book!
}
```

## Integration with RustAPI

This example shows how to:
1. **Define GraphQL types** using `#[derive(SimpleObject)]`
2. **Create resolvers** with `#[Object]` impl blocks
3. **Build schema** with queries and mutations
4. **Serve GraphQL endpoint** alongside REST API
5. **Share state** between GraphQL and REST endpoints

## Production Tips

1. **Add authentication** — Protect mutations with JWT
2. **Implement DataLoader** — Batch database queries
3. **Enable subscriptions** — Real-time updates via WebSocket
4. **Add field complexity** — Prevent expensive queries
5. **Cache responses** — Use Redis for query caching
