# Database maintenance

There are times when Heroku needs to perform a maintenance on our database
instances, for example to apply system updates or upgrade to a newer database
server.

We must **not** let Heroku run maintenances during the maintenance window to
avoid disrupting production users (move the maintenance window if necessary).
This page contains the instructions on how to perform the maintenance with the
minimum amount of disruption.

# Primary database

Performing maintenance on the primary database requires us to temporarily put
the application in read-only mode. Heroku performs maintenances by creating a
hidden database follower and switching over to it, so we need to prevent writes
on the primary to let the follower catch up.

Maintenance should take less than 5 minutes of read-only time, but we should
still announce it ahead of time on our status page. This is a sample message we
can use:

> The crates.io team will perform a database maintenance on YYYY-MM-DD from
> hh:mm to hh:mm UTC.
>
> We expect this to take less than 5 minutes to complete. During maintenance
> crates.io will only be available in read-only mode: downloading crates and
> visiting the website will still work, but logging in, publishing crates,
> yanking crates or changing owners will not work.

## Primary database checklist

**1 hour before the maintenance**

1. Go into the Heroku Scheduler and disable the job enqueueing the downloads
   count updater. You can "disable" it by changing its schedule not to run
   during the maintenance window. The job uses a lot of database resources, and
   we should not run it during maintenance.

**5 minutes before the maintenance**

2. Scale the background worker to 0 instances:

   ```
   heroku ps:scale -a crates-io background_worker=0
   ```

**At the start of the maintenance**

3. Update the status page with this message:

   > Scheduled maintenance on our database is starting.
   >
   > We expect this to take less than 5 minutes to complete. During maintenance
   > crates.io will only be available in read-only mode: downloading crates and
   > visiting the website will still work, but logging in, publishing crates,
   > yanking crates or changing owners will not work.

3. Configure the application to be in read-only mode without the follower:

   ```
   heroku config:set -a crates-io READ_ONLY_MODE=1 DB_OFFLINE=follower
   ```

   The follower is removed because while Heroku tries to prevent connections to
   the primary database from failing during maintenance we observed that the
   same does not apply to the follower database, and there could be brief
   periods while the follower is not available.

3. Wait for the application to be redeployed with the new configuration:

    ```
    heroku ps:wait -a crates-io
    ```

3. Run the database maintenance:

   ```
   heroku pg:maintenance:run --force -a crates-io
   ```

1. Wait for the maintenance to finish:

    ```
    heroku pg:wait -a crates-io
    ```

3. Confirm all the databases are online:

   ```
   heroku pg:info -a crates-io
   ```

3. Confirm the primary database fully recovered (should output `false`):

   ```
   echo "SELECT pg_is_in_recovery();" | heroku pg:psql -a crates-io DATABASE
   ```

3. Switch off read-only mode:

   ```
   heroku config:unset -a crates-io READ_ONLY_MODE
   ```

   **WARNING:** the Heroku Dashboard's UI is misleading when removing an
   environment variable. A red badge with a "-" (minus) in it means the
   variable was *successfully removed*, it doesn't mean removing the variable
   failed.  Failures are indicated with a red badge with a "x" (cross) in it.

3. Wait for the application to be redeployed with the new configuration:

    ```
    heroku ps:wait -a crates-io
    ```

3. Update the status page and mark the maintenance as completed with this
   message:

   > Scheduled maintenance finished successfully.

   The message is posted right now and not at the end because this is when
   production users are not impacted by the maintenance anymore.

3. Scale the background worker up again:

   ```
   heroku ps:scale -a crates-io background_worker=1
   ```

3. Confirm the follower database is available:

   ```
   echo "SELECT 1;" | heroku pg:psql -a crates-io READ_ONLY_REPLICA
   ```

3. Enable connections to the follower:

   ```
   heroku config:unset -a crates-io DB_OFFLINE
   ```

3. Re-enable the background job disabled during step 1.

# Follower database

Performing maintenance on the follower database doesn’t require any external
communication nor putting the application in read-only mode, as we can just
redirect all of the follower’s traffic to the primary database. It shouldn’t be
done during peak traffic periods though, as we’ll increase the primary database
load by doing this.

## Follower database checklist

**At the start of the maintenance**

1. Configure the application to operate without the follower:

    ```
    heroku config:set -a crates-io DB_OFFLINE=follower
    ```

1. Wait for the application to be redeployed with the new configuration:

    ```
    heroku ps:wait -a crates-io
    ```

1. Start the database maintenance:

    ```
    heroku pg:maintenance:run --force -a crates-io READ_ONLY_REPLICA
    ```

1. Wait for the maintenance to finish:

    ```
    heroku pg:wait -a crates-io READ_ONLY_REPLICA
    ```

1. Confirm the follower database is ready:

    ```
    heroku pg:info -a crates-io
    ```

1. Confirm the follower database is responding to queries:

    ```
    echo "SELECT 1;" | heroku pg:psql -a crates-io READ_ONLY_REPLICA
    ```

1. Enable connections to the follower:

    ```
    heroku config:unset -a crates-io DB_OFFLINE
    ```

1. Wait for the application to be redeployed with the new configuration.

    ```
    heroku ps:wait -a crates-io
    ```
