# Rust Infrastructure hosting for static websites

The Rust Infrastructure team provides hosting for static websites available for
all Rust teams. This document explains the requirements a website needs to meet
and how to setup one.

## Requirements for hosting websites

* **The website must be managed by a Rust team, or be officially affiliated with
  the project.**  
  The infrastructure team has finite resources and we can't offer hosting for
  community projects.
* **The website’s content and build tooling must be hosted on a GitHub
  repository in either the [rust-lang](https://github.com/rust-lang) or
  [rust-lang-nursery](https://github.com/rust-lang-nursery) organizations.**  
  The infrastructure team must be able to rebuild the website content at any
  time (for example if we need to switch hosting), and having it hosted on a
  GitHub repository inside infra-managed organizations is the best way for us
  to ensure that. Even though we'd prefer for all the repositories to be public
  it's not a requirement.
* **The website must be built and deployed with a CI service.**  
  We have custom tooling built around hosting static websites on our infra, and
  at the moment they work with Travis CI and Azure Pipelines. If you need
  different CI services ask us in advance and we'll adapt the tooling to your
  provider of choice.
* **The website must reach an A+ grade on the
  [Mozilla Observatory](https://observatory.mozilla.org/).**  
  Browsers have multiple security features toggleable only through HTTP
  response headers, and those features enhance users' privacy and prevent
  exploits from working. An A+ grade on the Observatory indicates all the
  important headers are correctly set.
* **The website must be hosted on platforms vetted by the infra team.**  
  We recommend either GitHub Pages or Amazon S3 (in the rust-lang AWS account)
  as the hosting and CloudFront as the CDN, but if you need other platforms
  that's good as long as we consider them secure and reliable.

## Static websites configuration

To avoid limitations of some hosting providers we have setup CloudFront to
enable additional, custom behaviors. These behaviors are configured through a
file named `website_config.json` at the root of the generated website content.

### Adding custom headers

One of the requirements for having a static website hosted by the
infrastructure team is to reach an A+ grade on the [Mozilla
Observatory](https://observatory.mozilla.org/), and that requires custom
headers to be set. To setup custom headers you need to add an `headers` section
to `website_config.json`. This example content includes all the headers
needed to reach grade B on the Observatory (to reach grade A+ a Content
Security Policy is required):

```json
{
    "headers": {
        "Strict-Transport-Security": "max-age=63072000",
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
        "X-XSS-Protection": "1; mode=block",
        "Referrer-Policy": "no-referrer, strict-origin-when-cross-origin"
    }
}
```

### Fixing GitHub Pages redirects

GitHub Pages behaves weirdly when it sits behind CloudFront and it needs to
issue redirects: since it doesn't know the real domain name it will use
`http://org-name.github.io/repo-name` as the base of the redirect instead of
the correct protocol and domain. To prevent this behavior the
`github_pages_origin` key needs to be added to `website_config.json`
with the origin base url as the value (excluding the protocol):

```json
{
    "github_pages_origin": "org-name.github.io/repo-name"
}
```

## Deployment guide

These deployments steps are meant to be executed by a member of the
infrastructure team since they require access to our AWS account.

### Configuring AWS

Create a CloudFront web distribution and set the following properties:

- **Origin Domain Name:** rust-lang.github.io/repo-name
- **Origin Protocol Policy:** HTTPS Only
- **Viewer Protocol Policy:** Redirect HTTP to HTTPS
- **Lambda Function Association:**
    - **Viewer Response:** arn:aws:lambda:us-east-1:890664054962:function:static-websites:4
- **Alternate Domain Names:** your-subdomain-name.rust-lang.org
- **SSL Certificate:** Custom SSL Certificate
    - You will need to request the certificate for that subdomain name through
      ACM (please use the DNS challenge to validate the certificate)
- **Comment:** your-subdomain-name.rust-lang.org

Wait until the distribution is propagated and take note of its
`.cloudfront.net` domain name.

Head over to the domain’s Route 53 hosted zone and create a new record set:

- **Name:** your-subdomain-name
- **Type:** CNAME
- **Value:** the `.cloudfront.net` domain name you saw earlier

Create an AWS IAM user to allow the CI provider used to deploy website changes
to perform whitelisted automatic actions. Use `ci--ORG-NAME--REPO-NAME` (for
example `ci--rust-lang--rust`) as the user name, allow programmatic access to
it and add it to the `ci-static-websites` IAM group. Then take note of the
access key id and the secret access key since you’ll need those later.

### Adding deploy keys

To deploy websites we don’t use GitHub tokens (since they don’t have granular
access scoping) but a deploy key with write access unique for each repository.
To setup the deploy key you need to be an administrator on the repository,
clone the [simpleinfra](https://github.com/rust-lang/simpleinfra) repository
and run this command:

```
$ cargo run --bin setup-deploy-keys rust-lang/repo-name
```

The command requires the `GITHUB_TOKEN` ([you can generate one
here](https://github.com/settings/tokens)) and the `TRAVIS_TOKEN` ([you can see
yours here](https://travis-ci.com/account/preferences)) to be present. It will
generate a brand new key, upload it to GitHub and configure Travis CI to use
it if the repo is active there.

### Configuring Travis CI

To actually deploy the website, this snippet needs to be added to your
`.travis.yml` (please replace the contents of `RUSTINFRA_DEPLOY_DIR` and
`RUSTINFRA_CLOUDFRONT_DISTRIBUTION`):

```yaml
env:
  RUSTINFRA_DEPLOY_DIR: path/to/be/deployed
  RUSTINFRA_CLOUDFRONT_DISTRIBUTION: ABCDEFGHIJKLMN
import:
  - rust-lang/simpleinfra/travis-configs/static-websites.yml
```

You will also need to set the contents of the `AWS_ACCESS_KEY_ID` and
`AWS_SECRET_ACCESS_KEY` environment variables on the Travis CI web UI with the
credentials of the IAM user you created earlier. The secret access key **must**
be hidden from the build log, while the access key id should be publicly
visible.

### Configuring Azure Pipelines

To actually deploy the website, this snippet needs to be added at the top of
your pipeline's YAML file:

```yaml
resources:
  repositories:
    - repository: rustinfra
      type: github
      name: rust-lang/simpleinfra
      endpoint: rust-lang
```

Then you can add this steps when you want to execute the deploy (please replace
the contents of `deploy_dir` and `cloudfront_distribution`):

```yaml
- template: azure-configs/static-websites.yml@rustinfra
  parameters:
    deploy_dir: path/to/output
    # Optional, only needed if GitHub pages is behind CloudFront
    cloudfront_distribution: AAAAAAAAAAAAAA
```

You will also need to set the following environment variables in the pipeline:

* `GITHUB_DEPLOY_KEY`: value outputted when adding the deploy key earlier
  (**secret**)
* `AWS_ACCESS_KEY_ID`: access key ID of the IAM user allowed to invalidate
  CloudFront (public)
* `AWS_SECRET_ACCESS_KEY`: access key of the IAM user allowed to invalidate
  CloudFront (**secret**)
