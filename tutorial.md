# Syntax
In Malachi, commands consist of literal text and variable captures/groups.
These are separated by at least one whitespace character; any number of any whitespace can be used and they all mean the same thing.
In contrast to regex, whitespace is not matched literally but used only as a syntactic separator.

Just like in regex, in order for a match to succeed, all the syntactic segments in it must be satisfied.
Unlike regex, every command starts with an implicit starting anchor (`^` in regex.)
A terminating anchor (`$` in regex) is currently not a part of the language but will be added in a future version.

Here's a Malachi command that will match the word "dog" literally:\
`dog`

The above command has only one segment and it is a literal.
This will match:
- "dog"
- "dog says woof"
- "dogs are nice!"

It will not match:
- "where is the dog?"
- "Dog"
- "do"

## Captures
Matching literal text can only get you so far. You most probably also want to extract values from your inputs.
`Captures` let you do exactly that; you can think of them like named capturing groups in regex.

The syntax is as follows:\
`<name[quantifier][: pattern1; pattern2; patternN]>`

That is, a capture is surrounded by angle brackets `<>`, must have a name, can have a quantifier symbol and can be followed by patterns  after a colon.

Before I show you some examples, these are the quantifiers:
- `` (no quantifier): The capture must match once.
- `?`: The capture can match once but is satisfied with no matches.
- `+`: The capture must match at least once.
- `*`: The capture can match any number of times including 0.

Simple enough? Lets see some examples!

#### Example Captures
- `<name>`\
	This is the simplest capture. It matches a word and stores it as "name".
	(A `word` is a whitespace delimited string.)
- `<name?>`\
	This is the same as above except it won't fail if it doesn't match because of the `?` quantifier.
- `<names+>`\
	This capture will match 1 or more words and store every one of them as "names".
	> In Malachi, you can access these as a list! So it won't just overwrite like in regex.
- `<names*>`\
	This is the same as the previous example except it is satisfied with 0 matches.

> Ok. That's cool and all, but what else can we do?
Patterns! It would be a shame to call Malachi a pattern matching language without them!

## Patterns
Patterns let you customize a capture. They are specified after the name and any quantifier, preceded by a colon (`:`).
They consist of filters separated by commas.

A capture can have any number of patterns. Each pattern is separated by a semicolon (`;`).
In order for a capture to succeed, any of its patterns must match.

## Filters
Filters do the matching.
They look like function calls in a programming language. They can have arguments -- quoted strings separated by commas.
Basically: `filter_name("arg1", "arg2", "...argN")`

There are shorthands for some of the filters: They can be  specified with an alternate syntax (the [eq][] filter and the [regex][] filter).

Filters' main purpose is to limit what the pattern can match.
Two special filters also modify the captured text; we'll talk about it in a bit.

The most basic filter is a filter that matches some text literally. This is the `eq` filter.
> Note: This may look pointless but there's more to it. Stick with me.

Here's an example:\
`<hahas*: eq("haha")>`

As you can see, we use the filter after the name, the quantifier and a `:`.
This command has only one segment and it is a capture named "haha".
The "haha" capture has only one pattern and that pattern has only one filter, [eq][].
The quantifier is of course optional but the name and the `:` are not.

This capture now will only match any amount of `haha`s, space-delimited or stuck together like "hahahaha".

It will match:
- `haha`
- `hahahaha`
- `haha haha haha`
- `haha hahahaha haha`

The [eq][] filter accepts multiple arguments; in that case it will match any of the provided strings.

This command matches any number of "dog" or "cat":\
`<dog_or_cat*: eq("dog", "cat")>`

You can specify multiple filters if you separate them with commas.
If a filter is specified more than once, its arguments will be combined into one filter.

The previous command can be specified like below:\
`<dog_or_cat*: eq("dog"), eq("cat")>`

## List of Filters
- [eq][]: Matches any of its arguments exactly.
- [starts][]: Matches a word starting with any of its arguments.
- [ends][]: Matches a word ending with any of its arguments.
- [nocase][]: Makes the [eq][] and [starts][] filters case insensitive.
- [notrim][]: Makes the [starts][] and the [ends][] filters not trim their matches.
- [regex][]: Validates a match with a regular expression.

