#!/bin/sh

# ! this works only if you use 1password cli
# ! if not you need to export the DISCORD_TOKEN 
# ! as Environment Variable each time manually

# * this use the cargo run, this is only for debugging 
# * and development, if you want to use the release
# * build it via 'cargo build --release' and run it 
# * 'target/release/basic_rust_discord_bot'

# TODO: Get also Colors in the 1Password Run 

op run --env-file=".env" -- cargo run
# op run --env-file=".env" -- ./target/release/basic_rust_discord_bot