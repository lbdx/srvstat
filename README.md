# srvstat

## Metrics Publisher

A lightweight Rust application that reads system metrics (such as Disk, CPU, and Memory usage) from the host and publishes them to an MQTT queue.

### Features

- Reads metrics from the system.
- Publishes metrics to an MQTT queue.
- Supports disk, CPU, memory, and temperature metrics.

#### Temperature Monitoring

The application can monitor system temperatures provided by available hardware sensors. Each detected temperature component (e.g., CPU package, specific cores, GPU, motherboard sensors, etc.) is published as an individual sensor to MQTT. This allows for granular monitoring in platforms like Home Assistant, where each temperature reading will appear as a distinct entity.

Temperatures are reported in Celsius (°C).

**Important Limitation:** Access to hardware temperature sensors might be restricted on some virtualized environments (like Docker containers or Windows Subsystem for Linux - WSL), or if the necessary kernel modules are not loaded on Linux. In such cases, temperature metrics may not be available, may be limited, or may report unexpected values.

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

