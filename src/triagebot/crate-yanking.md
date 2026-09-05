# Crate yanking

For security reasons, we want all `rust-lang` crates on crates.io to be owned by the [rust-lang-owner] account, and handle all crate publishing from CI, using [trusted publishing]. That means that individuals and teams from the Rust Project will not own the crates on crates.io. However, we still want to offer them a way to yank or unyank crates easily. This is implemented in triagebot.

[trusted publishing]: https://crates.io/docs/trusted-publishing

## Yanking a crate

If you want to yank some `rust-lang` crate, send a triagebot command `@**triagebot** yank <crate-name> <crate-version>` to a special [channel] on Zulip. Triagebot will then perform the yank for you.

For this to work, the given crate must be registered in the [team] database, in a `[[crates-io]]` section of some repository. Furthermore, you must be a member of at least one Rust team that is marked as an owner of the crate in this `[[crates-io]]` section.

> Note that the team crate ownership will not be visible on crates.io, it is only visible in the `team` repo.

## Unyanking a crate

For unyanking, the mechanism is the same, but the command is `@**triagebot unyank <crate-name> <crate-version>`.

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Crate.20yanking.2Funyanking
[team]: https://github.com/rust-lang/team
[rust-lang-owner]: https://crates.io/users/rust-lang-owner
