#!/usr/bin/env python3
"""
Example demonstrating Bromium's logging functionality.

This example shows how the logging system works in Bromium, particularly
for XPath generation and element finding operations.
"""

import os
import time
from bromium import WinDriver

def demonstrate_logging():
    """Demonstrate various Bromium operations that generate logs."""
    
    print("Bromium Logging Example")
    print("=" * 40)
    
    # Note: Logging is automatically initialized when bromium module is imported
    # Log files are written to:
    # - Windows: %APPDATA%\Bromium\logs\
    # - Other platforms: ~/.local/share/bromium/logs/
    
    print("Creating WinDriver (this will be logged)...")
    driver = WinDriver(timeout_ms=5000)
    
    # Get screen context
    print("\nGetting screen context...")
    screen_context = driver.get_screen_context()
    print(f"Screen: {screen_context.get_screen_width()}x{screen_context.get_screen_height()}, "
          f"Scale: {screen_context.get_screen_scale()}")
    
    # Get current cursor position
    print("\nGetting cursor position...")
    x, y = driver.get_curser_pos()
    print(f"Cursor position: ({x}, {y})")
    
    # Get UI element at cursor position (this will generate XPath logs)
    print(f"\nGetting UI element at cursor position ({x}, {y})...")
    print("Check logs for detailed XPath generation process...")
    try:
        element = driver.get_ui_element(x, y)
        print(f"Found element: {element.get_name()}")
        print(f"Element XPath: {element.get_xpath()}")
        print(f"Element handle: {element.get_handle()}")
        print(f"Runtime ID: {element.get_runtime_id()}")
    except Exception as e:
        print(f"Error getting UI element: {e}")
    
    # Demonstrate XPath-based element finding
    print("\nTesting XPath-based element finding...")
    
    # Example XPath for Windows Start button (may not work on all systems)
    start_button_xpath = r'/Pane[@ClassName="#32769"][@Name="Desktop 1"]/Pane[@ClassName="Shell_TrayWnd"][@Name="Taskbar"]/Pane[@ClassName="Windows.UI.Input.InputSite.WindowClass"]/Pane[@ClassName="Taskbar.TaskbarFrameAutomationPeer"][@AutomationId="TaskbarFrame"]/Button[@Name="Start"][@AutomationId="StartButton"]'
    
    try:
        print("Searching for Start button...")
        print("Check logs for detailed element search process...")
        start_element = driver.get_ui_element_by_xpath(start_button_xpath)
        print(f"Found Start button: {start_element.get_name()}")
        
        # Demonstrate clicking (uncomment to actually click)
        # print("Clicking Start button...")
        # start_element.send_click()
        
    except Exception as e:
        print(f"Could not find Start button (this is normal if XPath doesn't match your system): {e}")
    
    # Test application launch (optional)
    print("\nTesting application launch...")
    try:
        calc_path = r"C:\Windows\System32\calc.exe"
        calc_xpath = r'/Window[@ClassName="ApplicationFrameWindow"][@Name="Calculator"]'
        
        if os.path.exists(calc_path):
            print(f"Launching Calculator...")
            success = driver.launch_or_activate_app(calc_path, calc_xpath)
            print(f"Launch result: {'Success' if success else 'Failed'}")
            
            if success:
                time.sleep(2)  # Wait for app to open
                
                # Try to get an element from the calculator
                print("Getting calculator window element...")
                calc_x, calc_y = driver.get_curser_pos()
                calc_element = driver.get_ui_element(calc_x, calc_y)
                print(f"Calculator element: {calc_element.get_name()}")
        else:
            print("Calculator not found at expected path")
            
    except Exception as e:
        print(f"Error with application launch test: {e}")

def show_log_file_location():
    """Show where the log files are stored."""
    print("\nLog File Information")
    print("=" * 20)
    
    if os.name == 'nt':  # Windows
        appdata = os.environ.get('APPDATA', '')
        if appdata:
            log_dir = os.path.join(appdata, 'Bromium', 'logs')
            print(f"Log directory: {log_dir}")
            
            if os.path.exists(log_dir):
                log_files = [f for f in os.listdir(log_dir) if f.endswith('.log')]
                if log_files:
                    print(f"Found {len(log_files)} log file(s):")
                    for log_file in sorted(log_files, reverse=True)[:3]:  # Show latest 3
                        log_path = os.path.join(log_dir, log_file)
                        size = os.path.getsize(log_path)
                        print(f"  - {log_file} ({size} bytes)")
                    
                    latest_log = os.path.join(log_dir, log_files[-1])
                    print(f"\nLatest log file: {latest_log}")
                    print("Tail of latest log file:")
                    print("-" * 40)
                    
                    try:
                        with open(latest_log, 'r', encoding='utf-8') as f:
                            lines = f.readlines()
                            for line in lines[-10:]:  # Show last 10 lines
                                print(line.rstrip())
                    except Exception as e:
                        print(f"Could not read log file: {e}")
                else:
                    print("No log files found yet")
            else:
                print("Log directory does not exist yet")
        else:
            print("Could not determine APPDATA directory")
    else:
        home = os.path.expanduser("~")
        log_dir = os.path.join(home, ".local", "share", "bromium", "logs")
        print(f"Log directory: {log_dir}")

if __name__ == "__main__":
    print("Starting Bromium logging demonstration...")
    print("This will create detailed logs of all operations.")
    print()
    
    demonstrate_logging()
    
    print()
    show_log_file_location()
    
    print("\nLogging demonstration completed!")
    print("Check the log files for detailed information about all operations.")