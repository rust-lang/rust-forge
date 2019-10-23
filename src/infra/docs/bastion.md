# Bastion server

* FQDN: `bastion.infra.rust-lang.org`
* [Ansible playbook][ansible] to deploy this server.
* [Terraform configuration][terraform] to create AWS resources.
* [Instance metrics][grafana] (only available to infra team members).

## Logging into servers through the bastion

To improve the security of our infrastructure it's not possible to connect
directly to a production server with SSH. Instead, all connections must come
from a small server called the "bastion", which only allows connections from a
few whitelisted networks and logs any connection attempt.

To log into a server through the bastion you can use SSH's `-J` flag:

```
ssh -J bastion.infra.rust-lang.org servername.infra.rust-lang.org
```

It's also possible to configure SSH to always jump through the bastion when
connecting to a host. Add this snippet to your SSH configuration file (usually
located in `~/.ssh/config`):

```
Host servername.infra.rust-lang.org
    ProxyJump bastion.infra.rust-lang.org
```

Please remember the bastion server only allows connections from a small list of
IP addresses. Infra team members with AWS access can change the whitelist, but
it's good practice to either have your own bastion server or a static IP
address.

The SSH keys authorized to log into each account are stored in the [simpleinfra
repository][keys]. Additionally, people with sensitive 1password access can use
the master key stored in the vault to log into every account, provided their
connection comes from any whitelisted IP.

## Common maintenance procedures

### Adding a new user to the bastion server

To add a new user to the bastion you need to add its key to a file named
`<username>.pub` in [`ansible/roles/common/files/ssh-keys`][keys], and change
the [Ansible playbook][ansible] adding the user to the list of unprivileged
users. Please leave a comment clarifying which servers the user will have
access to.

Once that's done [apply the playbook][ansible-apply] and [add a new whitelisted
IP address](#updating-the-whitelisted-ips).

### Updating the whitelisted IPs

Due to privacy reasons, all the static IP addresses of team members with access
to the bastion are stored on [AWS SSM Parameter Store][ssm] instead of public
git repositories. To add or update an IP address you can run this command
(taking care of replacing `USERNAME` and `IP_ADDRESS` with the proper values):

```
aws ssm put-parameter --type String --name "/prod/bastion/allowed-ips/USERNAME" --value "IP_ADDRESS/32"
```

If you're adding an IP address instead of updating it, you'll also need to add
the username to the list in [`terraform/services.tf`][allowed-ips] (key
`allowed_users` in the `service_bastion` module).

Once you made all the needed changes you wanted you need to [apply the
Terraform configuration][terraform-apply].

[ansible]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/playbooks/bastion.yml
[terraform]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/services/bastion
[grafana]: https://grafana.rust-lang.org/d/rpXrFfKWz/instance-metrics?orgId=1&var-instance=bastion.infra.rust-lang.org:9100
[keys]: https://github.com/rust-lang/simpleinfra/tree/master/ansible/roles/common/files/ssh-keys
[ansible-apply]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/README.md#executing-a-playbook
[ssm]: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html
[allowed-ips]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/services.tf
[terraform-apply]: https://github.com/rust-lang/simpleinfra/tree/master/terraform#applying-the-configuration
