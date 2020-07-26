# Database Subscriptions

This is a look into how a single component can have multiple parents, and how we deal with them.

## The Case

A single database can house the data for multiple services, which means each is a parent of the data from a
design perspective. In a lot of cases, each service wants to keep up to date with the most
recent data since the current value of the data may have been changed by someone else.

Some possible solutions to this are

1. Each service polling regularly. This works, but is very network heavy as all the data must be downloaded
   and processed each time a poll is made, regardless whether a change was made.
2. Inter-process notification where each service tells the others of changes that were made. This becomes
   somewhat brittle, as each service has to know about all the others and each has to handle the business
   logic of using the changes.
3. Subscriptions: This uses any successful calls modifying the data to generate
   announcements to any subscribed services. This can minimize traffic, but will increase the workload on the
   for the each change to the DB.

Though there are pros and cons to each method, I'm going to select number 3 as the method to keep all the
services synchronized as it demonstrates can demonstrate [flux pattern](../core/flux.md) well, in addition to
being the most scalable.

## Implementation

Let's take a look at the classic shopping cart problem of having a single item in stock which two customers
want. For this case, I'm going to have 3 different services running: Customer 1, customer 2, and an inventory
server.

TODO: Convert this to a graphic

1. Customer 1: Empty Cart | Add Button, Customer 2: Empty Cart | Add Button, Inventory: In Cart 0 | In Stock 1
2. Customer 1: Adds To Cart
   Customer 1: 1 Item | Remove, Customer 2: Empty Cart | Sold Out, Inventory: In Cart 1 | In Stock 1
3. Customer 1: Removes from cart
   Customer 1: Empty Cart | Add Button, Customer 2: Empty Cart | Add Button, Inventory: In Cart 0 | In Stock 1
4. Customer 2 Adds to Cart
   Customer 1: Empty Cart | Sold Out, Customer 2: 1 Item | Remove, Inventory: In Cart 1 | In Stock 1
5. Inventory adds 10 items
   Customer 1: Empty Cart | Add Button, Customer 2: 1 Item | Change Quantity, Inventory: In Cart 1 | In Stock 11

Since shopping carts are time sensitive, we want the views to always remain as accurate as possible, and even
do things like have the page alert the user at the moment when an item was returned to stock.
