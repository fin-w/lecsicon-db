# lecsicon-db
Convert the [lecsicon](https://github.com/techiaith/lecsicon-cymraeg-bangor) CSV into a SQLite database for faster, immediate access to the data. A CLI prompt is also made available for browsing the data.

## To use
Assuming Rust and Cargo are installed, clone the repo and make sure to set the DATABASE_URL in the .env file to the desired location of the database that will be created. Set up Diesel:
```
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
```
Ensure the Hunspell .aff and .dic files for Welsh are present at /usr/share/hunspell/cy_GB.aff and /usr/share/hunspell/cy_GB.dic for spelling correction suggestions (files are available [here](https://github.com/fin-w/LibreOffice-Geiriadur-Cymraeg-Welsh-Dictionary/tree/main/dictionaries)).

Use [tiwtor](https://github.com/fin-w/tiwtor) for a demonstration of leciscon-db in action.
