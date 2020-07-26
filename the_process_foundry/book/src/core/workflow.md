# Workflow

Status: Thinking through

A workflow is a set of actions to run based on a specific change in state. It should be choreography rather
than orchestration. The bulk of this is going to be to adding routing to make sure events get to the next
proper step. It should be started with a single trigger and using routing to follow to subsequent events instead
of having a process listener waiting for result before sending the next step. I'm not sure this is possible,
but I'm damn well going to try.

TODO: Examine Luigi/Airflow for how they do things
TODO: Workflow design doc
TODO: Workflow example
