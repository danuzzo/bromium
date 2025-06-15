# from time import sleep
from bromium import WinDriver

# Create a WinDriver instance with a timeout value
driver = WinDriver(5)

# Get screen context information
screen_context = driver.get_screen_context()
print(f"Screen width: {screen_context.get_screen_width()}")
print(f"Screen height: {screen_context.get_screen_height()}")
print(f"Screen scale: {screen_context.get_screen_scale()}")

# Get current cursor position
x, y = driver.get_curser_pos()
print(f"Current cursor position: ({x}, {y})")

# Get UI element at specific coordinates
element = driver.get_ui_element(x, y)
print(f"Returned element: {repr(element)}")
print(f"UI Element name: {element.get_name()}")
print(f"UI Element xpath: {element.get_xpath()}")
print(f"UI Element handle: {element.get_handle()}")

xpath = element.get_xpath()

# Get UI element by XPath
element_by_xpath = driver.get_ui_element_by_xpath(xpath)
print(f"XPath based UI Element name: {element_by_xpath.get_name()}")
print(f"Returned xpath based element: {repr(element_by_xpath)}")

# Click the UI element
print(f"Clicking UI Element {element_by_xpath.get_name()} now...")
element_by_xpath.send_click()
print("Click action completed.")