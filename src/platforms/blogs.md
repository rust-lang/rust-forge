# Rust Blog Guidelines

## Context

The Rust project maintains two blogs. The “main blog” (blog.rust-lang.org) and a “team blog”
(blog.rust-lang.org/inside-rust). This document provides the guidelines for what it takes to write
a post for each of those blogs, as well as how to propose a post and to choose which blog is most
appropriate.

## How to select the right blog: audience

So you want to write a Rust blog post, and you’d like to know which blog you should post it on.
Ultimately, there are three options:

- The main Rust blog
    - Suitable when your audience is “all Rust users or potential users”
    - Written from an “official position”, even if signed by an individual
- The team Rust blog
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

To decide between the main blog and the team blog, the question to ask yourself is **who is the
audience** for your post. Posts on the main blog should be targeting **all** Rust users or
potential users — they tend to be lighter on technical detail, and written without requiring as
much context. Posts on the team blog can assume a lot more context and familiarity with Rust.

## Writing for the Main Rust blog

The Leadership Council ultimately decides what to post on the main Rust blog.

Post proposals describing exciting developments from within the Rust org are welcome, as well as
posts that describe exciting applications of Rust. We do not generally do “promotional
cross-posting” with other projects, however.

If you would like to propose a blog post for the main blog,
please reach out to a [Leadership Council member](https://www.rust-lang.org/governance/teams/leadership-council).
It is not suggested to just open PRs against the main Rust blog that add posts without first discussing it with a Leadership Council member.

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

## Team Rust blogs

Teams can generally decide for themselves what to write on the team Rust blog.

Typical subjects for team Rust blog posts include:

- New initiatives and calls for participation
- Updates and status reports from ongoing work
- Design notes

To propose a blog post for the team blog of a particular team, reach out to the team lead or some
other team representative.
