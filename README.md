# Rust Config Server

A **Spring Boot Config Server** equivalent implemented in Rust using Axum, providing centralized configuration management for microservices and distributed applications.

## 🚀 Features

- **🔧 Configuration Management**: Load configs from files and Git repositories
- **🌍 Profile Support**: Environment-specific configurations (dev, prod, staging)
- **🏷️ Label/Branch Support**: Git branch-based configuration versions
- **🔐 Encryption/Decryption**: Secure sensitive configuration values with AES-256-GCM
- **💚 Health Checks**: Separate health endpoint on different port
- **📊 Metrics**: Prometheus metrics on separate port
- **📚 Client Library**: Easy-to-use Rust client library
- **🌐 Environment Integration**: Automatic environment variable population
- **🔄 Refresh Support**: Dynamic configuration refresh without restart
- **📂 Multiple Formats**: YAML, JSON, Properties file support
- **🔀 Git Integration**: Pull configurations from Git repositories

## 📋 Table of Contents

- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [API Reference](#api-reference)
- [Client Usage](#client-usage)
- [Docker Deployment](#docker-deployment)
- [Examples](#examples)
- [Contributing](#contributing)

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Config Files  │    │   Git Repository │    │  Client Apps    │
│   (YAML/JSON)   │◄──►│   (Optional)     │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
          │                       │                       │
          └───────────────────────┼───────────────────────┘
                                  ▼
                    ┌─────────────────────────┐
                    │    Config Server        │
                    │   ┌─────────────────┐   │
                    │   │  Main Port      │   │  ← Configuration API
                    │   │  (8888)         │   │
                    │   ├─────────────────┤   │
                    │   │  Health Port    │   │  ← Health Checks
                    │   │  (8889)         │   │
                    │   ├─────────────────┤   │
                    │   │  Metrics Port   │   │  ← Prometheus Metrics
                    │   │  (8890)         │   │
                    │   └─────────────────┘   │
                    └─────────────────────────┘
```

## 🚀 Quick Start

### 1. Start the Config Server

```bash
# Clone the repository
git clone https://github.com/AarambhDevHub/rust-config-server.git
cd rust-config-server

# Run the server
cd server
cargo run
```

### 2. Create Configuration Files

```yaml
# server/configs/application.yml
server:
  port: 8080
database:
  url: postgresql://localhost:5432/myapp
  username: postgres
  password: "{cipher}encrypted-password"
```

### 3. Use the Client Library

```rust
use config_client::{init_config, get_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from server
    init_config(
        "http://localhost:8888",
        "myapp",
        "dev",
        Some("master")
    ).await?;

    // Access configuration
    let db_url = get_config("database.url").await
        .unwrap_or_default();

    println!("Database URL: {}", db_url);
    Ok(())
}
```

## 📦 Installation

### Prerequisites

- **Rust 1.70+**
- **Git** (for Git repository support)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/AarambhDevHub/rust-config-server.git
cd rust-config-server

# Build all components
cargo build --release

# Run tests
cargo test
```

### Using Cargo

```bash
# Add to your Cargo.toml
[dependencies]
config-client = { git = "https://github.com/AarambhDevHub/rust-config-server.git", package = "config-client" }
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_PORT` | Main server port | `8888` |
| `HEALTH_PORT` | Health check port | `8889` |
| `METRICS_PORT` | Metrics port | `8890` |
| `CONFIG_PATH` | Local config directory | `./configs` |
| `GIT_URI` | Git repository URL | - |
| `GIT_USERNAME` | Git username | - |
| `GIT_PASSWORD` | Git password/token | - |
| `ENCRYPT_KEY` | Encryption key (32 chars) | `default-secret-key-32-characters` |
| `DEFAULT_LABEL` | Default Git branch | `master` |

### Configuration File Structure

```
server/configs/
├── application.yml              # Base configuration
├── application-dev.yml          # Development profile
├── application-prod.yml         # Production profile
├── myapp.yml                   # Application-specific
├── myapp-dev.yml              # App + profile specific
└── myapp-prod.yml             # App + profile specific
```

### Configuration Precedence (Highest to Lowest)

1. `{application}-{profile}.yml`
2. `{application}.yml`
3. `application-{profile}.yml`
4. `application.yml`

## 📖 API Reference

### Get Configuration

```http
GET /{application}/{profile}/{label}
```

**Parameters:**
- `application`: Application name
- `profile`: Environment profile (dev, prod, etc.)
- `label`: Git branch/tag

**Response:**
```json
{
  "name": "myapp",
  "profiles": ["dev"],
  "label": "master",
  "version": "abc123",
  "propertySources": [
    {
      "name": "myapp-dev.yml",
      "source": {
        "database.url": "postgresql://localhost:5432/devdb",
        "debug": true
      }
    }
  ]
}
```

### Encrypt Value

```http
POST /encrypt
Content-Type: application/json

{
  "value": "my-secret-password"
}
```

**Response:**
```json
{
  "encrypted": "{cipher}AQAEncryptedValue..."
}
```

### Decrypt Value

```http
POST /decrypt
Content-Type: application/json

{
  "encrypted": "{cipher}AQAEncryptedValue..."
}
```

### Refresh Configuration

```http
POST /refresh
```

### Health Checks

```http
GET :8889/health        # Overall health
GET :8889/health/live   # Liveness probe
GET :8889/health/ready  # Readiness probe
```

### Metrics

```http
GET :8890/metrics                    # Prometheus format
GET :8890/actuator/prometheus        # Alternative endpoint
```

## 🔧 Client Usage

### Basic Usage

```rust
use config_client::{init_config, get_config, get_config_or};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize configuration
    init_config(
        "http://localhost:8888",
        "myapp",
        "production",
        Some("v1.0.0")
    ).await?;

    // Get configuration values
    let database_url = get_config("database.url").await;
    let max_connections = get_config_or("database.max-connections", "10").await;

    println!("DB URL: {:?}", database_url);
    println!("Max Connections: {}", max_connections);

    Ok(())
}
```

### Advanced Client Usage

```rust
use config_client::{ConfigClientBuilder, get_all_config, print_all_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Direct client usage
    let client = ConfigClientBuilder::new()
        .server_url("http://config-server:8888")
        .application("payment-service")
        .profile("staging")
        .label("release-1.2")
        .build();

    let config = client.fetch_config().await?;

    // Encrypt sensitive data
    let encrypted = client.encrypt_value("api-key-12345").await?;
    println!("Encrypted: {}", encrypted);

    // Print all loaded configuration
    print_all_config().await?;

    Ok(())
}
```

### Configuration Types

```rust
// Get as specific types
let port: i32 = get_config("server.port")
    .await
    .and_then(|v| v.parse().ok())
    .unwrap_or(8080);

let debug_enabled: bool = get_config("debug")
    .await
    .and_then(|v| v.parse().ok())
    .unwrap_or(false);
```

## 🐳 Docker Deployment

### Docker Compose

```yaml
version: '3.8'

services:
  config-server:
    build: ./server
    ports:
      - "8888:8888"  # Main API
      - "8889:8889"  # Health
      - "8890:8890"  # Metrics
    environment:
      - CONFIG_PATH=/configs
      - GIT_URI=https://github.com/your-org/config-repo.git
      - GIT_USERNAME=your-username
      - GIT_PASSWORD=your-token
    volumes:
      - ./configs:/configs
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8889/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  app:
    build: ./app
    depends_on:
      - config-server
    environment:
      - CONFIG_SERVER_URL=http://config-server:8888
```

### Dockerfile

```dockerfile
# Server Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin config-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/config-server /usr/local/bin/
EXPOSE 8888 8889 8890
CMD ["config-server"]
```

## 💡 Examples

### Example Configuration Files

**application.yml:**
```yaml
server:
  port: 8080
  shutdown-timeout: 30s

database:
  url: postgresql://localhost:5432/defaultdb
  username: postgres
  password: "{cipher}encrypted-password"
  pool:
    max-connections: 10
    min-connections: 2

logging:
  level:
    root: INFO
    app: DEBUG

features:
  new-ui: false
  analytics: true
```

**myapp-dev.yml:**
```yaml
database:
  url: postgresql://localhost:5432/devdb
  username: devuser
  password: devpass

logging:
  level:
    root: DEBUG

features:
  new-ui: true
  debug-mode: true
```

### Git Repository Structure

```
config-repo/
├── application.yml
├── application-dev.yml
├── application-prod.yml
├── payment-service.yml
├── payment-service-prod.yml
├── user-service.yml
└── user-service-dev.yml
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: config-server
spec:
  replicas: 2
  selector:
    matchLabels:
      app: config-server
  template:
    metadata:
      labels:
        app: config-server
    spec:
      containers:
      - name: config-server
        image: your-registry/config-server:latest
        ports:
        - containerPort: 8888
        - containerPort: 8889
        - containerPort: 8890
        env:
        - name: GIT_URI
          value: "https://github.com/your-org/config-repo.git"
        - name: GIT_USERNAME
          valueFrom:
            secretKeyRef:
              name: git-credentials
              key: username
        - name: GIT_PASSWORD
          valueFrom:
            secretKeyRef:
              name: git-credentials
              key: token
---
apiVersion: v1
kind: Service
metadata:
  name: config-server-service
spec:
  selector:
    app: config-server
  ports:
  - name: api
    port: 8888
    targetPort: 8888
  - name: health
    port: 8889
    targetPort: 8889
  - name: metrics
    port: 8890
    targetPort: 8890
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration

# Test with coverage
cargo tarpaulin --out Html

# Load test
curl -X GET http://localhost:8888/myapp/dev/master
curl -X POST http://localhost:8888/encrypt \
  -H "Content-Type: application/json" \
  -d '{"value":"test-secret"}'
```

## 📊 Monitoring

### Prometheus Metrics

- `config_requests_total` - Total configuration requests
- `config_requests_failed_total` - Failed configuration requests
- `config_request_duration_seconds` - Request duration histogram

### Health Endpoints

- `/health` - Overall application health
- `/health/live` - Kubernetes liveness probe
- `/health/ready` - Kubernetes readiness probe

### Grafana Dashboard

Import the provided Grafana dashboard from `monitoring/grafana-dashboard.json` for visualization.

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit changes**: `git commit -m 'Add amazing feature'`
4. **Push to branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Development Setup

```bash
# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Run with hot reload
cargo watch -x run

# Run tests on file changes
cargo watch -x test
```

### Code Standards

- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`
- **Security audit**: `cargo audit`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ☕ Support & Community

If you find Ignitia helpful, consider supporting the project:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-ffdd00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/aarambhdevhub)


## 🙏 Acknowledgments

- **Spring Cloud Config** - Inspiration for the API design
- **Axum** - Web framework
- **Tokio** - Async runtime
- **Rust Community** - Amazing ecosystem

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/AarambhDevHub/rust-config-server/issues)
- **Discussions**: [GitHub Discussions](https://github.com/AarambhDevHub/rust-config-server/discussions)
- **Documentation**: [Wiki](https://github.com/AarambhDevHub/rust-config-server/wiki)
