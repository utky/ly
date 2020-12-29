# ly

Reinventing the wheel: Pomodoro recorder CLI

## DESIGN NOTES

ly has two mode: server and client.

server mode exposes HTTP and gRPC TCP port to communicate with clients.
HTTP port (8080) accepts access from browser to view timer and visualised metrics.
gRPC port (8081) accepts command from CLI client.

## Setup

Initialize database.

```
$ ly init
```

## Task

List backlog

```
$ ly b
```

```
$ ly b -a
```

List todo

## Pomodoro

```
$ ly start
```

## Legal

Copyright © 2020 FIXME
