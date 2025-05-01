from bromium import WinDriver

# Create a WinDriver instance with a timeout value
driver = WinDriver(timeout=5)

# Get current cursor position
x, y = driver.get_curser_pos()
print(f"Current cursor position: ({x}, {y})")

# Get UI element at specific coordinates
element = driver.get_ui_element(x, y)
print(f"UI Element name: {element.get_name()}")
print(f"UI Element xpath: {element.get_xpath()}")

# Get screen context information
screen_context = driver.get_screen_context()
print(f"Screen width: {screen_context.get_screen_width()}")
print(f"Screen height: {screen_context.get_screen_height()}")
print(f"Screen scale: {screen_context.get_screen_scale()}")