### The `eq` Filter
We've seen this one in the previous examples but there's more to this filter.
Most importantly, it has a special syntax if you want: it can be used without the `eq()`.

That is, just put a string.

These commands are equivalent:
- `<foo: eq("foo", "bar")>`
- `<foo: "foo", "bar">`
- `<foo: eq("foo"), "bar">`

Second, this filter works with the [nocase][] filter, making the comparison case insensitive.

### The `starts` Filter
This filter will match any text having any of its arguments as a prefix.
By default, it will trim the prefix from the matched text; you can prevent it by using the [notrim][] filter.
This filter also works with the [nocase][] filter, making prefix matching case insensitive.

#### Examples
- `<user: starts("user=")>`\
	Matches:
	- `user=insomnia` (value: `"insomnia"`)
- `<args+: starts("-"), notrim()>`\
	Matches:
	- `-foo -bar` (values: `["-foo", "-bar"]`)
	- `--foo --bar` (values: `["--foo", "--bar"]`)
- `<name: starts("name="), nocase()>`\
	Matches:
	- `name=Joe` (value: `"Joe"`)
	- `NAME=Joestar` (value: `"Joestar"`)

### The `ends` Filter
This filter has similar behaviour to the [starts][] filter except it matches a prefix.\
Unlike the [starts][] filter, the [nocase][] filter currently has no effect with this filter.
The [notrim][] filter can be used alongside it.

### The `nocase` Filter
> This filter takes no arguments.

The `nocase()` filter makes the [eq][] and [starts][] filters case insensitive.

### The `notrim` Filter
> This filter takes no arguments.

The `notrim()` filter prevents the [starts][] and [ends][] filters from trimming their matches.

