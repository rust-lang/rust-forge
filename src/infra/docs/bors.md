# Bors

The infrastructure team manages an instance of [Homu] called "Bors", to be used
by repositories inside the `rust-lang` organization. The instance is available
at [bors.rust-lang.org], and is backed by the [@bors] GitHub account.

The service is configured [with Terraform][tf], and it's automatically deployed
from the [rust-lang/homu] repository onto our [ECS cluster][ecs].

## Maintenance procedures

### Fixing inconsistencies in the queue

Homu is quite buggy, and it might happen that the queue doesn't reflect the
actual state in the repositories. This can be fixed by pressing the
"Synchronize" button in the queue page. Note that the synchronization process
itself is a bit buggy, and it might happen that PRs which were approved but
failed are re-approved again on their own.

### Adding a new repository to bors

There are multiple steps needed to add a repository to our Bors instance:

1. The [@bors] GitHub account needs to be granted write access to the
   repository.

2. Each CI provider needs to have a single GitHub Check Run to gate on. This is
   not provided by default on GitHub Actions, but it can be simulated with
   these two jobs, which will generate a `bors build finished` check:

   ```yaml
   end-success:
     name: bors build finished
     if: success()
     runs-on: ubuntu-latest
     needs: [ALL, OTHER, JOBS]
     steps:
       - name: Mark the job as successful
         run: exit 0

   end-failure:
     name: bors build finished
     if: "!success()"
     runs-on: ubuntu-latest
     needs: [ALL, OTHER, JOBS]
     steps:
       - name: Mark the job as a failure
         run: exit 1
   ```

   Make sure to replace `[ALL, OTHER, JOBS]` with a list of all the jobs you
   want to gate on.

3. Add the repository name to the `permissions!` macro in the [team
   repository][team-permissions.rs], and grant the `bors.REPOSITORY.review`
   permission to the right people.

3. Add the repository to the `repositories` map in [the Terraform configuration
   file][tf-repos]. This will create a webhook and inject its secret key in the
   bors execution environment.

4. Add the repository to the [Bors configuration][bors-config], taking
   inspiration from other repositories.

[@bors]: https://github.com/bors
[Homu]: https://github.com/rust-lang/homu
[bors-config]: https://github.com/rust-lang/homu/blob/master/cfg.production.toml
[bors.rust-lang.org]: https://bors.rust-lang.org
[ecs]: ./ecs-service.md
[rust-lang/homu]: https://github.com/rust-lang/homu
[team-permissions.rs]: https://github.com/rust-lang/team/blob/master/src/permissions.rs
[tf-repos]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/bors/_config.auto.tfvars
[tf]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/bors/
