# Nodes Demo

Demo Platform for decentralized connection of devices.

Workflow:

1. Startup cli application through `cargo run --bin cli` -> Prints a qrcode to the terminal
2. Startup mobile app and scan the qrcode
3. Write "Hello, cli!" into the textfield in the mobile app and press send.
4. A new line "> Hello, cli!" is printed to the terminal
5. Write "Hello, mobile!" into the terminal
6. The message "Hello, mobile!" is shown in the app.
