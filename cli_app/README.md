This is a CLI app intended to simulate the pouring of a beer when looking at the data.  

The idea here tech-wise is to:

- establish a web socket connection with the server,
- take input via CLI eg. the amount poured/sold, the beer id,
- send a ws message to the server:
  - to update the database,
  - handle the db response,
  - send a message to the client to update ui instantly.