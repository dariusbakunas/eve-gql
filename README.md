# Experimental EVE GraphQL Server

Started this project to learn Rust

## Setup

### Database Setup

* Download mysql data dump: [download](https://www.fuzzwork.co.uk/dump/)
* Start the container `docker-compose up` and load the data dump

### Environment

* create .env file in project root and configure DATABASE_URL (base64 encoded):

```
DATABASE_URL=base64(mysql://user:password@127.0.0.1:3306/eve-gql)
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

* character info:

```
{
  character(id: XXXXXXXX) {
    id
    name
    ancestry {
      name
    }
    bloodline {
      name
    }
    race {
      name
      description
    }
    skillQueue(token: "XXXXXXXXX") {
      id      
      index
      finishedLevel
      startDate
      finishDate
      trainingStartSp
      levelEndSp
      levelStartSp    
    }
  }
}

```

* get skill groups:

```
{
  skillGroups {
    id
    name
    skills {
      id
      name    
      description  
    }
  }
}
```