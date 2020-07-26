# Routing

This root level component handles knowing all the child components and how to calculate next hop. This
should be able to sync between TPF instances, so we can route anywhere based on the message rules.

The pieces that comprise routing

- Routing Parser -> Parse message rules to deliver place
- Pub/Sub bus -> A place to put messages when direct delivery doesn't make sense.
