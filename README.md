# SigmaOS Notification

Python CLI and in-memory notification model for sending, listing, closing, and
suppressing notifications. The state model assigns monotonic IDs and bounds
history; the default sender invokes the external `notify-send` command.

## Current integration scope

This repository is not a D-Bus notification daemon and does not export the
`org.freedesktop.Notifications` service. The CLI sends the app, urgency, and
summary through `notify-send`; the body is stored and printed but not passed to
that command. History and do-not-disturb state are stored in a JSON file at
`~/.config/sigma-notify/centre.json` by default. Each CLI invocation creates a
fresh center; active-notification state is neither persisted nor restored.
Consequently, `close` cannot find a notification sent by an earlier CLI
invocation, even though the notification may still be visible in the desktop.

Critical urgency bypasses the model's do-not-disturb suppression. This policy
is local to this implementation and does not override desktop notification
settings.

## Development

`make test` runs deterministic state-model checks without invoking the desktop
session bus. These checks do not establish notification-daemon compatibility.
