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
    ```

### Configuration

The application is configured via environment variables:

- **`BROKER_URL`** (Required): The URL of the MQTT broker.
  - Example: `export BROKER_URL=tcp://localhost:1883`

- **`SRVSTAT_FAN_SPEED_COMMAND`** (Optional): A command to execute for reading fan speed in RPM.
  - If set, the application will execute this command, and the standard output is expected to be a numerical RPM value.
  - Example: `export SRVSTAT_FAN_SPEED_COMMAND="cat /sys/class/hwmon/hwmon0/device/fan1_input"`

- **`SRVSTAT_FAN_SPEED_FILE`** (Optional): Path to a file whose content is the fan speed in RPM.
  - This variable is used if `SRVSTAT_FAN_SPEED_COMMAND` is not set. The application will read the content of this file, expecting it to be a numerical RPM value.
  - Example: `export SRVSTAT_FAN_SPEED_FILE="/tmp/fan_speed_rpm.txt"`

If neither `SRVSTAT_FAN_SPEED_COMMAND` nor `SRVSTAT_FAN_SPEED_FILE` is set, fan speed metrics will not be actively reported (will default to 0 RPM).

