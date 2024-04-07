# Malachi
Malachi is a domain specific pattern matching language made mainly for defining bot commands.

There is a tutorial: [tutorial.md](tutorial.md).

## Syntax Example
```
?divine [
	<part?: starts("part=")>
	<canto?: starts("canto="), /^\d+$/>
	<verse?: starts("verse="), /^\d+$/>
]
```

## Usage Examples
See the [examples directory](/examples).
