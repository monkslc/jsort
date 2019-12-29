# jsort
A command-line tool to format json

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

