# Simple SMTP Relay Server.

A simple SMTP relay server in rust. The default host is local host and the default port is 2525.

This server listens for the SMTP emails and relays them.


### Procedure
- Start the server. This will make the server listen on the port specified in the config.

    ```> smtprelay-rs -c <CONFIG_PATH>```
- Use netcat to send mail to the server. Use the host and port used in the listener section of the config.

    ```> nc localhost 2525 <<EOT ```
- Construct a basic test email.
    ```
    > HELO yourdomain.com
    > MAIL FROM: test@yourdomain.com
    > RCPT TO: user@example.com
    > DATA
    > It works!
    > .
    > QUIT
    > EOT
    ```

### Note
Some SMTP servers block open relay servers due to spamming. On some inboxes the mail may appear in the spam category.
