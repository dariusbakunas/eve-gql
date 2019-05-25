# Experimental EVE GraphQL Server

Started this project to learn Rust

## Setup

### Database Setup

* Download mysql data dump: [download](https://www.fuzzwork.co.uk/dump/)
* Start the container `docker-compose up` and load the data dump

### Environment

* create .env file in project root and configure DATABASE_URL:

```
DATABASE_URL=mysql://user:password@127.0.0.1:3306/eve-gql
```

### Start

```
cargo run
```

Once server starts, access GraphiQl at `http://127.0.0.1:8080/graphiql`

## Example Queries

* get all invTypes:

```
{
  invTypes {
    id
    name
  }
}
```