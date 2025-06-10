# Bromium

Bromium is a Python library that provides bindings to interact with the Windows UI Automation API through Rust. It enables users to automate tasks and interact with Windows UI elements programmatically.

## Features

- Get cursor position coordinates
- Retrieve UI element information at specific coordinates
- Get screen context information (size and scaling)
- Cross-platform development using Rust with Python bindings

<!-- ## Installation

```bash.\
pip install bromium
``` -->

## Usage

Here's a basic example of how to use Bromium:

```python
from bromium import WinDriver

# Create a WinDriver instance with a timeout value
driver = WinDriver(timeout=5)

# Get current cursor position
x, y = driver.get_curser_pos()
print(f"Cursor position: ({x}, {y})")

# Get UI element at specific coordinates
element = driver.get_ui_element(x, y)
print(f"UI Element name: {element.get_name()}")

# Get screen context information
screen_context = driver.get_screen_context()
print(f"Screen width: {screen_context.get_screen_width()}")
print(f"Screen height: {screen_context.get_screen_height()}")
print(f"Screen scale: {screen_context.get_screen_scale()}")

# Launch or activate an application
app_path = r"C:\Windows\System32\calc.exe"
xpath = r'/Window[@ClassName="ApplicationFrameWindow"][@Name="Calculator"]'
success = driver.launch_or_activate_app(app_path, xpath)
if success:
    print("Calculator is now in focus")
```

## API Reference

### WinDriver

The main class for interacting with Windows UI elements.

#### Methods

- `get_curser_pos() -> tuple[int, int]`: Get current cursor coordinates
- `get_ui_element(x: int, y: int) -> Element`: Get UI element at specified coordinates
- `get_ui_element_by_xpath(xpath: str) -> Element`: Get UI element from an xpath
- `get_screen_context() -> ScreenContext`: Get screen size and scaling information
- `launch_or_activate_app() -> bool`: Launches a new application or activates an existing window

### Element

Represents a Windows UI Automation element.

#### Methods

- `get_name() -> str`: Get the name of the UI element
- `get_xpath() -> str`: Get the xpath of the UI element
- `get_handle() -> int`: Get the window handle of the UI element    
- `send_click()`: Send a left mouse click to the UI element

### ScreenContext

Contains information about the screen configuration.

#### Methods

- `get_screen_width() -> int`: Get screen width in pixels
- `get_screen_height() -> int`: Get screen height in pixels
- `get_screen_scale() -> float`: Get screen scaling factor (as a decimal, e.g., 1.25 for 125%)

## Requirements

- Python 3.8 or higher
- Windows operating system

## Building from Source

To build the project from source, you'll need:

1. Rust toolchain (cargo, rustc)
2. Python 3.8+
3. maturin (for building Python wheels)

```bash
# Clone the repository
git clone https://github.com/yourusername/bromium.git
cd bromium

# Build the project using maturin
maturin build

# Install in development mode
maturin develop
```

## Logging

Bromium includes comprehensive logging functionality that helps debug issues and understand the internal workings of XPath generation and element finding operations.

### Automatic Initialization

Logging is automatically initialized when the bromium module is imported. No additional setup is required.

### Log File Location

Log files are automatically created and stored in the following locations:

