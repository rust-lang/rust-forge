# Selection of AWS Regions

The Rust project has deployed a lot of resources on AWS, and most of them are in
`us-west-1`. As we are growing our footprint and expand to more international
locations, we are reconsidering which regions we want to use.

Please note that this is mainly for new resources that we are deploying, such as
new AWS accounts. Existing resources might get migrated, but this is a
significant effort that might not be worth it given our limited time.

## Selection Criteria

We have two criteria that we use to make this decision:

- **Price** - Pricing differs between regions, and we can reduce our costs by
  deploying to cheaper regions.
- **Location** - We want to host our services close to most of our users. But
  given that Rust is used globally, we won't be able to satisfy everyone.

## Price

Looking at the current distribution of our bill, outbound traffic is by far the
most expensive item. This severely limits the price savings we might enjoy by
switching to a cheaper region.

Even if we assume that we will be able to significantly reduce our outbound
traffic cost on AWS (e.g. by moving to Fastly), the difference between regions
is not massive.

## Locations

Because most of our traffic comes from the US, we want to run most of our
infrastructure here. The following regions are interesting to us:

- `us-east-1` or `us-east-2` (cheaper)
- `us-west-1` (already in use)

Services we want to distribute more globally, e.g. the dev-desktops, we also
want to deploy to Europe. Here, the following regions seem the most reasonable:

- `eu-west-1` (cheaper)
- `eu-central-1` (more central location)

## Decision

[We decided](https://rust-lang.zulipchat.com/#narrow/stream/242791-t-infra/topic/meeting.202022-12-12)
to use the following regions for new resources:

- `us-east-2` - Given that most of our infrastructure is hosted in the US, we
  want to use a cheaper region here to benefit at least a little bit.
- `eu-central-1` - Since we're not deploying that many resources to Europe, we
  want to optimize for location here.

When deploying new resources, they should be deployed to `us-east-2` by default.
Only resources that need to be geographically distributed should be deployed to
`eu-central-1`.
