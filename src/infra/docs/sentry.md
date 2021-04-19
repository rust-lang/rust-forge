# Sentry

The infrastructure team manages a [Sentry](https://sentry.io) organization on
`sentry.io` for the Rust Team to use. The instance is generously sponsored by
Sentry, and this document explains how to use it.

## Log into the instance

Every member of the `rust-lang` GitHub organization can authenticate in our
Sentry instance, using their GitHub credentials. Visit [the authentication
page][auth], click the "Single Sign-On" tab and enter the `rust-lang`
Organization ID. You'll be then prompted to log with your GitHub Account!

If this is the first time signing into our Sentry organization, you might have
to [request access to the teams you're on][teams]. Once you request access, a
member of the infrastructure team will approve it.

## Request a new project

If you're a member of a Rust Team and you want to use Sentry for a project your
team manages, you need to follow these steps:

1. If the project is public facing (i.e. people outside the team are supposed
   to access it) you need to contact the Core team to request support in
   amending the privacy policy, adding a note that your service is using Sentry
   too similar to the existing ones.

2. Once the privacy policy is sorted out (whenever needed), you can contact the
   infrastructure team to create a new project in the Sentry interface and
   potentially a new Sentry team.

3. Finally, you can integrate the Sentry SDK with your project.

## Creating a new project

This section documents how the infrastructure team can actually create new
projects when requested. You need to either have a personal Sentry account with
"Owner" permissions, or access to the Sensitive 1Password vault (where the
admin credentials are stored).

To create a project, authenticate in Sentry and visit the [create new project
page][create]. Pick the technology stack the team is using, a relevant name and
the team responsible for it (you can create new teams by clicking the "+"
icon). Finally, if you created a new team, add the relevant people to it.

[auth]: https://sentry.io/auth/login/
[teams]: https://sentry.io/settings/rust-lang/teams/
[create]: https://sentry.io/organizations/rust-lang/projects/new/
