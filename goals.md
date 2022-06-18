# Goals  

## Intercommunication between client and user

- **Commands** that the client can send:
    - Type of `lnpkg` to **represent the result** of receiving the command and acting according the message.
    - Command functionalities such as changing their username and more.


- Ability for the client to request information from the server, such as:
    - List of users (*a list of their IDs, that can be used to request that client's identity*).

- *R U alive?*, server's ability to check if a client it's still connected.


## User's *fingerprint*

- Accounts feature:
    - The client would be able to create accounts, making the `server::comm_elements::Client` object have both a session ID, and an account ID.
