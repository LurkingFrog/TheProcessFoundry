# The Process Foundry

A concept for a BPMN microservice framework with a focus on the developer's user experience (UX). As with all
frameworks, this one sucks. I would welcome the help to make it suck less.  uI'm currently writing smaller 
(ha ... smaller) tools such as Grapht, PivoTable, and NomNomicon.

## The Reason

Most of my projects tend to be automation of wrapping APIs and CLIs (I know, essentiaally the same thing) and
wiring them together. To reduce all the boilerplate, I'm abstracting each one into  set of Actions and Events
that can be securely routed to each other. Hopefully, this hideously complex idea will make things simpler in
the long run.

## Projects

- Grapht: A distributed, yet local, cache. I've found that once I've gotten the info I need from a server, I
  still need to slice it up and reorganize it based on user clicks, which always needs to be written. Having
  it in shared memory allows for . It's meant to reduce network load by 
  being able to use ephepmeral storage to re-query previously retrieved data.
- PivoTable: Generate WASM code to display pivot tables in a Grapht database based on a YAML configuration.
- NomNomicon - Rust macros using simple YAML defined grammar to build a string/stream parser. I tend to write a lot of DSLs and
  being able to properly document and validate them would be huge.

## The Initial Use Cases

I'm doing the proof of concept on two use cases. 

- Code watcher: A simple watcher that rebuilds project systems based on files changed - see watcher.sh for
  what I currently use. Each individual codebase/server should actually be run inside a docker container so
  we can work with everything as a proper microservice.
- Continuous backup: I need to backup an instance of Postgres running in a docker container. Simple 
  enough to write a script for, but the inter-process communication between TPF, Docker Compose, Docker, 
  Bash, Postgres, and Google Objects is a great place to expose the abstractions needed to make this all happen.

## TODOs

Too many to list. I'm currently focused on Grapht and PivoTable to build an invoicer. I put a bit of time into
some other side projects that can make use of them that in order to give myself some different use cases.

## Future research

https://github.com/google/tarpc - Remote Process Call (RPC) framework

## FAQ

- **I don't write rust, can I still use this?**
  Yes, you can. Eventually. The Process Foundry was designed as a microservice framework, so all
  the interfaces, actions, and events are all published publically. If you write a module in a different
  language that adheres to those, it will work.
