# Zulip

[Rust's Zulip](https://rust-lang.zulipchat.com) is used by a number of teams, notably
the compiler, language, and library teams, along with their working groups.

Zulip can be an unintuitive platform to get started with. To get started, take a
look at the [getting started
guide](https://zulipchat.com/help/getting-started-with-zulip). For more detail,
examine the [Zulip user documentation](https://zulipchat.com/help/)!

## Where to go for help with using Zulip

If you're testing a feature, or want to get help, the `#zulip` stream is the
place to go. Like elsewhere, the best thing to do is to create a new topic
for each question.

## Getting started

It is recommended to first look at the official [getting started
guide](https://zulipchat.com/help/getting-started-with-zulip). Like Rust itself,
Zulip is a bit special and reading the documentation before digging can be
really helpful.

You'll definitely want to configure the streams that you're subscribed to when
getting started; the default set is quite limited, and there are many groups
that exist beyond it. Subscribing to a stream is very low cost -- it is similar
to being "in" an IRC channel, except that logs are available for all streams,
regardless of subscription status.

It's not necessary to introduce yourself, but feel free to say hello in the
`#new members` stream.

## User groups

User groups can be pinged by anyone with the `@<group>` notation, same as
pinging another user. Groups can be created by anyone, and anyone can join a
group.

Users should feel free to join (or leave) groups on their own. Furthermore,
users should feel free to create groups as needed, though it is currently
expected that this is somewhat rare. You should name your group similar to how
you would name a stream for the same purpose, though groups can be more
fine-grained (or less). For example, `@T-compiler/meeting` currently does not
have a dedicated stream.

## Appropriate conversation

In most streams, you should try to keep conversations related to team business.
The `#general` stream is a bit broader, but even there, discussions should be
closely related to Rust (though may not relate to projects of any particular
team). All channels are expected to be used for discussions related to the Rust
project, though; discussions of (for example) wildlife or sightseeing are not
appropriate.

## Streams

These are similar to "channels" on other platforms (i.e., there should not be
too many). On the other hand, you can choose which streams you subscribe to, so
there can be more than channels on other platforms. Read [Zulip's
documentation](https://zulipchat.com/help/about-streams-and-topics) for more
details.

Streams are appropriate for any Rust official group. For example, working
groups, project groups, teams are all examples of official groups. These should
ideally also be represented in the [team repository](../infra/team-maintenance.md).

### Default streams

**This section is still under debate, and it is not yet clear which direction we
will go. It is non-normative, and should not be used yet for modifications to
the Zulip instance.**

The default set of streams is chosen to allow incoming people to be able to have
at least one place to go that can then, if necessary, direct them to a more
specific location.

Currently that means that every top-level group present on Zulip is by default
visible. Specifically, no stream that contains a `/` will be enabled by default.

Currently this set is:
 * general
 * t-lang
 * t-compiler
 * t-libs
 * project-ffi-unwind
 * project-inline-asm
 * project-safe-transmute
 * rust-survey-2019
 * wg-async-foundations
 * wg-database
 * wg-formal-methods
 * wg-secure-code
 * wg-traits
 * zulip

An alternative, minimalistic, approach is to use:
 * general
 * zulip
 * announce
 * new members

as the default set, which would push people into customizing their default set when
starting out.

### Stream naming

A stream should be named such as `#t-{team}/{group name}`. For example,
`#t-compiler/wg-parallel-rustc`. More levels of nesting are fine, e.g., a
working group might want "subgroups" as well, though you may want to omit the
team name in such a case -- keeping the stream name short is good for usability,
to avoid confusion between different streams which share the same prefix.

If no top-level team exists, or the group spans multiple teams (e.g.,
project-ffi-unwind), then the top level team should be omitted.

Streams should be clearly communicated as being for a specific purpose. That
purpose can be broad, but it should likely include a group of some kind (even if
that group is transient, e.g., people who are having trouble with the rust build
system, or people working on the compiler). Furthermore, we do not currently
intend for this Zulip to be a general place for community projects not
affiliated with the Rust organization; if they wish to use Zulip, it is [free
for open source](https://zulipchat.com/for/open-source/).

When a new stream is created, you should announce it in `#announce`. This is
generally done automatically by Zulip.

## Topics

A topic is attached to every message within a given stream (these are the
subdivisions within streams). Topics are generally transient, and live for as
long as there is active discussion on a topic. Thinking of topics like email
subjects is helpful.

New conversation in a given stream should almost always start in a new topic,
not a preexisting one. Unlike (for example) GitHub issues, you should not
attempt to search for a past topic on the same subject. Do not spend too long on
the name of the topic, either, beyond trying to make it short. Topics should
generally be no longer than 20 characters (loosely two to three words), to make
sure it is visible to users.

You should eagerly fork new discussion topics into fresh topics. Note that this
can be done with the tail of another topic (if accidentally you diverge into
another area of discussion).

To fork from an existing topic, see Zulip's documentation
[here](https://zulipchat.com/help/rename-a-topic).

## Messages

Zulip is a unique platform which combines synchronous and
asynchronous communication in one location. You should not generally expect that
your messages will receive a response quickly, and unlike (for example) Discord,
there is likely not much reason to "re-ping" on a particular issue every few
hours as your message is unlikely to vanish into history, being isolated to a
specific topic.

### Linkifiers

Our Zulip supports a lot of helpful linkifiers, and we're generally happy to add
more on request. See [the
documentation](https://rust-lang.zulipchat.com/help/add-a-custom-linkification-filter)
for the format. Propose one in `#zulip`!

Generally, `github-org/repo#123` works for linking to an issue or PR; the below
list gives a few more "special cased" repositories.

Don't forget that standard Markdown syntax for links also works.

We currently support linking to issues on a few repositories:

* rust-lang/rust with [`#4545`](https://github.com/rust-lang/rust/issues/4545) or `rust#4545`
* rust-lang/rfcs with `RFC#3434` or `rfc#3434`
* rust-lang/async-book with `async-book#2334`
* rust-lang/chalk with `chalk#2334`
* rust-lang/compiler-team with `compiler-team#3433`
* rust-lang/ena with `ena#3434`
* rust-lang/miri with `miri#3434`
* rust-lang/polonius with `polonius#3434`
* rust-analyzer/rust-analyzer with `rust-analyzer#3434`
* rust-lang/rustc-dev-guide with `rustc-dev-guide#3434`
* rust-lang/stdarch with `stdarch#3434`
* rust-lang/team with `team#3434`
* rust-lang/unsafe-code-guidelines with `ucg#3434`

We currently support linking to commits on these repositories:

* rust-lang/rust with 40-character long SHAs, e.g., [`25434f898b499876203a3b95c1b38bad5ed2cc5d`](https://github.com/rust-lang/rust/commit/25434f898b499876203a3b95c1b38bad5ed2cc5d)

## Read-only view

Our Zulip instance has the web-public streams beta feature enabled, and we use
it for all public streams. Please let us or Zulip developers know if there's any
problems with this. The previous solution to the web-public view was the zulip
archive, which now redirects to the web public view.
