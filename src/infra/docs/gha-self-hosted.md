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

### Updating the GitHub Actions runner version

Our self-hosted CI runs on a [custom fork of the GitHub Actions
runner][rust-lang/gha-runner], which improves the security of the setup. The
fork needs to be manually rebased every time a new version comes out though,
and that needs to be done relatively quickly to prevent CI from
stopping[^self-updates].

Once a new release of [actions/runner] is out, clone [rust-lang/gha-runner] and
fetch the new tag pushed to the upstream repository. Then, rebase the changes
on top of the latest tag:

```bash
git rebase --onto ${NEW_TAG} ${OLD_TAG} ${OLD_TAG}-rust${N}
```

For example, if the new tag is `v2.275.0`, the old tag is `v2.274.2` and there
were two releases of our fork, the command to execute would be:

```bash
git rebase --onto v2.275.0 v2.274.2 v2.274.2-rust2
```

The last commit to rebase *will* conflict, as that commit updates the version
number and the release notes. Add the `-rust1` suffix to the new version number
and remove the description of the changes from the changelog (keeping the
*"Fork of the GitHub Actions runner used by the Rust Infrastructure Team."*
sentence). Once the rebase is complete force-push the commits to `main`.

After you force-push the new commits to `main` you're done! CI will create a
tag, build the release, upload it to GitHub Releases, and automatically push a
commit to [rust-lang/gha-self-hosted][gha-self-hosted] bumping the pinned
runner version to download in the images. The servers will then shortly pull
the latest changes, rebuild the images and restart idle VMs.

[^self-updates]: The GitHub Actions runner really wants to self-update when a
  new release is out, but such updates would prevent our security mitigations.
  Because of that, one of the patches in our fork disable self-updates, but
  that means the runner just stops working until it's updated.

[rust-lang/gha-runner]: https://github.com/rust-lang/gha-runner
[actions/runner]: https://github.com/actions/runner

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

If the contents of the `images/` directory were changed, an image rebuild will
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
