# Architecture

This is the top level design of The Process Foundry. Read the sub-headings to learn more about the individual
components layers involved

## Overview

The design is tree based, which each level of node able to only communicate to its parent and children. This
is done to try and simplify routing and reduce network traffic, since most inter-process communication (IPC)
doesn't need to go all the way to the root of the tree.

The Process Foundry is a global root node used to manage all of the children. It is the only node that is
exposed to every descendent. It will handle common items that require inter-process communication such as
routing and security which require all nodes to remain synchronized.

- message queue - Pub/Sub transportation
- registry - Keeps track of the status of each
- routing - Manages the message queue with locations.
- Health-check - Aggregates the health of all descendents
- API
- RBAC/ACL
- Workflow manager
- Child Components

