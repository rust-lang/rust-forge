# Repositories

While the [`rust-lang/rust`] repository contains the standard library, the library team also maintains other repositories alongside the [crate maintainers] subteam. Although crate maintainers are given an open invitation to become regular library team members, they are technically a separate team with their own rules for membership, which are generally based upon contributions and vibes instead of formal policy.

[crate maintainers]: https://rust-lang.org/governance/teams/library/#team-crate-maintainers

The following repositories are shared by the entire project:

* [`rust-lang/calendar`] holds all team calendars, including the calendar for libs.
* [`rust-lang/calendar-generation`] as a weird/historical case implements the above and is managed by the whole project.
* [`rust-lang/goals`] holds project goals, some of which may be relevant to libs.
* [`rust-lang/rfcs`] holds RFCs for the project, some of which may be relevant to libs.
* [`rust-lang/rust-forge`] contains this page, along with other project policies.

[`rust-lang/calendar`]: https://github.com/rust-lang/calendar
[`rust-lang/calendar-generation`]: https://github.com/rust-lang/calendar-generation
[`rust-lang/goals`]: https://github.com/rust-lang/goals
[`rust-lang/rfcs`]: https://github.com/rust-lang/rfcs
[`rust-lang/rust-forge`]: https://github.com/rust-lang/rust-forge

The following repositories contain parts of the standard library, managed by libs.

* [`rust-lang/rust`] contains the standard library in addition to the compiler and other tools.
* [`rust-lang/enzyme`] contains the enzyme fork used for [`std::autodiff`].
* [`rust-lang/stdarch`] contains the [`std::arch`] module.
* [`rust-lang/portable-simd`] contains the [`std::simd`] module.[^project-portable-simd]

[`rust-lang/rust`]: https://github.com/rust-lang/rust
[`rust-lang/enzyme`]: https://github.com/rust-lang/enzyme
[`rust-lang/stdarch`]: https://github.com/rust-lang/stdarch
[`rust-lang/portable-simd`]: https://github.com/rust-lang/portable-simd
[`std::autodiff`]: https://doc.rust-lang.org/nightly/std/autodiff/index.html
[`std::arch`]: https://doc.rust-lang.org/nightly/std/arch/index.html
[`std::simd`]: https://doc.rust-lang.org/nightly/std/simd/index.html

The following contain documentation for the libs team:

* [`rust-lang/api-guidelines`] contains API guidelines.
* [`rust-lang/libs-team`] contains meeting minutes, ACPs, and other team-specific tools and documentation.
* [`rust-lang/std-dev-guide`] contains the [standard library developers guide].
* [`rust-lang/wg-allocators`] contains documentation for the allocators working group.[^wg-allocators]

[`rust-lang/api-guidelines`]: https://github.com/rust-lang/api-guidelines
[`rust-lang/libs-team`]: https://github.com/rust-lang/libs-team
[`rust-lang/std-dev-guide`]: https://github.com/rust-lang/std-dev-guide
[`rust-lang/wg-allocators`]: https://github.com/rust-lang/wg-allocators
[standard library developers guide]: https://std-dev-guide.rust-lang.org/

[^project-portable-simd]: Owned by the `project-portable-simd` subteam; only subteam members can merge changes.
[^wg-allocators]: Owned by the `wg-allocators` subteam; only subteam members can merge changes.

The following crates are managed by the crate maintainers subteam:

* [`backtrace-rs`]
* [`cc-rs`]
* [`cmake-rs`]
* [`compiler-builtins`]
* [`ferris-says`]
* [`flate2-rs`]
* [`getopts`]
* [`glob`]
* [`hashbrown`][^hashbrown]
* [`libc`]
* [`libz-sys`]
* [`log`]
* [`pkg-config-rs`]
* [`regex`][^regex]
* [`socket2`]

[`backtrace-rs`]: https://github.com/rust-lang/backtrace-rs
[`cc-rs`]: https://github.com/rust-lang/cc-rs
[`cmake-rs`]: https://github.com/rust-lang/cmake-rs
[`compiler-builtins`]: https://github.com/rust-lang/compiler-builtins
[`ferris-says`]: https://github.com/rust-lang/ferris-says
[`flate2-rs`]: https://github.com/rust-lang/flate2-rs
[`getopts`]: https://github.com/rust-lang/getopts
[`glob`]: https://github.com/rust-lang/glob
[`hashbrown`]: https://github.com/rust-lang/hashbrown
[`libc`]: https://github.com/rust-lang/libc
[`libz-sys`]: https://github.com/rust-lang/libz-sys
[`log`]: https://github.com/rust-lang/log
[`pkg-config-rs`]: https://github.com/rust-lang/pkg-config-rs
[`regex`]: https://github.com/rust-lang/regex
[`socket2`]: https://github.com/rust-lang/socket2

[^hashbrown]: Since [`std::collections::HashMap`] is implemented via `hashbrown`, it is managed by the larger libs team instead of crate maintainers, currently. This may change in the future.
[^regex]: As a historical case, `regex` has a dedicated subteam controlling its membership.

[`std::collections::HashMap`]: https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html
