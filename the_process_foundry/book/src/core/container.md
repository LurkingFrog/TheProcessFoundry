# Container

A container is an introspective object that holds a set of applications and other containers (hooray for
recursive definitions). It is used to organize its children and route events/actions to the next step
in both directions, up to the message bus/router, and down to the "bare metal". This is what will enable
app discovery.

TODO: Add PNG of docker-compose example or should this wait until workflows?:

- docker-compose -> docker-container -> shell -> pg_basebackup
- iNotify -> docker-compose -> docker-container -> shell
