# Application

An application is an interface which defines a set of functions to convert TPF actions into native commands,
monitor state, and sends out messages describing any changes in state.

At its core, an application is something that executes actions and emits events. It has no context of anything
outside the scope of its functionality. It can be accessed by one or more containers, depending on the
context.

A primary example of this is the user shell

TODO: Add component/function design png
