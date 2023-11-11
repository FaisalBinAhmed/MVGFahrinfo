# MVG Fahrinfo

MVG Fahrinfo is a CLI tool to keep up-to-date with latest departure times of Munich public transport.
The app is a native binary and uses official (albeit unpublished) MVG API.

It is made with Rust. 🦀

It features:

- Very low resource usage.
- Beautiful terminal interface.
- Automatic refreshing of departures.
- Searching of stations.
- Saving stations to file.
- Real-life colors for vehicles for easier identification.
- Easy navigation with shortcuts.

## Using

Clone the repository and run `cargo run` in the root directory.
In the first run, it will fetch the stations list from the server and save it to `stations.json` file.

To force update the stations list file, just delete the file and run the app again.
The app will stay open in your terminal and will refresh the departures every 60 seconds.

To exit the app, press `q` or `Ctrl+C`.

## Installing

To run it globally, you can install the app with `cargo binstall mvgfahrinfo`. Make sure you have `binstall` [binstall repo](https://github.com/cargo-bins/cargo-binstall) installed. Once installed, you can invoke the app just by running `mvgfahrinfo` in the terminal.
This is a binary crate and not a library, so you shouldn't use it as a dependency.

I might provide some pre-built binaries for Windows/MacOS/Linux in the future. :)

## Shortcuts

### Normal mode

- `tab` - Switch between departures and stations list.
- `r` - Refresh departures.
- `s` - Search for a station.
- `Up/Down` - Navigate through the list of stations.
- `Enter` - Select a station.
- `q` - Quit the app.
- `Ctrl+C` - Quit the app.

### Search mode

- `Esc` - Exit search mode.
- `Up/Down` - Navigate through the list of stations.
- `Enter` - Select a station.

## Screenshots

![Current Departures in Munich Hauptbahnhof](https://imgur.com/jsHDPsd.png)
![All stations list](https://imgur.com/8hVONcX.png)
![Station search](https://imgur.com/7d4Xk6Q.png)

## Credits

- [MVG](https://mvg.de) for the API.
- Ratatui for the beautiful terminal interface framework.

## License

MIT

### Limitations

Currently, the app only handles ASCII or 1 byte UTF-8 character input. If you are searching a station with non-ascii characters (i.e. ö,ß etc.) in its name, the app will ignore the input. Please type the closest characters & scroll a bit down to select the station from the list (this will be fixed at a later version.)
