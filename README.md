# pbp-steam-interface
Server application for routing steam api calls back to the end user.

## Building
- have rustup stable installed
- run `cargo build --release`, the dependencies will be fetched automatically

## How it works
The server listens for connections on a specific port, and expects an http request containing only the Steam ID of a user.
It then uses the Steam API to fetch the installed games of the user, returning them with a network response.

## Steam API key
Make sure to have your Steam API key in a '.env' file, which should reside in the same directory as the executable.
It will be read in this format: `STEAM_API_KEY=YOUR_STEAM_API_KEY_HERE`
