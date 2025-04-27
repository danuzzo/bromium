# Bromium

Bromium is a Python library that provides bindings to interact with the Windows UI Automation API through Rust. It enables users to automate tasks and interact with Windows UI elements programmatically.

## Features

- Get cursor position coordinates
- Retrieve UI element information at specific coordinates
- Get screen context information (size and scaling)
- Cross-platform development using Rust with Python bindings

## Installation

```bash
pip install bromium
```

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
```

## API Reference

### WinDriver

The main class for interacting with Windows UI elements.

#### Methods

- `__init__(timeout: int)`: Initialize the WinDriver with a timeout value in seconds
- `get_curser_pos() -> tuple[int, int]`: Get current cursor coordinates
- `get_ui_element(x: int, y: int) -> Element`: Get UI element at specified coordinates
- `get_screen_context() -> ScreenContext`: Get screen size and scaling information

### Element

Represents a Windows UI Automation element.

#### Methods

- `get_name() -> str`: Get the name of the UI element

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

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]