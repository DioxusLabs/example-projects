# ~~Bluetooth~~ WiFi scanner app

This desktop app showcases the use of async tasks and interaction with the global Tokio runtime from within a Dioxus app. 

Originally, this app scanned for bluetooth, but macOS 13 disabled arbitrary bluetooth scanning, so we can't do that anymore. But we can scan WiFi - however synchronously.

For now, pretend we used async in a separate thread :)


![Demo of app](./demo_small.png)
