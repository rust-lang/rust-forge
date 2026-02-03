# Third-party and Out-of-tree Crates Policy
This policy describes the guidelines for creating out-of-tree crates for use in the compiler and
using third-party crates within the compiler.

## Out-of-tree crates
One of the primary goals of this policy is to ensure that there is consistency in how out-of-tree
crates used in the compiler are set up (at least, those maintained by the compiler team and living
in `rust-lang`) and that the experience is uniform across `rust-lang/rust` and these crates.

### When should parts of the compiler be extracted into an out-of-tree crate?
This is left to the discretion of compiler team members but should be discussed with the rest of
the team, either through raising the question at the weekly triage meeting or asynchronously using
[an approval decision](./proposals-and-stabilization.md). If the crate is a product of a working
group, there should already be agreement within the working group that an out-of-tree crate is
suitable.

When considering creating an out-of-tree crate, it is worth balancing how general the crate should
be with the increased maintenance burden that this may bring if widely used.

### Where should compiler crates live?
Out-of-tree compiler crates should be hosted in the `rust-lang` organization - this simplifies
integration with external infrastructure tooling and will inherit existing team permissions on
GitHub. It should be made clear in any documentation that the compiler team and any appropriate
working groups are responsible for the crate. It is not recommended to start with a prototype in
another organization or personal repository.

### Can existing out-of-tree crates from personal accounts or other organizations be transferred?
Yes, this is encouraged. In order to do this, discuss this with the team and familiarize yourself
with [the GitHub documentation for repository transfers][repo_transfers] and then arrange to perform
the transfer. Once a transfer is complete, a redirect will exist in the original account or
organization and this will conflict with the names of any new forks of the transferred repository -
an email to GitHub is required to resolve this.

[repo_transfers]: https://help.github.com/en/articles/transferring-a-repository

### Who owns these crates?
It is desired that a compiler team member or working group has loose ownership over a crate so
that there are clear owners who are responsible for making sure that new versions are published and
that pull requests are reviewed.

Sometimes a non-project member is the primary owner of a crate and the compiler team are added as
"fallback" maintainers in case the primary maintainer is no longer reachable. This is discouraged
but can be justified on a case-by-case basis.

### What should these crates be named?
Crate naming will be discussed when new out-of-tree crates are proposed to the compiler team.

Crate naming will differ on a case-by-case basis. Crates that are inherently tied to the
compiler would benefit from a name that is prefixed with `rustc_`. This is an indicator of how
stable the crate may be to prospective users. Other crates, which are more general-purpose, will
have names that are disentangled from the compiler.

### Are there any limitations on the review policy for out-of-tree crates?
Generally, the working groups and team members that are primarily free to maintain the crate using
whatever practices are best suited to their group, however, there are some limitations so that there
is some uniformity across the compiler and out-of-tree crates:

- Everyone with r+ on `rust-lang/rust` should be able to review and approve PRs.
- Where possible, only active participants in the crate (or related working group) need be on the
  review rotation for the crate.
- It is fine to have additional reviewers on the crate who do not otherwise have r+ for Rust as a
  whole, if those reviewers are actively involved in the working group or crate maintenance.
- Major pull requests should have multiple reviewers.

### What is required of an out-of-tree crate?
It is required that out-of-tree crates must:

- Be dual-licensed with Apache 2.0 and MIT when maintained by the compiler team (as the compiler
  is) unless there is a compelling reason to do otherwise.
    - If another license is desired, this must be brought up when proposing the new crate for
      compiler team members to agree. Prefer [licenses accepted by tidy][licenses], unless otherwise
      required (ports of code from other projects, etc).
- Abide by Rust's code of conduct.
- Specify that the crate is maintained by the Rust compiler team and any appropriate working groups.
    - In particular, this should detail the expected level of maintenance and stability for any
      prospective users.
    - This should also link to the working group details in this repository.
- Be added to the list at the bottom of this page.
- Follow semantic versioning.
- Use GitHub merge queues and `@triagebot`.
- Use labels that are compatible with the existing triage process. This will allow nominated issues
  in your out-of-tree crate to be discussed during triage meetings.
    - eg. `T-compiler`, `I-compiler-nominated` (a full list is to be decided)

