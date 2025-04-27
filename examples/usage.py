from bromium import Windriver

driver = Windriver(10)

# Get the screen size and scale
screen_context = driver.get_screen_context()
print(f"Screen Height: {screen_context.get_screen_height()},  Screen Width: {screen_context.get_screen_width()}, Screen Scale: {screen_context.get_screen_scale()}")

# Get the current mouse position and the UI element at that position
(x, y) = driver.get_curser_pos()
element = driver.get_ui_element(x, y)
print(f"Element at ({x}, {y}): {element}")


