# Contributing to Rust Google Cloud Pub/Sub Project

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Google Cloud SDK (gcloud CLI)

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/raveendra11/pubsub.git
cd pubsub
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your Google Cloud Project ID and credentials path
```

3. Build the project:
```bash
cargo build
```

4. Run tests:
```bash
cargo test
```

## Code Structure

- `src/main.rs` - Application entry point and CLI argument handling
- `src/publisher.rs` - Publishing functionality
- `src/subscriber.rs` - Subscribing functionality
- `src/config.rs` - Configuration management
- `src/error.rs` - Error types and handling

## Making Changes

1. Create a new branch for your feature:
```bash
git checkout -b feature/your-feature-name
```

2. Make your changes and ensure the code compiles:
```bash
cargo check
```

3. Run linting:
```bash
cargo clippy
```

4. Format your code:
```bash
cargo fmt
```

5. Build and test:
```bash
cargo build --release
cargo test
```

6. Commit your changes:
```bash
git commit -m "Description of your changes"
```

7. Push to your fork and create a pull request.

## Integration with Google Cloud Pub/Sub

To fully integrate with Google Cloud Pub/Sub, the following dependencies should be added:

- `google-pubsub-api` for the Pub/Sub API
- `yup-oauth2` for OAuth2 authentication
- `hyper` for HTTP client support

See the commented sections in `publisher.rs` and `subscriber.rs` for implementation guidance.

## Testing

Run tests with:
```bash
cargo test
```

For integration tests with Google Cloud Pub/Sub, ensure your service account has appropriate permissions:
- `pubsub.topics.create`
- `pubsub.topics.publish`
- `pubsub.subscriptions.consume`

## Debugging

Enable debug logging:
```bash
RUST_LOG=debug cargo run -- [command] [args]
```

## Performance

For performance optimization and production use:
- Build with release profile: `cargo build --release`
- Consider connection pooling for multiple publishers/subscribers
- Implement message batching for bulk publishing
