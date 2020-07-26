# Invoicing

Automatically generate a monthly per customer activity report in PDF form for Fishhead Labs. Currently
done manually, I want to change this to be able to run from a cron job/curl call.

## Initial Case

The actual workflow

- Send message "Run Invoices" to TPF
- Upsert DB from Google Docs. Required components:
  - Postgres
  - Subpar (Import/Export)
  - Postgres DB (From a reuse perspective, this should likely be a child instance from Postgres)
- Run submission/org query: If submissions are from unlisted organizations, throw an error and return to the Upsert step after manual fixing
  - Postgres DB
- Filter submissions by finalized since last invoice, loop through each org:
  - Create a new Invoice record with all new submissions, payments, and outstanding invoices.
  - Run local https://stackoverflow.com/questions/46077392/additional-options-in-chrome-headless-print-to-pdf
- Upsert Google Docs with new data

## Work Tasks

Main:

- Make a TPF Listener with exposed API calls (Hyper server)
  - Send Message - Place a message on the message bus for delivery
  - Health-check - Get the status of the entire system or specific components
  - Version - Get the version of all components running
  - Schema - Get the actions/events schema of a registered component version
- Revise the Shell.find to work generically
- Register Local shell with TPF Router.
- Design a simple postgres accessor - Action Query
- Register postgres with TPF Router
- Return query results over message bus
- Enumerate the all the queries that need to be run
- Create Invoice tab in google doc and populate it with past data
- Create Payment tab in google doc and populate it with past data

Stretch:

- Expose Process Foundry/FHL admin to internet (OpnSense + new router)
- Dll Registration (So databases can be created based on schema as well as give the ability to trim the core app)
