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

# sleep(5)  # Wait for 5 seconds

# Get UI element at specific coordinates
element = driver.get_ui_element(x, y)
print(repr(element))
# print(f"UI Element name: {element.get_name()}")
# print(f"UI Element xpath: {element.get_xpath()}")
# print(f"UI Element handle: {element.get_handle()}")

# x, y = (136, 645)  # coordinates of UiTreeWalk.cpp in explorer window
# element = driver.get_ui_element(x, y)
# print(repr(element))



xpath = element.get_xpath()
# xpath = r"/Pane[@ClassName=\"#32769\"][@Name=\"Desktop 1\"]/Pane[@ClassName=\"Chrome_WidgetWin_1\"][@Name=\"UiTreeWalk.cpp - bromium - Visual Studio Code\"]/Document[@Name=\"UiTreeWalk.cpp - bromium - Visual Studio Code\"][@AutomationId=\"482160\"]/Group/Window/Group/Group/Group/Group/Group/Group/Group/Group[@AutomationId=\"workbench.view.explorer\"]/Group/Group/Group/Tree[@Name=\"Files Explorer\"]/Group/TreeItem[@Name=\"windriver.rs\"][@AutomationId=\"list_id_5_22\"]/Group/Group[@Name=\"C:\\LocalData\\Rust\\bromium\\src\\windriver.rs • Modified\"]/Group/Text[@Name=\"windriver.rs\"]"



# Get UI element by XPath
element_by_xpath = driver.get_ui_element_by_xpath(xpath)
print(f"XPath based UI Element name: {element_by_xpath.get_name()}")
print(repr(element_by_xpath))

# Click the UI element
print(f"Clicking UI Element {element_by_xpath.get_name()} now...")
element_by_xpath.send_click()
