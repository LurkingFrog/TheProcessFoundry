# The Process Foundry

A BPMN microservice framework. As with all frameworks, this one sucks. I would welcome the help to make it
suck less.

## The Reason

Most of my projects tend to be automation of wrapping APIs and CLIs (I know, essentiaally the same thing) and
wiring them together. To reduce all the boilerplate, I'm abstracting each one into  set of Actions and Events
that can be securely routed to each other. Hopefully, this hideously complex idea will make things simpler in
the long run.

## The Initial Use Cases

I'm doing the proof of concept on two use cases:

- Continuous backup of an instance of Postgres running in a docker container. Simple enough to write a script
  for, but the inter-process communication between TPF, Docker Compose, Docker, Bash, Postgres, and Google
  Objects is a great place to expose the abstractions needed to make this all happen.
- Code watcher - A simple watcher that rebuilds project systems based on files changed - see watcher.sh for
  what I currently use. Each individual codebase/server should actually be run inside a docker container so
  we can work with everything as a proper microservice.

## TODOs

Too many to list. I'm currently implementing a continuous Postgres backup system to hammer out my ideas.
I'll start tracking these once I'm through the Proof of Concept phase.

## FAQ

- **I don't write rust, can I still use this?**
  Yes, you can. Eventually. The Process Foundry was designed as a microservice framework, so all
  the interfaces, actions, and events are all published publically. If you write a module in a different
  language that adheres to those, it will work.
