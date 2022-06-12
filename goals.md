# Goals  

## Intercomunication between client and user

- **Commands** that the client can send:
    - Type of `lnpkg` to **represent the result** of receiving the command and acting according the message.
    - Command functionalities such as change their username.


- Broadcast event messages such as:
    - A **user connecting** to the server.
    - A **user leaving** the server.

- Client ability to request information from the server, such as:
    - It's own identity (*eg. client id, name...*).
    - Other's identity.
    - List of users (*a list of their IDs, that can be used to request that client's identity*).

- *R U alive?*, the server being able to send pings to the client to test its connectivity.