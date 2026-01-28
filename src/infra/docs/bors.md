# Bors

The infrastructure team manages a merge queue bot called ["Bors"][Bors], to be used
for `rust-lang/rust`. The instance is available
at [bors.rust-lang.org], and is backed by the [@bors] GitHub account.

The service is configured [with Terraform][tf], and it's automatically deployed
from the [rust-lang/bors] repository onto our [ECS cluster][ecs].

[@bors]: https://github.com/bors
[Bors]: https://github.com/rust-lang/bors
[bors.rust-lang.org]: https://bors.rust-lang.org
[ecs]: ./ecs-services.md
[rust-lang/bors]: https://github.com/rust-lang/bors
[tf]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/bors/
