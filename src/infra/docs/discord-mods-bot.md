# Discord moderation bot

* Source code: [rust-lang/discord-mods-bot][repo]
* Hosted on: [`rust-ecs-prod` ECS cluster][ecs]
* Maintainers: [technetos]

The bot is hosted on the `rust-ecs-prod` ECS cluster, on the project's AWS
account, with the `discord-mods-bot` service name. Its container image is
stored in a ECR repository with the same name, and its data is stored in the
`shared` RDS PostgreSQL instance.

Automatic deploys are setup from the [rust-lang/discord-mods-bot][repo] GitHub
repository.

The Discord bot account is [`rustbot#4299`][devportal]. [pietroalbini],
[Mark-Simulacrum], [alexcrichton] and [aidanhs] have access to the developer
portal.

## Common maintenance procedures

[Instructions on how to manage ECS services are available here.][ecs]

[repo]: https://github.com/rust-lang/discord-mods-bot
[ecs]: ecs-services.md
[devportal]: https://discordapp.com/developers/applications/615806512790503424
[technetos]: https://github.com/technetos
[pietroalbini]: https://github.com/pietroalbini
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
[alexcrichton]: https://github.com/alexcrichton
[aidanhs]: https://github.com/aidanhs
