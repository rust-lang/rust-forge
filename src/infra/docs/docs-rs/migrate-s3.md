If you're running your own instance of docs.rs for personal development, and you'd like to test out the S3 integration, there are a couple steps involved. It's mostly straightforward, but there's at least one major caveat that should be taken into account.

## Requirements

Since docs.rs uses a fixed bucket name to upload files, you'll need to set up an independent server that implements the Amazon S3 API. [Minio](https://min.io/) is an example of a server you can set up. Instructions for installing and configuring Minio (or any S3-compliant provider) are beyond the scope of this article.

## Configuring your docs.rs instance

Once you have your server and credentials set up, you need to tell the docs.rs server to use it. You'll need to add some extra environment variables to the environment file you use to configure docs.rs. It uses the `S3_ENDPOINT` variable to determine where to call, and the other variables to configure its access privileges. The available environment variables are documented in `rusoto`'s [`EnvironmentProvider`](https://docs.rs/rusoto_credential/0.40.0/rusoto_credential/struct.EnvironmentProvider.html).

```text
AWS_ACCESS_KEY_ID=<access key>
AWS_SECRET_ACCESS_KEY=<access secret>
S3_ENDPOINT=<endpoint url>
```

## Migrating files out of the database

Before you restart the process to load the new credentials, the files that are currently in the local database need to be migrated out to S3. Once docs.rs sees that it has AWS credentials in its environment, it will stop reading from the `files` table entirely. Therefore, to properly transition to using the new file storage, you need to run an extra command to upload them. It's helpful to lock the build queue while this is happening, so that no new files will sneak in while the migration happens.

```sh
cratesfyi build lock
# edit the environment file with the above variables if you haven't already
cratesfyi database move-to-s3 # this will take a while, depending on the size of your database
sudo systemctl restart cratesfyi # or however you manage your docs.rs service
# verify that files are loading from S3/Minio/etc
cratesfyi build unlock
```

Once this is done, new crate builds will upload files directly to your file storage service rather than into the database. The `move-to-s3` command will remove rows from the `files` table in the database as it uploads them, so once you're done, you can compact the database to shrink its on-disk size.