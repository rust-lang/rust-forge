# Bors

The infrastructure team manages an instance of [Homu] called "Bors", to be used
for `rust-lang/rust`. The instance is available
at [bors.rust-lang.org], and is backed by the [@bors] GitHub account.

The service is configured [with Terraform][tf], and it's automatically deployed
from the [rust-lang/homu] repository onto our [ECS cluster][ecs].

## Maintenance procedures

### Fixing inconsistencies in the queue

Homu is quite buggy, and it might happen that the queue doesn't reflect the
actual state in the repositories.

See [Fixing inconsistencies in the bors queue](./bors/queue-resync.md) for
instructions on how to do this properly.

[@bors]: https://github.com/bors
[Homu]: https://github.com/rust-lang/homu
[bors.rust-lang.org]: https://bors.rust-lang.org
[ecs]: ./ecs-services.md
[rust-lang/homu]: https://github.com/rust-lang/homu
[tf]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/bors/
