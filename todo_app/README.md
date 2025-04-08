# Todo App


The first version of this todo is on the command line. But is obsolete now.
This is a todo app that extracts query parameters from a request and creates a todo states that is stored as long as the server is up.

## Features

- Add tasks
- List added tasks
- Delete/remove added tasks
- App State memory to store states.

How to setup the project
------------------------

To get this project locally do

```sh
git clone https://github.com/Jagoum/S_Project.git
cd S_Project.git
```


 How to use
 -----------

To use this simple app run these commands

```sh
# Open a termianl
cargo build 
cargo run
```

```sh
# open another terminal

## To add a task
 curl -X POST http://localhost:7879/todos\?id\=1\&title\=good%20morning%20do%20not%20miss%20prayers%20today 

## To see all added task
curl -X GET http://localhost:7879/todos

## To delete a task 
curl -X DELETE http://localhost:7879/todos/1
```