- **Windows**: `%APPDATA%\Bromium\logs\`
- **Other platforms**: `~/.local/share/bromium/logs/`

### Log File Management

- Log files are named with timestamps: `bromium_YYYYMMDD_HHMMSS.log`
- Old log files are automatically cleaned up, keeping only the last 10 files
- Each log file contains detailed timestamped entries with module and line information

### Log Levels and Content

Bromium logs at different levels to provide comprehensive debugging information:

#### INFO Level
- Application lifecycle events (driver creation, module loading)
- High-level operation results (element found, XPath generated)
- Performance summaries

#### DEBUG Level  
- Detailed operation progress
- Step-by-step XPath generation process
- Element search strategies and results
- UI automation API interactions

#### TRACE Level
- Lowest-level details
- Individual attribute parsing
- Raw data from Windows APIs
- Internal state changes

### Key Logged Operations

#### XPath Generation (`[XPATH_*]` tags)
- Coordinate-to-XPath conversion process
- Raw element data from Windows UI Automation
- XPath formatting and simplification steps
- Performance timing for generation operations

Example log entries:
```
[2024-06-09 14:30:15.123] [DEBUG] [XPATH_GENERATE] coordinates=(100, 200) - Starting XPath generation
[2024-06-09 14:30:15.145] [DEBUG] [XPATH_FORMAT] input_length=1245 - Matching original C++ XPath format
[2024-06-09 14:30:15.167] [INFO] [XPATH_GENERATE] coordinates=(100, 200) - XPath generation successful
```

#### Element Finding (`[UIAUTO_*]` tags)
- XPath parsing into searchable elements
- UI element traversal and matching
- Multiple candidate handling and validation
- Runtime ID-based element lookup

Example log entries:
```
[2024-06-09 14:30:16.200] [INFO] [UIAUTO_FIND_BY_XPATH] xpath_length=245 - Starting element search by XPath
[2024-06-09 14:30:16.225] [DEBUG] [UIAUTO_TRAVERSE] element=1/3 - Searching for element: control_type=Button
[2024-06-09 14:30:16.250] [INFO] [UIAUTO_FIND_BY_XPATH] success - Successfully found element: name='OK'
```

#### Performance Monitoring (`[PERF]` tags)
- Operation timing for performance analysis
- Identifies bottlenecks in XPath generation and element finding

Example log entries:
```
[2024-06-09 14:30:15.123] [DEBUG] [PERF] Starting operation: generate_xpath
[2024-06-09 14:30:15.167] [DEBUG] [PERF] Completed operation 'generate_xpath' in 44.5ms
```

### Configuring Log Output

While the file logging is automatic, you can control console output using environment variables:

```bash
# Set log level for console output
export RUST_LOG=debug

# Run your Python script
python your_script.py
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

### Troubleshooting with Logs

When experiencing issues:

1. **XPath Generation Problems**: Look for `[XPATH_*]` entries to see the raw element data and formatting steps
2. **Element Not Found**: Check `[UIAUTO_*]` entries to understand the search process and why elements weren't matched
3. **Performance Issues**: Review `[PERF]` entries to identify slow operations
4. **Application Launch Issues**: Look for application control logs in the launch_or_activate_app operations

### Example Usage

```python
from bromium import WinDriver

# Logging starts automatically
driver = WinDriver(timeout=5)

# These operations will be logged in detail:
x, y = driver.get_curser_pos()           # Cursor position logging
element = driver.get_ui_element(x, y)    # XPath generation + element creation
found = driver.get_ui_element_by_xpath(xpath)  # XPath parsing + element search
```

### Log File Example

```
[2024-06-09 14:30:15.123] [INFO] [src/lib.rs:25] Bromium library initialized successfully
[2024-06-09 14:30:15.124] [INFO] [src/windriver.rs:45] Creating new WinDriver with timeout: 5000ms
[2024-06-09 14:30:15.125] [DEBUG] [src/windriver.rs:78] Getting current cursor position
[2024-06-09 14:30:15.126] [DEBUG] [src/windriver.rs:85] Cursor position: (1024, 768)
[2024-06-09 14:30:15.127] [INFO] [src/xpath.rs:234] [XPATH_GENERATE] coordinates=(1024, 768) - Starting XPath generation
[2024-06-09 14:30:15.128] [DEBUG] [src/xpath.rs:240] Setting process DPI awareness
[2024-06-09 14:30:15.170] [INFO] [src/xpath.rs:267] [XPATH_GENERATE] coordinates=(1024, 768) - XPath generation successful
```

## License

Apache License 2.0

<!-- ## Contributing

[Add contribution guidelines here] -->