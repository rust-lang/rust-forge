# Rust Blog Guidelines

## Context

The Rust project maintains two blogs.
The “main blog” ([blog.rust-lang.org](https://blog.rust-lang.org/index.html))
and a “inside Rust blog”
([blog.rust-lang.org/inside-rust](https://blog.rust-lang.org/inside-rust/)).
This document provides the guidelines for what it takes to write
a post for each of those blogs, as well as how to propose a post and to choose which blog is most
appropriate.

## How to select the right blog: audience

So you want to write a Rust blog post, and you’d like to know which blog you should post it on.
Ultimately, there are three options:

- The main Rust blog
    - Suitable when your audience is “all Rust users or potential users”
    - Written from an “official position”, even if signed by an individual
- The inside Rust blog
    - Suitable when your audience is “all Rust contributors or potential contributors”
    - Written from an “official position”, even if signed by an individual
- Your own personal blog
    - Everything else

There are two key questions to answer in deciding which of these seems right:

- Are you speaking in an “official capacity” or as a “private citizen”?
- Who is the audience for your post?

In general, if you are speaking as a “private citizen”, then you are probably best off writing on
your own personal blog.

If, however, you are writing in an **official capacity**, then one of the Rust blogs would be a
good fit. Note that this doesn’t mean you can’t write as an individual. Plenty of the posts on
Rust’s blog are signed by individuals, and, in fact, that is the preferred option. However, those
posts are typically documenting the official position of a team — a good example is Aaron Turon’s
classic post on [Rust’s language ergonomics
initiative](https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html). Sometimes, the posts are
describing an exciting project, but again in a way that represents the project as a whole (e.g.,
Manish Goregaokar’s report on [Fearless Concurrency in Firefox
Quantum](https://blog.rust-lang.org/2017/11/14/Fearless-Concurrency-In-Firefox-Quantum.html)).

To decide between the main blog and the inside Rust blog, the question to ask yourself is **who is the
audience** for your post. Posts on the main blog should be targeting **all** Rust users or
potential users — they tend to be lighter on technical detail, and written without requiring as
much context. Posts on the inside Rust blog can assume a lot more context and familiarity with Rust.

## Writing for the Main Rust blog

The Leadership Council ultimately decides what to post on the main Rust blog.

Post proposals describing exciting developments from within the Rust org are welcome, as well as
posts that describe exciting applications of Rust. We do not generally do “promotional
cross-posting” with other projects, however.

### Release note blog posts

One special case are the regular release note posts that accompany every Rust release. These are
managed by the release team and go on the main blog.

The blog posts are published on the same day as the release by the same person in the release team
running the release. Releases always happen on Thursdays.

Before publishing a release post, it goes through a drafting process:

1. The milestone (e.g. for [1.39.0]) for the release is consulted.
2. PRs that we think are sufficiently important are included, and some items are headlined.
   The writing of a blog post typically happens through a [hackmd](https://hackmd.io/) document.
3. Headlined items are sometimes written by different people, and we try to peer-review each
   subsection.
4. The blog post draft is submitted as a PR on the blog repo for final review a few days before the
   release.

[1.39.0]: https://github.com/rust-lang/rust/milestone/66?closed=1

## Inside Rust blogs

Teams can generally decide for themselves what to write on the inside Rust blog.

Typical subjects for inside Rust blog posts include:

- New initiatives and calls for participation
- Updates and status reports from ongoing work
- Design notes

## Approval process

For both the inside Rust and main blogs, we require an approval from:

* Any team lead (top level or not)
* Any leadership council member
* Rust Foundation project director

These are primarily the members of the
[inside-rust-reviewers](https://github.com/rust-lang/team/blob/master/teams/inside-rust-reviewers.toml)
marker team[^1]. Note that this applies to *both* the main and inside Rust blogs
(renaming will happen at some later point).

[^1]: Release team members are also included there for release blog purposes,
  but aren't considered eligible approvers for any random post at this time.

This approval should evaluate:

* Is the tone and content of the post appropriate for the official venue?
  * For example, we should avoid negative commentary about other ecosystems/languages.
* Is it clear on whose behalf the post is written?
  * This may not just be the by-line, but also the langauge used. If the post
    takes official positions on behalf of the Rust Project as a whole, please
    make sure at least one member of the leadership council signs off on it. If the
    post is taking positions on behalf of a team, then that team should be in
    agreement with the content.

In general, it's a good idea to make sure someone (not necessarily the approver
above) has proofread the post, but we generally prioritize unblocking posting
over perfect content for the blog. Note that the above generally does *NOT*
require that this person is a member of the team you're posting on behalf,
though they should be aware of the post.

### Getting a review

Triagebot will automatically assign a leadership council representative to each
new blog PR. That representative is who you should ping if you're not getting a
review promptly, but you may get a faster review from asking team lead(s) for
their review as well. In other words, we recommend escalating to your team
lead(s), then pinging your team's leadership council representative, and
finally the assigned reviewer.

The assigned reviewer is ultimately responsible for making sure a review
happens.

Typically you should expect at least ~1 week of latency on initial review.
Re-review for any final edits or a final merge button press can usually be
promptly driven -- find a member of the group above available when you need to
merge the post.
