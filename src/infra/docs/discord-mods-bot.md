# Discord moderation bot

* Source code: [rust-lang-nursery/discord-mods-bot][repo]
* Hosted on: [`bots.infra.rust-lang.org`][rust-bots] (behind the bastion -- [how to connect][bastion-connect])
* Maintainers: [technetos]

## Service configuration

The service uses a PostgreSQL database called `discord_mods_bot` hosted on the
same server, and connects to it with the `discord_mods_bot` user. Backups are
not yet setup for the database contents.

The service is run with docker-compose on the home of the `ec2-user` user, and
its docker image is hosted on ECR. The image is automatically rebuilt by the
git repository's CI every time a new commit is pushed to master.

The server doesn't expose anything to the outside, as it uses websockets to
communicate with Discord.

The bot is [`rustbot#4299`][devportal]. [pietroalbini], [Mark-Simulacrum],
[alexcrichton] and [aidanhs] have access to the developer portal.

## Common maintenance procedures

### Deploying changes to the bot

Once the CI build on the `master` branch of [the repo][repo] ends you can SSH
into the server and run this command:

```
./redeploy
```

The command might also redeploy other services hosted on the same server.

[repo]: https://github.com/rust-lang-nursery/discord-mods-bot
[rust-bots]: https://github.com/rust-lang/infra-team/blob/master/docs/hosts/rust-bots.md
[bastion-connect]: https://github.com/rust-lang/infra-team/blob/master/docs/hosts/bastion.md#logging-into-servers-through-the-bastion
[devportal]: https://discordapp.com/developers/applications/615806512790503424
[technetos]: https://github.com/technetos
[pietroalbini]: https://github.com/pietroalbini
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
[alexcrichton]: https://github.com/alexcrichton
[aidanhs]: https://github.com/aidanhs
