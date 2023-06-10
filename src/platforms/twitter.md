# Twitter

The Rust project has a number of official Twitter accounts, credentials for which are
currently maintained by the infrastructure team.

* [`@rustlang`](https://twitter.com/rustlang)
* [`@RustStatus`](https://twitter.com/RustStatus)
* [`@cratesiostatus`](https://twitter.com/cratesiostatus)
* [`@rustasync`](https://twitter.com/rustasync)

## Twitter Guidelines

The project runs the Twitter account [@rustlang](https://twitter.com/rustlang)
The account is handled by a small team of volunteers.

The account will mostly tweet links to the Rust blog and Rust Insiders blog.
Additionally it will retweet:

* links to blog posts about Rust, retweeting the original author if possible
* questions about Rust, so all followers can help
* Meetup or conference announcements
* announcements of new Rust projects
* anything else relevant

We will not retweet:

* content that bashes other programming languages/projects or is otherwise unconstructive in its discussion of language/tech choice
* Personal announcements ("Today I start my job at $COMPANY writing Rust")
* Learning Rust updates ("Today I started to learn Rust")

The Direct Messages are open to everyone.
If someone wants something retweeted, they should send the tweet via DM.
The vast majority of these things should be retweeted, keeping it to the above rules.
Requests of an author via DM or Tweet to not retweet something will be honored.

Additionally account handlers may look through the [#rustlang](https://twitter.com/hashtag/rustlang?src=hashtag_click) hashtag for noteworthy content.

The account will only follow a small number of Project-owned/related Twitter accounts.
At the time of writing (February 2022) this is only [@cratesiostatus](https://twitter.com/cratesiostatus) and [@rust_foundation](https://twitter.com/rust_foundation).

### Access

Currently access to all four accounts is granted together via a 1password
vault; we don't split this into more fine-grained access. Some automation uses
API keys of the status accounts to automatically tweet about upcoming events on
crates.io.

Access is limited to a small set of folks in the
[twitter](https://github.com/rust-lang/team/blob/master/teams/twitter.toml)
marker team; this isn't automated (changes should ping infra admins for provisioning access).

People with access to 1password should:

* Never change the password or take other administrative action (this is only
  to be done by infra admins)
* Exclusively use the project-hosted instance to keep a copy of the password
  (don't save it to any other password database, including in browser)
* Never share the password with others (even if they're in the list)
  * All access should always go through regular channels to ensure we're not
    accidentally leaking the password by passing it through unsecure channels
    (e.g., email)
* Be aware that the password may change regularly (requiring re-authorization)

If you believe you should have access, please file a PR against the team
repository requesting it and note in the description that you've read this
policy.
