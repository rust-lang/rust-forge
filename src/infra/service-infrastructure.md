# Service Infrastructure

The Rust Infrastructure team maintains several services and bots. Their list can
be found [here][service-catalog]. Questions about infrastructure, including current
status, should go to the [#t-infra Zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/242791-t-infra).

**Our stability guarantees**: many of our services rely on publicly-accessible
storage and APIs, but not all of these are intended for public consumption. At
the moment, **only the resources behind `static.rust-lang.org` are considered
stable**, meaning that those resources will not change without (at least) prior
notice. If you are relying on other parts of the Rust project infrastructure for
your own work, please let the infrastructure team know.

[service-catalog]: https://github.com/rust-lang/infra-team/blob/main/service-catalog/README.md
