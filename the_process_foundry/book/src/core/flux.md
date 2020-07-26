# Data Flow

The dataflow in this system is designed to be unidirectional. An action will will generate an ack event with
the message id. Results are pushed onto the router as multiple applications may care about the result even
though, only a single action was triggered. This simplifies handling state changes and maintaining a cache.

Web pages should also be treated as an equal container, receiving events and updates for realtime data syncing.
The idea is to compile a WebAssembly module and wrap it in JS so web users will be treated as equal to onsite
containers/applications. This hopefully keep the codebase with bug fixes up to date without having to manage
multiple programming languages.

I'm basing this off the idea of [Flux](https://facebook.github.io/flux/docs/in-depth-overview/)

TODO: Add example data flow for the watcher/rebuilder workflow
