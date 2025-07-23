# Bastion server

- FQDN: `bastion.infra.rust-lang.org`
- [Ansible playbook][ansible] to deploy this server.
- [Terraform configuration][terraform] to create AWS resources.
- [Instance metrics][grafana] (only available to infra team members).

## Logging into servers through the bastion

To improve the security of our infrastructure it's not possible to connect
directly to a production server with SSH. Instead, all connections must come
from a small server called the "bastion", which only allows connections from a
few whitelisted networks and logs any connection attempt.

To log into a server through the bastion, use one of the following methods:

- Use SSH's `-J` flag:

  ```console
  ssh -J <username>@bastion.infra.rust-lang.org <username>@servername.infra.rust-lang.org
  ```

- Configure your SSH client to always jump through the bastion when connecting to a host:

  - Add this snippet to your SSH configuration file (usually located in `~/.ssh/config`):

    ```console
    Host servername.infra.rust-lang.org
        ProxyJump <username>@bastion.infra.rust-lang.org
    ```

  - Use SSH:

    ```console
    ssh <username>@servername.infra.rust-lang.org
    ```

The SSH keys authorized to log into each account are stored in the [simpleinfra
repository][keys]. Additionally, people with sensitive 1password access can use
the master key stored in the vault to log into every account.

## Common maintenance procedures

### Adding a new user to the bastion server

To add a new user to the bastion you need to add its key to a file named
`<username>.pub` in [`ansible/roles/common/files/ssh-keys`][keys], and change
the [Ansible playbook][ansible] adding the user to the list of unprivileged
users. Please leave a comment clarifying which servers the user will have
access to.

Once that's done [apply the playbook][ansible-apply].

[ansible]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/playbooks/bastion.yml
[terraform]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/bastion
[grafana]: https://grafana.rust-lang.org/d/rpXrFfKWz/instance-metrics?orgId=1&var-instance=bastion.infra.rust-lang.org:9100
[keys]: https://github.com/rust-lang/simpleinfra/tree/master/ansible/roles/common/files/ssh-keys
[ansible-apply]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/README.md#executing-a-playbook
