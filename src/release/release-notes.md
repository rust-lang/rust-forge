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

Try to keep the spelling of the document to en-US.

[relnotes]: https://github.com/rust-lang/relnotes