### The `regex` Filter
The `regex` filter validates the match with a regular expression.
The syntax of these regular expressions are of the [regex crate](https://crates.io/crates/regex)s.

In order to avoid the [escape hell](https://github.com/Hamz-a/php-regex-best-practices/blob/master/06 Escaping a backslash hell.md), Malachi defines an alternate syntax for this filter.
It is the same as the Javascript's regex literals: a regular expression wrapped in a pair of forward slashes (`/`).
You can of course use the normal filter syntax: `regex("\\d+")`, however you will need to escape every backslash twice because Malachi strings recognize some of the escape sequences.

There are notable "gotchas" when dealing with this filter you should be aware of:

- The [nocase][] filter has no effect on the regex.
- This filter gets the string it will run against only after every other filter has done its job.
	This for example means that the [starts][] without [notrim][] will first trim its match, then the regex will be run against the trimmed string.
- The regex is not anchored, use `^` and `$` inside it if you want that behaviour.
- If you use `^` or `$` as anchors inside the regex, those will refer to the start and the end of the match respectively; not to start and end of the whole input.

#### Examples of Regex Validation
So far we've only seen single-segment commands; however a more realistic use case would involve sequences of patterns.
So, lets write a command that takes any amount of numbers and sums them.
We want the command to have valid inputs so each argument should be valid numbers.
We could accomplish this through a regular expression such as `\d+`.

The command:\
`?add <numbers+: /^\-?\d+$/>`

Will do the trick!\
Notice that we've used `^` and `$` anchors inside the regex. The reason for this is without it, an input like `"?add not1number"` would have matched.

The above command will now match:
- `?add 1 2 3` (values: `["1", "2", "3"]`)
- `?add -1 0 22` (values: `["-1", "0", "22"]`)

But won't match:
- `?add 2 books`
- `?add -9C`

Now to demonstrate when the regex gets its input, lets write a command that asks for a year and wants you to prefix it with `year=`.

The command:\
`!year <year: starts("year="), /^\d+$/>`

Will match:
- `!year year=2022` (value: `"2022"`)
But won't match:
- `!year 2022`
- `!year year=this-year`

Notice that the regex only checks for the input to be all digits and the capture matched `year=2022`.
The reason is, the [starts][] filter (`starts("year=")`) trimmed the match before passing it to the regex.
So the regex actually matched against the text `2022`.

## Match Groups
So far we've seen sequential patterns.
But we can also match captures out of order!
Imagine we had a command that took several flags, just like a CLI app: `-age=` and `-name=`.
It would be less than optimal if the flags had to be specified in a specific order like "first the -name=... then the -age=...".

Match groups solve this problem by letting us match out of order.
A match group is one segment that contains 1 or more captures it matches out of order.

There are two kinds of match groups: `Priority Groups` and `normal Groups`.\
They only differ in the order they try to match their captures.

Priority groups try every capture in order, until one of them matches and the remaining text is tried again from the first capture to the last until all the captures are satisfied.\
Normal groups also do the same, however they reorder their captures for potentially more matches as the matching continues.

The syntax for match groups are any number of captures grouped together by `{} (Normal groups) or `[]` (Priority groups).

Lets implement a command for viewing a verse from the [Dante's Divine Comedy](https://en.wikipedia.org/wiki/Divine_Comedy).
This command will have 3 optional parameters: `part=`, `canto=` and `verse=`.
We want to be able to use our command with the flags in any order:
- `?divine part=inferno canto=1 verse=1`
- `?divine canto=2 part=paradiso`
- `?divine verse=2 part=purgatorio canto=4`
And so on...

For this, we use a match group.
> In this example a priority group and a normal group will have the same behaviour. We'll also see an example where it makes a difference.

Here's the `?divine` command:
```text
?divine {
	<part?: starts("part=")>
	<canto?: starts("canto="), /^\d+$/>
	<verse?: starts("verse="), /^\d+$/>
}
```

The canto and the verse need to be numbers so we use a regex to validate them.
We also used a normal group (`{}`), though it doesn't matter in this example.

As you can see, the group contains one or more captures and does not require a separator character.

Here's the command in action:
```rust
let cmd = malachi::Command::new(r"?divine {
	<part?: starts('part=')>
	<canto?: starts('canto='), /^\d+$/>
	<verse?: starts('verse='), /^\d+$/>
}")?;

let m = cmd.get_matches("?divine part=inferno canto=1").unwrap();
assert_eq!(Some("inferno"), m.get_once("part"));
assert_eq!(Some("1"), m.get_once("canto"));
assert_eq!(None, m.get_once("verse"));

let m = cmd.get_matches("?divine verse=2 canto=3 part=paradiso").unwrap();
assert_eq!(Some("paradiso"), m.get_once("part"));
assert_eq!(Some("3"), m.get_once("canto"));
assert_eq!(Some("2"), m.get_once("verse"));
# Ok::<(), malachi::Error>(())
```

> Notice how we used single quotes for the strings? They have the exact same semantics as double quotes or backticks.

#### Priority vs Normal Groups
This is best demonstrated with an example:

##### With Normal Groups
Command:
```text
?foo {
	<flags*: starts(`-`)>
	<args*>
}
```

Input: `?foo -foo -bar`\
Result:
- `flags`: `["foo"]`
- `args`: `["-bar"]`

##### With Priority Groups
Command:
```text
?foo [
	<flags*: starts(`-`)>
	<args*>
]
```

Input: `?foo -a -b -c`\
Result:
- `flags`: `["a", "b", "c"]`
- `args`: `None`

## Strings
Malachi quoted strings can use 3 kinds of quotation: `"`, `'` and \`.
they all have the same behaviour.

Some escape sequences are recognized:
- `\n` inserts a newline.
- `\t` inserts a tab.
- `\\` inserts a single backspace.
- You can escape the quotation itself. E.g. `\"`

Unquoted literals (i.e. outside quotation, as a `literal` segment) have some more escape sequences.
You can escape the opening tokens of captures and match groups by prefixing them with a backslash. E.g. `\<`

To insert a space character as a literal segment you can escape the space with `\`. E.g. `\    `. Though this has little use.

[eq]: #the-eq-filter
[starts]: #the-starts-filter
[ends]: #the-ends-filter
[nocase]: #the-nocase-filter
[notrim]: #the-notrim-filter
[regex]: #the-regex-filter
