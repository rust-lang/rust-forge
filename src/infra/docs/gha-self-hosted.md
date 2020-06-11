# Custom GitHub Actions runners

The Infrastructure Team manages a pool of self-hosted GitHub Actions runners,
meant to be used by whitelisted repositories that need to run tests on
platforms not supported by the GitHub-hosted runners. We're currently running
the following machines:

* `ci-arm-1.infra.rust-lang.org`: AArch64 runners, hosted on [packet]
  ([configuration][host_vars-ci-arm-1]).

The server configuration for the runners is managed with Ansible ([playbook],
[role]), and the source code for the tooling run on the server is in the
[gha-self-hosted] repository.

Please get in touch with the Infrastructure Team if you need to run builds on
this pool for your project in the `rust-lang` organization.

## Maintenance procedures

### Changing the instances configuration

The set of instances available in each host is configured through the
Ansible configuration located in the [simpleinfra repo][simpleinfra]:

```
ansible/envs/prod/host_vars/{hostname}.yml
```

You'll be able to add, remove and resize instances by changing that file and
applying the changes:

```
ansible/apply prod gha-self-hosted
```

### Forcing an update of the source code

The server checks for source code updates every 15 minutes, but it's possible
to start such check in advance. You need to log into the machine you want to
act on, and run the following command:

```
sudo systemctl start gha-self-hosted-update
```

If the contents of the `image/` directory were changed, an image rebuild will
also be started. The new image will be used by each VM after they finish
processing the current job.

### Forcing a rebuild of the images

The server automatically rebuilds the images every week, but it's possible to
rebuild them in advance. You need to log into the machine you want to act on,
and run the following command:

```
sudo systemctl start gha-self-hosted-rebuild-image
```

### Managing the lifecycle of virtual machines

Each virtual machine is assigned a name and its own systemd unit, called
`gha-vm-{name}.service`. For example, the `arm-1-1` VM is managed by the
`gha-vm-arm-1-1.service` systemd unit. You can stop, start and restart the
virtual machine by stopping, starting and restarting the systemd unit.

Virtual machines are configured to restart after each build finishes.

### Logging into the virtual machines

It's possible to log into the virtual machines from localhost to debug builds.
This should be used as the last resort. Each VM binds SSH on a custom port on
the host (configured in the host Ansible configuration), and allows access to
the `manage` user (with password `password`). For example, to log into the VM
with port `2201` you can run:

```
ssh manage@localhost -p 2201
```

Note that the VM image regenerates its own host key every time it boots, so
you'll likely get host key mismatch errors when connecting to a freshly booted
VM.

[gha-self-hosted]: https://github.com/rust-lang/gha-self-hosted
[host_vars-ci-arm-1]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/envs/prod/host_vars/ci-arm-1.infra.rust-lang.org.yml
[packet]: https://www.packet.com
[playbook]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/playbooks/gha-self-hosted.yml
[role]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/playbooks/gha-self-hosted.yml
[simpleinfra]: https://github.com/rust-lang/simpleinfra
