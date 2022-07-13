# rust-mongo-graphql

## Running
```
cargo run
```

## .env
[.env]
```
ADDRESS=127.0.0.1 PORT=8000
GRAPHQL_PATH=graphql 
GRAPHIQL_PATH=graphiql
MONGODB_URI=mongodb://localhost:27017 
MONGODB_NAME=rust_test
SITE_KEY=0F4EHz+1/hqVvZjuB8EcooQs1K6QKBvLUxqTHt4tpxE= 
CLAIM_EXP=10000000000 
```

## Summary
[dependencies]
```
we used async-graphql and async-std on project
```
[folder structure]

+ main.rs
+ models is folder model database
+ services is folder for client need to do on service
+ graphql is folder for connect graphql server have mutation and query
+ config is folder for connection database .....

