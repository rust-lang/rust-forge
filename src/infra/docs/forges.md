# Git Forges

The Rust infrastructure team actually maintains two Git forges with distinct purposes:
[Github] and [Gitlab].

## Github

Github has been the forge of choice for Rust development for many years, and officially,
**it is the only Git forge the Rust Infrastructure team supports for Rust development**.
We provide and maintain features such as:

- Infrastructure as Code (e.g. [team])
- Custom bots (e.g. `triagebot`, `rfcbot`, `bors`, etc)
- Continuous Integration (Github Actions)
- Continuous Deployment (e.g. over AWS)
- Automatic dependency updates
- and many others

## Gitlab

In 2026, the Rust Infrastructure team started supporting some **read-only**
repository mirrors on Gitlab.

We leverage built-in [Gitlab mirroring] capabilities
to sync key repositories `github/rust-lang`  and `gitlab/rust-lang`. The
[sync schedule] is not configurable and it's managed by Gitlab infrastructure, and it 
syncs **only Git commits** between these two forges. There are no plans to sync other
contents between Github and Gitlab, such as issues and pull requests.

These mirrors might serve as an alternative to fetch out recent code in case Git operations
are down for Github, or in case Github is not available for some reason.

[Github]: https://github.com/rust-lang
[Gitlab]: https://gitlab.com/rust-lang/rust
[team]: https://github.com/rust-lang/team
[Gitlab mirroring]: https://docs.gitlab.com/user/project/repository/mirror
[sync schedule]: https://docs.gitlab.com/user/project/repository/mirror/pull/#how-pull-mirroring-works
