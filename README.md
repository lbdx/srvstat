# srvstat

## Metrics Publisher

A lightweight Rust application that reads system metrics (such as Disk, CPU, and Memory usage) from the host and publishes them to an MQTT queue.

### Features

- Reads metrics from the system.
- Publishes metrics to an MQTT queue.
- Supports disk, CPU, and memory metrics.

### Project Structure

- `config`: Contains configuration settings for the application.
- `domain`: Defines core domain logic, such as metric categories and services.
- `outbound`: Handles outbound operations, including reading and writing metrics.

### Usage

1. Clone the repository:
    ```bash
    git clone <repository-url>
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Set environment variables for configuration (such as BROKER_URL):
    ```bash
    export BROKER_URL=tcp://localhost:1883

