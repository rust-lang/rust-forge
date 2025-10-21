# Labeling

You can apply GitHub labels to an issue or PR by posting a comment.
Labeling of issues can be very helpful for searching, tying issues together, and indicating information in a formal way, such as the status.

The Triage WG helps with labeling issues.
If you are interested in helping triaging issues, see the [Triage WG procedure](../release/triage-procedure.md).

## Usage

The general form of the comment should be `@rustbot label` followed by a space-separated list of labels to add or remove.
You can remove labels by prefixing them with the `-` character.
Some examples:

* `@rustbot label A-diagnostics A-macros`
* `@rustbot label +T-lang -T-compiler` --- Removes `T-compiler` and adds `T-lang`.

Labels are parsed from left to right and then applied by grouping *adding* labels and *removing* labels. Example:
```
# this command ...
@rustbot label +Alpaca -Bench -Carlo +Esteban +Dwight

# ... will be executed as:
@rustbot label +Alpaca +Esteban +Dwight -Bench -Carlo
```

Labels are parsed and applied from left to right (canceling conflicting labels). Example:
```
# this command ...
@rustbot label +Alpaca -Bench -Carlo +Esteban +Dwight +Bench

# ... will be executed as:
@rustbot label +Alpaca +Esteban +Dwight -Carlo
```

The syntax for the command is somewhat flexible, supporting a few different forms to suit your pleasure.
Some examples of variants you can use:

* `@rustbot label: +T-lang, -T-compiler`
* `@rustbot label: +T-lang and -T-compiler`
* `@rustbot modify labels to +T-lang and -T-compiler`
* `@rustbot modify labels: +T-lang and -T-compiler`
* `@rustbot modify labels to +T-lang -T-compiler`
* `@rustbot labels "+good first issue"`

The command can be terminated with a `.`, `;`, or the end of the line.

Formally the grammar is:

> Command → `@rustbot` `modify`? *label-word* `to`? `:`? *label-list* (`;` | `.`)?
>
> label-word →\
> &nbsp;&nbsp; &nbsp;&nbsp; `label`\
> &nbsp;&nbsp; | `labels`
>
> label-list →\
> &nbsp;&nbsp; &nbsp;&nbsp; *label-delta*<sup>+</sup>\
> &nbsp;&nbsp; | *label-delta* `and` *label-list*\
> &nbsp;&nbsp; | *label-delta* `,` *label-list*\
> &nbsp;&nbsp; | *label-delta* `,` `and` *label-list*
>
> label-delta →\
> &nbsp;&nbsp; &nbsp;&nbsp; `+` *label*\
> &nbsp;&nbsp; | `-` *label*\
> &nbsp;&nbsp; | *label* \
> &nbsp;&nbsp; | `"` `+` *label-quoted* `"`\
> &nbsp;&nbsp; | `"` `-` *label-quoted* `"`\
> &nbsp;&nbsp; | `"` *label-quoted* `"`
>
> label-quoted → \[^"]*
>
> label → \[^.,:!?;\n() ]+


### Permissions

All labels can be assigned by rust-lang organization team members (and wg-triage, wg-async).
Users not on a team can only assign labels that are explicitly authorized in `triagebot.toml`.
It is encouraged for maintainers to allow the majority of labels to be applied by anyone.
An example of one that would be restricted is `beta-accepted`, since accepting a backport to beta is usually only done by a team member.

## Configuration

Labeling support is enabled on a repo by having a `[relabel]` table in `triagebot.toml`:

```toml
[relabel]
```

Permissions for allowing unauthenticated labeling is done by listing the labels in the `allow-unauthenticated` list:

```toml
[relabel]
# any label is allowed to be set by team members (anyone on a team in rust-lang/team)
# but these can be set by anyone in the world
allow-unauthenticated = [
    "C-*", # any C- prefixed label will be allowed for anyone, independent of authorization with rust-lang/team
    "!C-bug", # but not C-bug (order does not matter)
]
```

### Aliases

The configuration also supports aliases, a single *word* that is expanded in a set of labels allowing setting multiple labels with a single command, useful when adding or removing the same set of labels over and over. To configure an alias, add to the triagebot the following item:
```toml
[relabel.alias-name]
add-labels = ["Foo", "Bar"]
rem-labels = ["Baz"]
```

`add-labels` and `rem-labels` and arrays of labels that the alias will expand to. For example, given the above configuration:
```
# the command
@rustbot label alias-name

# translates to
@rustbot label +Foo +Bar -Baz
```

Aliases can also be *negative*, inverting the effect:
```
# this command
@rustbot label -cmd-alias

# translates to
@rustbot label +Baz -Foo -Bar
```

You can also mix labels and aliases. Self-canceling labels will be omitted:
```
# this command
@rustbot label cmd-alias +Baz

# translates to:
@rustbot label +Foo +Bar
```

## Implementation

See [`src/handlers/relabel.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/relabel.rs).
