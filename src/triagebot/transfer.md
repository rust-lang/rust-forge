# Issue Transfer

The `transfer` command allows you to transfer a GitHub issue from one repository to another.

## Usage

To transfer an issue to another repository, enter a comment with the form:

`@rustbot transfer <repository-name>`

It is recommended to also include a comment explaining why you are transferring. For example:

```
Transferring to rust-lang/cargo since this is an issue with how cargo
implements diagnostic reports.

@rustbot transfer cargo
```

**IMPORTANT: There will be no visual indication that the issue is being transferred.** Due to GitHub API limitations, you will not see any activity. **You must reload the page** to view the issue in its new location. It may take a few moments for GitHub to transfer all the data.

**WARNING:** Transferring is a partially destructive command. For example, labels and milestones that don't exist in the target repository will be removed from the issue.

The transfer command is limited to team members of the rust-lang org, and transfers can only happen to repositories in the rust-lang org.

## Configuration

The source repository must have an empty `transfer` table to enable transfers *from* that repository. Issues can be transferred to any repository in the rust-lang org.

```toml
[transfer]
```

## Implementation

See [`parser/src/command/transfer.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/transfer.rs) and [`src/handlers/transfer.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/transfer.rs).
