# Internal-facing infrastructure announcements

For significant _internal facing_ infrastructure changes or updates, the Infrastructure Team can
announce that change in the infrastructure announcement Zulip channel [`#t-infra/announcements`].
Please keep in mind that we would like to keep that channel to be _low traffic_ and have a _high
signal-to-noise ratio_.

By _internal facing_, we mean infrastructure that primarily is used by or only impacts project team
members and other specific project-internal users (such as [Dev Desktop](./dev-desktop.md) users).

## Syncing membership before posting an infra announcement

Currently, since [`#t-infra/announcements`] is *not* managed through the `team` repo as it is a
public Zulip stream, an infra team member looking to post an infra announcement should first
manually resync the following Zulip groups to the stream:

- `T-all` (all project team members)
- `gsoc-contributors`, since they have access to Dev Desktops.
- `ospp-contributors`, likewise.
- `cloud-compute-users`, likewise.

This can be done in Zulip through:

> Channel Settings > t-infra/announcements > Subscribers > Add subscribers

Note that Zulip admin privileges are needed to perform this action.

## Announcement pinging etiquette

Except for crucial announcements affecting _all_ project members, **avoid** general `T-all` Zulip
group pings. `T-all` members will already be subscribed to the announcement channel unless they
explicitly leave.

Prefer either:

- Just making an announcement topic, without explicitly pinging all project members, or
- Only use *targeted* pings for specific project teams when only specific project teams are
  affected.



[`#t-infra/announcements`]:
    https://rust-lang.zulipchat.com/#narrow/channel/533458-t-infra.2Fannouncements
