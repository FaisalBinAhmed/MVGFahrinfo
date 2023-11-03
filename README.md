# MVG Fahrinfo

MVG Fahrinfo is a CLI tool to keep up-to-date with latest departure times of Munich public transport.
The app is a native binary and uses official (albeit unpublished) MVG API.

It is made with Rust. 🦀

It features:

- Very low resource usage.
- Beautiful terminal interface.
- Automatic refreshing of deartures.
- Searching of stations.
- Saving stations to file.
- Real-life colors for vehicles for easier identification.
- Easily navigable with shortcuts.

## Using

Clone the repository and run `cargo run` in the root directory.
In the first run, it will fetch the stations list from the server and save it to `stations.json` file.

To force update the stations list file, just delete the file and run the app again.
The app will stay open in your terminal and will refresh the departures every 60 seconds.

## Screenshots

![Current Departures in Munich Hauptbahnhof](https://imgur.com/jsHDPsd.png)
![All station list](https://imgur.com/8hVONcX.png)
![Station search](https://imgur.com/7d4Xk6Q.png)
