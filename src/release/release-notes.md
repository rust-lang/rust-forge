# Preparing Release Notes
The release notes for the next release should be compiled at the beginning of
the beta cycle, 6 weeks ahead of the release.

Clone the [relnotes] utility. This program pulls all pull requests made against
`rust-lang/rust` and `rust-lang/cargo` within the latest release cycle and
prints out a markdown document containing all the pull requests, categorised
into their respective sections where possible, and prints the document to
`stdout`.

Only pull requests that impact stable users of Rust should be included.
Generally, more exciting items go toward the top of sections. Most items are
simply links to the PR that landed them; some that need more explanation have
additional, unlinked text; anything supported by an RFC has an additional RFC
link. Reuse the PR titles or write descriptions as needed for clarity.

Try to keep the language of the document independent of en-US or en-UK, when it
can't be avoided defer to en-US grammar and syntax.

## Pinging `relnotes-interest-group` for relnotes PR and relnotes blog post

Contributors may be interested to help review the relnotes PRs and relnotes
blog posts (e.g. on behalf of their team). They can opt-in to being pinged by
adding themselves to the
[`relnotes-interest-group` marker team][relnotes-interest-group].

When creating a relnotes PR and release blog post, please ping this
notification group via

```
@rustbot ping relnotes-interest-group
```


[relnotes]: https://github.com/rust-lang/relnotes
[relnotes-interest-group]: https://github.com/rust-lang/team/blob/master/teams/relnotes-interest-group.toml
