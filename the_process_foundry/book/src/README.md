# The Process Foundry

Welcome to The Process Foundry (TPF), a business process model and notation (BPMN) based microservice
framework.

Be warned, like all frameworks it will suck. Read ahead to see if it can be made to suck less for you.

The goal of this projects is to create a backbone to implement inter-process communications between disparate
processes on so the developer can focus on their code rather than digging into every niggling decision that
goes into a microservice.

It is designed to be composable, but with sane defaults that the dev will never even know about unless
they go looking.

## Ideas to mull over

THINK: Is it worth creating a book preprocessor to scan the code for ToDo listing to include RFC/ToDos with
links to project management in the book?
THINK: Does Github's project management offer enough to use (even temporarily?)

## ToDo

- Add project management tool to contain bugs/feature requests
- CODE REVIEW for documentation
- Refactor into multiple projects and squash into a proper initial commit
- Design workflows
  - Review Luigi/Airflow docs for ideas
- Implement watcher.sh as TPF workflow (dog-fooding)
  - Create iNotify Application
  - Emit file touched event
  - Does workflow actually do anything after triggering, or does it just configure pub/sub and disappear?
  - Review choreography vs orchestration
- Add ELK stack as a system in order to enable logging
- Create TPF GUI
  - Research compiling WebAssembly container/cache and wrapping it in JavaScript
  - Is Kibana useful here, since it claims to be a dashboard/insight manager?
