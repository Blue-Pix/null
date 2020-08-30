# null

## Usage

```bash
# to get tweet
$ cargo run
```

## Setup

Prepare `.env` file below.

```
TW_NAME = twitter_user_screen_name_here
TW_TOKEN = twitter_api_token_here
```

Or you can pass those as environment variables when running code.

## Test

Run tests sequentially, because of environment variables manipulation.

```
cargo test -- --test-threads=1
```