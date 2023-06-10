# Notifications

The notifications system helps a user keep track of GitHub notifications.

## Usage

Each registered team member has a notifications page at:

`https://triage.rust-lang.org/notifications?user=<github-username>`

Whenever you are mentioned on GitHub with a direction mention (`@user`) or via a team mention (`@rust-lang/libs`) anywhere in the rust-lang organization, this will add an entry to the notifications list.

The notifications list can also be edited via Zulip by [private-messaging triagebot](https://rust-lang.zulipchat.com/#narrow/pm-with/261224-triage-rust-lang-bot).
Any Rust organization member can edit their notifications page, or pages of other Rust organization team members.
To do so, the editor must have a `zulip-id` listed in their `people/username.toml` file in the [team repository](https://github.com/rust-lang/team/).
The bot will tell you which ID to use when talking to it for the first time; please `r? @Mark-Simulacrum` on PRs adding Zulip IDs.

The following commands are supported:

 * `acknowledge <url>` (or short form `ack <url>`)
 * `acknowledge <idx>` (or short form `ack <idx>`)

These both acknowledge (and remove) a notification from the list.

 * `acknowledge all` or `acknowledge *` (or short form `ack all` or `ack *`)

This acknowledges and removes all notifications.

 * `add <url> <description... (multiple words)>`

This adds a new notification to the list.

 * `move <from> <to>`

This moves the notification at index `from` to the index `to`.

 * `meta <idx> <metadata...>`

This adds some text as a sub-bullet to the notification at `idx`. If the metadata is empty, the text is removed.

 * `as <github username> <command...>`

This executes any of the above commands as if you were the other GitHub user.

## Configuration

There is no configuration for this feature.

## Implementation

See [`src/handlers/notification.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/notification.rs),
[`src/notification_listing.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/notification_listing.rs), and
[`src/db/notifications.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/db/notifications.rs).
