# actix-graphql-react

**NOT** complete - Clean boilerplate for GraphQL app built with Rust & TypeScript.

* Backend Demo (WIP): https://api.budshome.com/gql
* Frontend Demo (WIP): https://cms.budshome.com

## Features

- [x] DB migration with Diesel
- [x] User: query & mutation
- [x] Project: query & mutation
- [x] User register
- [ ] Sign up & Sign in
- [ ] Encrypt password & Change password
- [ ] Profile Update
- [ ] JSON web token authentication

## Stacks

### Backend

- [Rust](https://www.rust-lang.org/zh-CN)
- [actix-web](https://crates.io/crates/actix-web) - Web server
- [juniper](https://crates.io/crates/juniper) - GraphQL server，[中文文档](https://juniper.budshome.com)
- [diesel](https://crates.io/crates/diesel) - ORM
- [PostgreSQL](https://postgresql.org) - Database
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken) - JSON Web Token
- [GraphQL Playground](https://github.com/prisma-labs/graphql-playground) - GraphQL UI

### Frontend

- [TypeScript](https://www.typescriptlang.org)
- [react](https://zh-hans.reactjs.org) - User Interfaces
- [apollo-client](https://www.apollographql.com/docs/react) - GraphQL client

## How to run?

### Backend

``` Bash
git clone https://github.com/zzy/actix-graphql-react.git
cd actix-graphql-react/backend/
```

#### Put the `DATABASE_URL` & `port` in a `.env` file.

``` Bash
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://username:password@localhost/actix_graphql > .env
echo GRAPHQL_PORT=5000 >> .env
```

#### Build & Release

- Build

``` Bash
diesel setup
diesel migration run
cargo build
cargo run
```

- Release

``` Bash
cargo build --release
cd target/release
./actix-graphql-react
```

GraphiQL: connect to http://localhost:5000/gql with browser.

### Frontend

``` Bash
git clone https://github.com/zzy/actix-graphql-react.git
cd actix-graphql-react/frontend/
```

#### yarn

``` Bash
yarn install # yarn upgrade; yarn install
yarn start
```

#### npm

``` Bash
npm install # npm up; npm install
npm start
```

Then go to http://localhost:3000
