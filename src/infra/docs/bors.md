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

   These jobs need to run on specific branches (`auto` and `try`) so it's necessary
   to add those branches to the list of branches tested by the CI provider. For GitHub
   Actions that looks like this:

   ```yaml
   on:
      push:
          branches: [ 
            auto,   # Added for bors
            try     # Added for bors
         ]
   ```

3. Add the repository name to the bors permissions array in the [team
   repository][team-permissions.rs], and grant the `bors.REPOSITORY.review`
   permission to the right teams or people. You can see an example of adding
   bors permissions to a team [here][bors-permission].

4. Add the repository to the `repositories` map in [the Terraform configuration
   file][tf-repos]. This will create a webhook and inject its secret key in the
   bors execution environment.

5. Add the repository to the [Bors configuration][bors-config], taking
   inspiration from other repositories. Note that the environment variables used
   in that config will be set automatically as long as you completed step 3 above.

6. Give it a test by commenting `@bors ping` in any PR. If you get a response back,
   you can then try to approve the PR with `@bors r+`.

[@bors]: https://github.com/bors
[Homu]: https://github.com/rust-lang/homu
[bors-config]: https://github.com/rust-lang/homu/blob/master/cfg.production.toml
[bors.rust-lang.org]: https://bors.rust-lang.org
[ecs]: ./ecs-services.md
[rust-lang/homu]: https://github.com/rust-lang/homu
[team-permissions.rs]: https://github.com/rust-lang/team/blob/52b4370214e1c8eabe483f3a26f22733d94b326f/config.toml#L18-L37
[bors-permission]: https://github.com/rust-lang/team/blob/a1532ec2b08c9d40c0a2c7643ffe72de9671e265/teams/wg-compiler-performance.toml#L25-L26
[tf-repos]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/bors/_config.auto.tfvars
[tf]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/bors/
