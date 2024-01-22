# Rust Forge

Welcome to the [Rust Forge]! Rust Forge serves as a repository of supplementary
documentation useful for members of [The Rust Programming Language].

[the rust programming language]: https://rust-lang.org
[rust forge]: https://forge.rust-lang.org

# Development

You can build a local version by installing [mdbook] and running the following command.

```console
mdbook build
```

This will build and run the `blacksmith` tool automatically. When developing
it's recommended to use the `serve` command to launch a local server to allow
you to easily see and update changes you make.

[mdbook]: https://github.com/rust-lang/mdBook

```console
mdbook serve
```

## JavaScript

Forge uses JavaScript to display dates for releases and "no tools breakage
week". When making modifications to the JavaScript, make sure it matches the
[standard] style. You can install `standard` and automatically format the code
using the following commands.

[standard]: https://standardjs.com/index.html

### Install commands

```console
# With Yarn
yarn global add standard
# With NPM
npm install --global standard
```

### Formatting

```console
standard --fix js/
```

# Contributing

## Adding teams

Any Rust team, working group, or project group can have a section in the Rust Forge.
First, please send a PR to add your team to the [`repos/rust-lang/rust-forge.toml`][team-repo] file to give your team permissions.

To add your team to the book, add it to [`src/SUMMARY.md`], like below, replacing `<TEAM_NAME>` with a filesystem- and URL-friendly version of your team's name:

```markdown
- [<TEAM NAME>](src/<TEAM_NAME>/README.md)
```

If you run `mdbook build`, `mdbook` will automatically create the folder and file for your team.

It's recommended that you put general team information in `src/<TEAM_NAME>/README.md`, such as where the meetings happen, repositories that the team manages, links to chat platforms, etc. Larger topics should be made as a subpage, e.g. (`src/release/topic.md`).

```markdown
- [TOPIC](src/<TEAM_NAME>/TOPIC.md)
```

Teams are responsible for merging their own content.
Please add your team to the `[assign.owners]` section of [`triagebot.toml`] so that the bot will auto-assign someone from the team.

[team-repo]: https://github.com/rust-lang/team/blob/master/repos/rust-lang/rust-forge.toml
[`src/SUMMARY.md`]: https://github.com/rust-lang/rust-forge/blob/master/src/SUMMARY.md
[`triagebot.toml`]: https://github.com/rust-lang/rust-forge/blob/master/triagebot.toml

## Maintenance of Rust Forge

The [Rust infra team] is responsible for maintaining the Rust Forge, ensuring that its build and publish system works, and coordinating any technical issues with teams.

[Rust infra team]: https://www.rust-lang.org/governance/teams/infra
