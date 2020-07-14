# Nodes Demo

Demo Platform for decentralized connection of devices.

Workflow:

1. Startup cli application through `cargo run --bin server` -> Prints a qrcode to the terminal
2. Startup mobile app and scan the qrcode
3. CLI and Mobile App show, that a connection has sucessfully been established
4. Write "Hello, cli!" into the textfield in the mobile app and press send.
5. A new line "&gt; Hello, cli!" is printed to the terminal
6. Write "Hello, mobile!" into the terminal
7. The line"&lt;Hello, mobile!" is printed to the terminal
8. The message "Hello, mobile!" is shown in the app.

## Roadmap

Different roadmap lists to show whats the goal of this project.

### v0.1.0

1. Implement Flutter mobile app which connects to the server by using the generated rust library.
2. Different levels of communication. Print QR Code with local network address and global reachable address.

### v0.2.0

1. Connect devices through routing library (uses kademlia). This allows connection without knowing meeting in person.