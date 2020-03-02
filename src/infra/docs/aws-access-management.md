# AWS access management

This document explains how to setup and manage AWS access for Rust team
members. If you're a team member and you need to access AWS with your existing
credentials, or you have received your credentials for the first time, check
out the ["AWS access for team members"](./aws-access.md) page.

## Granting access

To grant access to a person, go to [`team-members-access/_users.tf`][tf] in the
Terraform configuration and add the new user to it, specifying which teams they
should be a member of. The user will be created as soon as you apply the
configuration.

By default, there will be no credentials attached to the user. To allow the
user to log in, go to the [IAM console][iam-users], open the security
credentials page of the user you just created, and enable a console password.
Let AWS generate a random one, and require the password to be changed on first
login.

Finally communicate to the user that they can join with the generated password,
and to follow the ["AWS access for team members"](./aws-access.md) page to
learn how to enable 2FA and gain access to their account.

## Revoking access

To revoke access from a person, log into the [IAM console][iam-users],
open the security credentials page of the user you want to delete, and:

* Disable console access by clicking "Manage" on the console password
* Disable 2-factor authentication by clicking "Manage" on the assigned MFA
  device
* Remove all the access keys, including inactive ones, by clicking the "x".

Once all the access was removed from the console, go to
[`team-members-access/_users.tf`][tf] in the Terraform configuration, remove
the user and apply the configuration.

[iam-users]: https://console.aws.amazon.com/iam/home#/users
[tf]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/team-members-access/_users.tf
