# jsort
A command-line tool to format json

### Install
`cargo install --git https://github.com/monkslc/jsort`

### Examples
**Passing in a file**
```
> jsort example.json
{
  "age": 39,
  "desc": "tall",
  "name": "Jane"
}
```

**Reading from stdin**
```
> cat example.json | jsort
{
  "age": 39,
  "desc": "tall",
  "name": "Jane"
}
```

