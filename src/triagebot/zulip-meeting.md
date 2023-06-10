# Zulip Meeting Management

Triagebot can respond to some commands in Zulip to assist with running a meeting.

## Usage

Enter a message in Zulip addressed to `@triagebot` with a command listed below.

### Document reading

```text
@triagebot read
```

This command will cause triagebot to post a comment to poll when everyone is finished reading some document, and are ready to start discussing it.
The message looks something like:

```text
Click on the :book: when you start reading (and leave it clicked).
Click on the :checkered_flag: when you finish reading.
```

Users can then click the emoji reaction buttons to indicate that they are currently reading, and then again when they are finished.

### End topic

```text
@triagebot end-topic
```

This command will cause triagebot to post a comment to poll if everyone in the meeting is ready to move on to the next topic.
The message looks something like:

```text
Does anyone have something to add on the current topic?
React with :working_on_it: if you have something to say.
React with :all_good: if not.
```

Users can then click the emoji reaction buttons to indicate if they are ready or not.

`@triagebot await` is an alias for `end-topic`.

### End meeting

```text
@triagebot end-meeting
```

This command will cause triagebot to post a comment to poll if everyone is ready to end the meeting.
The message looks something like:

```text
Does anyone have something to bring up?
React with :working_on_it: if you have something to say.
React with :all_good: if you're ready to end the meeting.
```

Users can then click the emoji reaction buttons to indicate if they are ready to end or not.

## Configuration

This feature has no configuration, it is available to all team members.
Note that your Zulip ID needs to be configured in the [teams database](https://github.com/rust-lang/team).

## Implementation

See [`src/zulip.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/zulip.rs).
