# lecsicon-db
Convert the lecsicon csv into a SQLite file for faster, immediate access to the data. A CLI prompt is also made available for browsing the data.

## To use
Assuming Rust and Cargo are installed, clone the repo and set up Diesel according to this tutorial https://diesel.rs/guides/getting-started.html but set it up for SQLite. Make sure to set the DATABASE_URL in the .env file and run ```diesel setup```. Ensure the Hunspell .aff and .dic files for Welsh are present at /usr/share/hunspell/cy_GB.aff and /usr/share/hunspell/cy_GB.dic for spelling correction suggestions.

See tiwtor for a demonstration of the leciscon-db in action.
