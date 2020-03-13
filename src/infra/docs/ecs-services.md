# ECS services management

Some applications running on the project's infrastructure are hosted in ECS
clusters on our AWS account. This document explains the common maintenance
procedures one should follow when operating them. Most of the actions explained
here [require AWS access][aws-access].

> **Note:** our ECS cluster is located in the Northern California
> (`us-west-1`) AWS region. Make sure it's the selected region when interacting
> with the AWS console.

## Inspecting the logs

Logs for applications hosted on ECS are stored in CloudWatch Logs, and can
be inspected in the AWS Console. [Open the console][console-access], go to
CloudWatch Logs and select the log group called `/ecs/<service-name>`. There
are two ways to inspect the logs:

* If you need to look at the application as a whole, you can get an aggregated
  view by clicking the "View all log events" button (or, on the classic
  interface, "Search Log Group").

* If you need to debug a specific instance of a container, separate log streams
  for each running task are available. The streams are named after the
  container name and the task ID.

Logs are periodically purged (retention varies based on the specific
application).

## Restarting an application

To restart an application, you can force a new deployment without actually
pushing any new code beforehand. To do so, run this command:

```
aws ecs update-service --cluster rust-ecs-prod --service <service-name> --force-new-deployment
```

## Deploying application changes

Each application stores its own Docker container in a [ECR repository][ecr] in
our AWS account. You can deploy changes both manually and automatically (with
GitHub Actions).

For production applications it's recommended to setup automatic deployment.

### Manual deployments

To manually deploy a local build you first need it to tag your built image
with its ECR name:

```
docker tag <image-tag> 890664054962.dkr.ecr.us-west-1.amazonaws.com/<repository-name>:latest
```

Then you can authenticate with ECR and push it:

```
$(aws ecr get-login --no-include-email --region us-west-1)
docker push 890664054962.dkr.ecr.us-west-1.amazonaws.com/<repository-name>:latest
```

Finally, you need to force a new deployment of the ECS service with:

```
aws ecs update-service --cluster rust-ecs-prod --service <service-name> --force-new-deployment
```

### Automatic deployments with GitHub Actions

The infrastructure team prepared an action for GitHub Actions that automates
deployments from CI. To use it, ask a team member to setup AWS credentials in
your repository, and then add this snippet to your workflow:


```
- name: Build the Docker image
  run: docker build -t deploy-image .

- name: Deploy to production
  uses: rust-lang/simpleinfra/github-actions/upload-docker-image@master
  with:
    image: deploy-image
    repository: <ecr-repository-name>
    region: us-west-1
    redeploy_ecs_cluster: rust-ecs-prod
    redeploy_ecs_service: <service-name>
    aws_access_key_id: "${{ secrets.AWS_ACCESS_KEY_ID }}"
    aws_secret_access_key: "${{ secrets.AWS_SECRET_ACCESS_KEY }}"
  if: github.ref == 'refs/heads/<deploy-branch>'
```

Be sure to replace `<ecr-repository-name>`, `<service-name>` and
`<deploy-branch>` with the correct values for your workflow. Once the workflow
changes are merged in the branch you chose for deploys, any future commits
pushed there will be deployed to the ECS cluster.

[aws-access]: aws-access.md
[console-access]: aws-access.md#using-the-aws-console
[ecr]: https://aws.amazon.com/ecr/
