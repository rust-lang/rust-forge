# Crater agents

* Source code: [rust-lang/crater][repo]
* Hosted on:
  * `crater-aws-1.infra.rust-lang.org` (behind the bastion -- [how to connect][bastion-connect])
  * `crater-azure-1.infra.rust-lang.org` (behind the bastion -- [how to connect][bastion-connect])
* Maintainers: [pietroalbini]
* [Application metrics][grafana-app] (only available to infra team members).
* Instance metrics (only available to infra team members):
  * [`crater-aws-1.infra.rust-lang.org`][grafana-instance-aws-1]
  * [`crater-azure-1.infra.rust-lang.org`][grafana-instance-azure-1]

## Service configuration

Crater agents are servers with our standard configuration running a Docker
container hosting the agent. A timer checks for updates every 5 minutes, and if
a newer Docker image is present the container will automatically be updated and
restarted. This service is [managed with Ansible][ansible].

## Common maintenance procedures

### Starting and stopping the agent

The agent is managed by the `container-crater-agent.service` systemd unit. That
means it's possible to start, stop and restart it with the usual systemctl
commands:

```
systemctl stop container-crater-agent.service
systemctl start container-crater-agent.service
systemctl restart container-crater-agent.service
```

### Inspecting the logs of the agent

Logs of the agents are forwarded and collected by journald. To see them you can
use journalctl:

```
journalctl -u container-crater-agent.service
```

### Manually updating the container image

The container is updated automatically every 5 minutes (provided a newer image
is present). If you need to update them sooner you can manuallly start the
updater service by running this command:

```
systemctl start docker-images-update.service
```

[repo]: https://github.com/rust-lang/docs.rs
[bastion-connect]: https://github.com/rust-lang/infra-team/blob/master/docs/hosts/bastion.md#logging-into-servers-through-the-bastion
[pietroalbini]: https://github.com/pietroalbini
[grafana-instance-aws-1]: https://grafana.rust-lang.org/d/rpXrFfKWz/instance-metrics?orgId=1&var-instance=crater-aws-1.infra.rust-lang.org:9100
[grafana-instance-azure-1]: https://grafana.rust-lang.org/d/rpXrFfKWz/instance-metrics?orgId=1&var-instance=crater-azure-1.infra.rust-lang.org:9100
[grafana-app]: https://grafana.rust-lang.org/d/WLeJySTZz/crater?orgId=1
[ansible]: https://github.com/rust-lang/simpleinfra/blob/master/ansible/playbooks/crater.yml
