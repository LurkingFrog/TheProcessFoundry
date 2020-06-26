# The Process Foundry

A BPMN microservice framework.

## The Reason

Most of my projects tend to be automation of wrapping APIs and CLIs (I know, essentiaally the same thing) and
wiring them together. To reduce all the boilerplate, I'm abstracting each one into  set of Actions and Events
that can be routed to each other. Hopefully, this hideously complex idea will make things simpler in the
long run.

## TODOs

Too many to list. I'm currently implementing a continuous Postgres backup system to hammer out my ideas.
I'll start tracking these once I'm through the Proof of Concept phase.

## FAQ

- **I don't write rust, can I still use this?**
  Yes, you can. Eventually. The Process Foundry was designed as a microservice framework, so all
  the interfaces, actions, and events are all published publically. If you write a module in a different
  language that adheres to those, it will work.
