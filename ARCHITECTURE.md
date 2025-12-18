# Architecture Notes

TODO: Rewrite this in C4-PlantUML.

At first I would like to constriant the system so that
the changes can only be made on the host,
i.e. the other storage devices are READ-ONLY.

Later, all devices will be able to make changes.
Then there's no more distinguishing between a peer and a host,
and a suitable name should be given.

```mermaid
---
title: Code
config:
  flowchart:
    htmlLabels: false
---
architecture-beta
    group g_main(internet)[offsync]

    service host(disk)[Host] in g_main
    service peer1(disk)[Peer 1] in g_main
    service peer2(disk)[Peer 2] in g_main
    service cloud(cloud)[Cloud] in g_main

    junction j_peer1 in g_main
    junction j_peer2 in g_main

    host:T --> B:cloud
    peer1:T <-- B:j_peer1
    j_peer1:R -- L:cloud
    peer2:T <-- B:j_peer2
    j_peer2:L -- R:cloud
```


## Implementation
