# malachi
A domain specific pattern matching language made mainly for defining bot commands.

# Status
Malachi is in its alpha stages, however it's already being used in a personal bot!
Make sure to check for updates.

# Warning
This crate only builds on nightly rust for now.

# TODO
-	[ ] Add documentation.
-	[ ] Improve the test suite.

# Notes
-	While declaring commands, any kind of whitespace (space, tab, newline, crlf) are treated the same.
-	`<>` defines a capture.
-	`[]` defines a list of captures to be matched out of order.
-	You can include a space literal by escaping it with `\\`. I.e. `.foo\ bar` will match the literal `"foo bar"` but not `"foo     bar"`.

# Capture Syntax
You can define captures in 3 ways:

1.	`<NAME[QUANTIFIER]>`
2.	`<NAME[QUANTIFIER]: FILTERS>`
3.	`<NAME[QUANTIFIER]: FILTERS1; FILTERS2; FILTERS3...>`

Quantifiers:
-	`?`: Match this capture 1 or 0 times.
-	`*`: Match this capture 0 or more times. The captures will be returned in a `Vec`.
-	`+`: Match this capture at least once. The values are returned in a `Vec`.
-	None: Match this exactly once.

If you use the first form, the default is to match on words (whitespace separated).
Otherwise you must use a filter.

# Filter Syntax
Filters are exactly like a function call in any mainstream language.
They may take arguments.
Arguments are always quoted strings.

Some examples:

-	`starts("--")`
-	`foo()`

# Strings
Strings are always quoted with one of the `"`, `'` or `\``.

You can escape the quotation to include it in the string.

Some more escape patterns are recognized:

-	`\n`: newline.
-	`\t`: tab.
-	`\r`: carriage return.

# Capture Group Syntax
A capture group defines a list of patterns that will be matched out of order.
There are two forms of capture groups:
-	Priority groups: These are delimited with `[]` and the match priority of the captures enclosed are unchanged.
-	Normal group: These groups are enclosed in `{}` and the enclosed captures may be re-ordered for potentially more matches.

The syntax is as follows:

Priority groups:
```
[ CAPTURE1 CAPTURE2 ...CAPTURE_N ]
```

Normal groups:
```
{ CAPTURE1 CAPTURE2 ...CAPTURE_N }
```

Example:

```
[<first> <second> <maybe_third?>]
```

# Some defaults
-	Patterns have an implicit starting anchor (`^` in regex).
-	Whitespace is only used as a terminator while matching.
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
The only required argument is the amount.

```
.bet <amount>
```

## See a tag
This command takes 1 or more space separated arguments.
It does not specify any pattern so the defaults apply:
-	Arguments are whitespace separated.

```
.tag <tags+:>
```

# Filters

-	`starts(prefix)`: The match must start with `prefix`.
-	`ends(suffix)`: The match must end with `suffix`.
