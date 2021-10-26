# First some defaults
-	Patterns have an implicit starting anchor (`^` in regex).
-	Matches are whitespace separated.
-	`starts` and `ends` filters trim the match.

# Examples
## Run code
This example demonstrates a command for running code.
The flags are optional but must start with `--`
The code block is not optional and must either start and end with:
-	"\`\`\`"
-	"\`"

By default, the `starts` and `ends` attributes also trim the match, which can be turned off by using `no_trim()`.

```
.run
<flags*: starts("--")>
<code:
	starts("```"), ends("```");
	starts("`"), ends("`");
>
```

## Get a Bible Verse
This example command has 3 arguments:
-	`book`: required, must start with `"book=`
-	`chapter` and `verse`: optional, must start with `chapter=` and `verse=` respectively

Since the patterns are wrapped in `[]`, they can be matched out of order.

```
.bible
[
	<book: starts("book=")>
	<chapter?: starts("chapter=")>
	<verse?: starts("verse=")>
]
```

## Bet some credits
This basic example command lets you bet some of your discord credits.
The only required argument is the amount, which must be a non-negative integer.

```
.bet
<amount: is("digits")>
```

## See a tag
This command takes 1 or more space separated arguments.
It does not specify any pattern so the defaults apply:
-	Arguments are whitespace separated.

```
.tag
<tags+:>
```

# Filters

-	`starts(prefix)`: The match must start with `prefix`.
-	`ends(suffix)`: The match must end with `suffix`.
-	`is(specifier)`: The match is `specifier`. Valid values are "digits", "numeric", "alphabetic", "alphanumeric".
-	`separator(text)`: Only applies to matches with possibly multiple values. Specifies the separator.
-	`no_trim()`: Do not trim the match with `starts()` and `ends()`.