### Is there a requirement for community infrastructure for an out-of-tree crate?
There is no requirement that community infrastructure (such as Zulip servers/streams) be created for
out-of-tree crates. This may be desirable if an out-of-tree crate gains a large community of
contributors and users, but otherwise, the working group or compiler team streams should be used
initially.

Linkifiers for auto-linking to issues and PRs on the primary Rust Zulip server can be added on
request.

### Are there any instructions for working with out-of-tree crates?
See [Using external repositories] in the rustc-dev-guide.

### How should stabilization/semantic changes be handled in out-of-tree crates?
It is important to involve the language team in any changes in out-of-tree crates that would result
in stabilization or semantic changes to the language. Submodule changes in PRs to `rust-lang/rust`
should be labelled appropriately (eg. `relnotes`, `t-compiler`, `t-lang`) just as if the change
were implemented in `rust-lang/rust` directly, include a description of the changes when it is not
obvious to those unfamiliar with the compiler or the out-of-tree crate.

[licenses]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/tidy/src/deps.rs#L10-L19
[Using external repositories]: https://rustc-dev-guide.rust-lang.org/external-repos

---

In summary, the process for establishing an out-of-tree crate is as follows:

1. Where appropriate, discuss and confirm the need within the working group for the out-of-tree
   crate.
1. Create a PR modifying this document to include the crate in the list below. Use
   [`@rfcbot merge`](https://github.com/anp/rfcbot-rs#usage) to gain agreement from compiler
   team members.
1. Create a new repository in the `rust-lang` organization using the teams repository.
    1. Ensure that the `access.teams.compiler` key is set to `'maintain'` so that the team have
       appropriate permissions.
    1. Add a README describing the intended purpose of the crate, which team and working group are
       responsible (link to their page in this repository) and the intended level of maintenance and
       stability.

       > This crate is developed and maintained by the [Rust compiler team](https://github.com/rust-lang/compiler-team/tree/master/procedures) for use within
       > `rustc`, in particular, it is the responsibility of the
       > [`.template`](./working-areas.md) working area. This crate will have regular
       > breaking changes and provides no stability guarantees | is intended to remain stable and have
       > limited breaking changes.
    1. Include the [LICENSE-APACHE][apache] and [LICENSE-MIT][mit] files from `rust-lang/rust`.
    1. Include or link the [CODE_OF_CONDUCT][coc] file from `rust-lang/rust`.
    1. Create a relevant `.gitignore` ([here's a sane default][gitignore]).
    1. Create `P-high`, `P-medium`, `P-low`, `I-compiler-nominated` and `T-compiler` labels.
1. Perform any initial development required before integration with rustc.
1. Publish initial version, following semantic versioning.
1. Add the crate as a dependency to the appropriate in-tree crate and start using.

[gitignore]: https://gitignore.io/api/vim,rust,emacs,clion,visualstudio,visualstudiocode
[triagebot]: https://github.com/rust-lang/rust/blob/HEAD/triagebot.toml
[apache]: https://github.com/rust-lang/rust/blob/HEAD/LICENSE-APACHE
[coc]: https://www.rust-lang.org/policies/code-of-conduct
[mit]: https://github.com/rust-lang/rust/blob/HEAD/LICENSE-MIT

## Third-party crates
It is sometimes desirable to use the functionality of an existing third-party crate in the compiler.

### When can a third-party crate be added as a compiler dependency?
It is desirable that a third-party crate being included in the compiler is well-maintained and that,
where possible, a compiler team member is added as a maintainer. You should consult with the
rest of the compiler team before making this decision.

### What about third-party dependencies to out-of-tree crates?
The same policies apply to all compiler-team-maintained crates used in the compiler.

## List of out-of-tree crates
This section contains the list of existing out-of-tree, compiler team-maintained crates:

  - [`rust-lang/chalk`](https://github.com/rust-lang/chalk/)
  - [`rust-lang/polonius`](https://github.com/rust-lang/polonius/)
  - [`rust-lang/measureme`](https://github.com/rust-lang/measureme/)
  - [`rust-lang/thorin`](https://github.com/rust-lang/thorin/)
