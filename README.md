# Rust Google Cloud Pub/Sub Project

A small Rust project demonstrating Google Cloud Pub/Sub integration for publishing and subscribing to messages.

## Features

- **Publisher**: Send messages to Google Cloud Pub/Sub topics
- **Subscriber**: Receive and process messages from Pub/Sub subscriptions
- **Async Support**: Built with Tokio for async/await concurrency
- **Error Handling**: Comprehensive error handling with custom error types
- **Logging**: Structured logging for debugging and monitoring

## Prerequisites

- Rust 1.70+ (Install from https://rustup.rs/)
- Google Cloud Project with Pub/Sub API enabled
- Google Cloud credentials (service account JSON key file)

## Setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Set up Google Cloud credentials

```bash
# Download your service account key from Google Cloud Console
# Then set the environment variable
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account-key.json
export GCP_PROJECT_ID=your-project-id
```

### 3. Create a Pub/Sub Topic and Subscription

```bash
# Using gcloud CLI
gcloud pubsub topics create my-topic
gcloud pubsub subscriptions create my-subscription --topic=my-topic
```

## Building the Project

```bash
cd /home/runner/work/pubsub/pubsub
cargo build --release
```

## Running

### Help

```bash
cargo run -- --help
```

Or after building:

```bash
./target/release/rust-pubsub
```

### Publishing Messages

```bash
# Publish to a topic
cargo run -- publish my-project my-topic

# Or after building:
./target/release/rust-pubsub publish my-project my-topic
```

### Subscribing to Messages

```bash
# Subscribe to messages
cargo run -- subscribe my-project my-subscription

# Or after building:
./target/release/rust-pubsub subscribe my-project my-subscription
```

## Project Structure

```
src/
├── main.rs       - Entry point with CLI argument parsing
├── publisher.rs  - Message publishing functionality
├── subscriber.rs - Message subscription and processing
├── config.rs     - Configuration management
└── error.rs      - Custom error types
```

## Environment Variables

- `GCP_PROJECT_ID` or `GOOGLE_CLOUD_PROJECT` - Your Google Cloud Project ID (required)
- `GOOGLE_APPLICATION_CREDENTIALS` - Path to service account JSON key file (required for authentication)
- `RUST_LOG` - Set log level (e.g., `RUST_LOG=info`)

## Example Usage

```bash
# Set up environment
export GCP_PROJECT_ID="my-gcp-project"
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/key.json"
export RUST_LOG=info

# Publish messages
cargo run -- publish my-gcp-project my-topic

# Subscribe to messages in another terminal
cargo run -- subscribe my-gcp-project my-subscription
```

## Dependencies

- **tokio** - Async runtime for Rust
- **google-pubsub1** - Google Cloud Pub/Sub API
- **serde_json** - JSON serialization
- **uuid** - UUID generation for message IDs
- **log** & **env_logger** - Logging
- **chrono** - Timestamp handling
- **thiserror** - Error handling

## Implementation Notes

This project includes skeleton implementations for publisher and subscriber functionality with detailed comments showing where the actual Google Cloud Pub/Sub API calls would be integrated. The current implementation includes:

1. **Simulated Publishing** - Creates and displays sample messages
2. **Simulated Subscription** - Simulates receiving messages for demonstration
3. **Error Handling** - Comprehensive error types and handling
4. **Configuration** - Environment-based configuration management

To fully integrate with Google Cloud Pub/Sub:

1. Uncomment and implement the actual API calls in `publisher.rs` and `subscriber.rs`
2. Add proper authentication using the `yup-oauth2` crate
3. Implement message serialization/deserialization as needed
4. Add retry logic and connection pooling

## License

MIT