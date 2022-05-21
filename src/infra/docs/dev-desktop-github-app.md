# Github App for pushing to github from the dev-desktops

These instructions are for server-side setup and debugging of the dev-desktop github app.
The user only needs to be directed to the app installation URL
and everything should just work for them.

We're using the python github library for all github operations.
You can find the docs at https://pygithub.readthedocs.io/en/latest/introduction.html

## How to setup an App

1. Go to https://github.com/settings/apps
2. New Github App
3. Fill out metadata (name and url)
4. disable WebHook checkbox
5. Set `Contents - Repository contents, commits, branches, downloads, releases, and merges.` to read/write
6. Set `Workflows - Update GitHub Action workflow files.` to read/write
7. Set to "enable on any account"
8. Create App
9. Go to https://github.com/settings/apps/{your_app_name_here} and copy the `App ID` into `app_id.txt` (same folder as `gen_temp_access_token.py`)

### How to generate a .pem file for your App

1. Go to https://github.com/settings/apps/{your_app_name_here}#private-key and generate a private key
2. Download starts, save it to somewhere private.
3. copy the .pem file into the same folder as the `gen_temp_access_token.py` and name it `dev-desktop.private-key.pem`

### How to install the app for a user

1. direct the user to https://github.com/settings/apps/{your_app_name_here}/installations
2. let them install it on the org/user they want to and restrict to the repositories they want to use

### How to generate a temporary access token for a specific user

1. invoke `gen_temp_access_token.py <github_username> <github_repository_name>`

## Integration into git command line

We're using [credential-helpers](https://git-scm.com/docs/gitcredentials#Documentation/gitcredentials.txt).
For debugging a credential helper, have it in userspace and invoke it with

`git -c credential.helper -c credential.UseHttpPath=true /path/to/helper push origin branch`

Note that this does not work for remotes that are registered with ssh urls. You must use https!

The first command line argument is `get`, `store` or `remove`.
In our case, we just abort (`exit(0)`) for everything but `get`, as we regenerate credentials on every invocation anyway.

The actual arguments are passed via stdin and usually look like

```
protocol=https
host=github.com
path=your_repo.git
```

