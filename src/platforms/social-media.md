# Social media

The Rust project has a number of official social media accounts, credentials for which are
currently maintained by the infrastructure team.

- Fediverse: [`@rust@social.rust-lang.org`](https://social.rust-lang.org/@rust)
- Bluesky: [`@rust-lang.org`](https://bsky.app/profile/rust-lang.org)
- Linkedin: [`rust-lang`](https://www.linkedin.com/company/rust-lang/)
- YouTube: [`@RustVideos`](https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA)
- Twitter/X:
  - [`@rustlang`](https://twitter.com/rustlang) (not used, just kept to prevent squatting)
  - [`@RustStatus`](https://twitter.com/RustStatus) (not used, just kept to prevent squatting)
  - [`@cratesiostatus`](https://twitter.com/cratesiostatus) (used only for status updates)
  - [`@rustasync`](https://twitter.com/rustasync) (not used, just kept to prevent squatting)

These accounts are managed by the [social media team]. The infrastructure team should also
have backup access in the 1password vault.

### Posting guidelines

These accounts will mostly tweet links to the Rust blog and Rust Insiders blog.
Additionally it will retweet:

* links to blog posts about Rust, retweeting the original author if possible
* questions about Rust, so all followers can help
* Meetup or conference announcements
* announcements of new Rust projects
* anything else relevant

We will not repost:

* content that bashes other programming languages/projects or is otherwise unconstructive in its discussion of language/tech choice
* Personal announcements ("Today I start my job at $COMPANY writing Rust")
* Learning Rust updates ("Today I started to learn Rust")

The Direct Messages are open to everyone.
If someone wants something reposted, they should send the content via DM.
The vast majority of these things should be reposted, keeping it to the above rules.
Requests of an author to not repost something will be honored.

Additionally account handlers may look through the `#rustlang` hashtag for noteworthy
content.
These accounts will only follow a small number of Project-owned/related social media
accounts.

### Access

Currently access to all accounts is granted together via a 1password vault; we don't split
this into more fine-grained access. Some automation uses API keys of the status accounts
to automatically tweet about upcoming events on crates.io.

Access is limited to a small set of folks in the [social media team] team; this isn't
automated (changes should ping infra admins for provisioning access).

People with access to 1password should:

* Never change the password or take other administrative action (this is only to be done
  by infra admins)
* Exclusively use the project-hosted instance to keep a copy of the password (don't save
  it to any other password database, including in browser)
* Never share the password with others (even if they're in the list)
  * All access should always go through regular channels to ensure we're not accidentally
    leaking the password by passing it through unsecure channels (e.g., email)
* Be aware that the password may change regularly (requiring re-authorization)

If you believe you should have access, please file a PR against the team repository
requesting it and note in the description that you've read this policy.

[social media team]: https://www.rust-lang.org/governance/teams/launching-pad#team-social-media
