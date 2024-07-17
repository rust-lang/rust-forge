# PR Assignment

Triagebot handles automatic and manual assignment of GitHub PRs.
It also handles welcoming new users when they post a PR.

Rust contributors can track and manage their own work queue using the Zulipchat integration. See [Tracking PR assignment](/triagebot/pr-assignment-tracking.md).

## Usage

Automatic assignment of new PRs is handled by the configuration in the `triagebot.toml`, described [below](#configuration).

Manual assignment can be done by posting a comment on the PR with the text:

* `r? @octocat` --- Assigns a specific user.
* `r? octocat` --- The `@` is optional.
* `r? libs` --- Chooses a random person from the libs ad-hoc group defined in `triagebot.toml`.
  For example, for the [rust-lang/rust](https://github.com/rust-lang/rust) repository, see [`triagebot.toml`](https://github.com/rust-lang/rust/blob/master/triagebot.toml) for a list of ad-hoc group names.
* `r? rust-lang/libs` --- The `rust-lang/` org name prefix is optional.
* `r? rustdoc` --- Chooses a random person from the rustdoc team.
  See the [teams database](https://github.com/rust-lang/team/tree/master/teams) for a list of team names.
* `r? rust-lang/rustdoc` --- The org name prefix is optional.
  It is strongly recommended that you do not use `@`, as that will subscribe and notify the entire team to the PR.

When choosing a user from a team, triagebot only looks at direct team members (it ignores subteams).

When looking up a name, triagebot will first look at ad-hoc groups, then rust-lang teams, and if it doesn't match either of those it assumes it is a GitHub user.

PRs can only be assigned to users with write permissions to the repo, any rust-lang org members with read permissions, or anyone who has commented on the PR.

### Ghost

Using `r? ghost` in the initial PR top-level comment when opening a PR will disable triagebot's auto-assignment.
`ghost` is GitHub's placeholder account for deleted accounts.
It is used here for convenience.
This is typically used for rollups or experiments where you don't want any assignments or noise.

## Configuration

PR assignment is enabled on the repository by having an `[assign.owners]` table in `triagebot.toml`:

```toml
# These are ad-hoc groups that can be referenced in `r?` and the `owners` table below.
# The values may contain GitHub usernames, other groups, or rust-lang teams.
# The `@` is optional.
# Group names should be lowercase.
[assign.adhoc_groups]
libs = ["@joshtriplett", "@Mark-Simulacrum", "@kenntytm", "@m-ou-se", "@thomcc"]
# Can reference other groups.
compiler = ["compiler-team", "compiler-team-contributors"]
compiler-team = ["cjgillot", "estebank"]
compiler-team-contributors = ["compiler-errors", "jackh726"]
# Can reference rust-lang teams.
libs = ["rust-lang/libs-api"]
# This is a special group that will be used if none of the `owners` entries matches.
fallback = ["@Mark-Simulacrum"]

# This specifies users, groups, or teams to assign for different paths.
# Triagebot will pick one person to assign.
# Paths are gitignore-style matches.
[assign.owners]
# Examples of assigning individuals.
"Cargo.lock" = ["@Mark-Simulacrum"]
"/library/std/src/sys/windows" = ["@ChrisDenton"]
# Example of assigning to a group.
"/library/std" = ["libs"]
# Supports gitignore patterns.
"*.js" = ["@octocat"]
# If you want to match all files, `*` should be sufficient.
"*" = ["@octocat"]
# Can use teams from the rust-lang teams database.
"/src/tools/cargo" = ["@rust-lang/cargo"]
```

If the `owners` map is configured, then triagebot will automatically select a reviewer based on which files were modified in the PR.
The existence of the owners table also enables the ability for users to post a comment with `r? name` to set the assignment to a specific user.

### Vacation

If a reviewer wants to temporarily prevent themselves from being assigned (automatically or manually) they can add themselves to the special
`assign.users_on_vacation` group.

```toml
[assign]
users_on_vacation = ["jyn514", "ChrisDenton"]
```

### Additional new PR trigger options

Triagebot will also post a welcome message to the user.
Its behavior depends on a few factors:

* PR authors who have not previously made any commits will get a more detailed welcome message.
* PR authors who have made commits will get an abbreviated message.
* If the initial PR comment has an `r?` command, then no welcome will be posted.

There are several options in `triagebot.toml` for controlling its behavior on new PRs:

```toml
[assign]
# If set, posts a warning message if the PR is opened against a non-default
# branch (usually main or master).
warn_non_default_branch = true
# If set, the welcome message to new contributors will include this link to
# a contributing guide.
contributing_url = "https://rustc-dev-guide.rust-lang.org/contributing.html"
```

Additionally, triagebot will post a comment with a warning if the PR modifies any submodules.

## Implementation

See [`parser/src/command/assign.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/assign.rs) and [`src/handlers/assign.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/assign.rs).
