"""
Test script for element staleness handling and auto-refresh functionality.

This script demonstrates:
1. Auto-refresh capability when elements become stale
2. Manual refresh() method
3. Disabling auto-refresh
"""

from bromium import WinDriver
import time

def test_auto_refresh():
    """Test automatic refresh when element becomes stale"""
    print("=" * 60)
    print("Test 1: Auto-refresh (enabled by default)")
    print("=" * 60)

    driver = WinDriver(timeout_ms=5)
    print(f"Driver created: {repr(driver)}")
    print(f"Auto-refresh enabled: {driver.get_auto_refresh()}")

    # Get current cursor position
    x, y = driver.get_curser_pos()
    print(f"Cursor position: ({x}, {y})")

    # Get element at cursor
    try:
        element = driver.get_ui_element(x, y)
        print(f"Element found: {element.get_name()}")
        print(f"Element XPath: {element.get_xpath()}")

        # Simulate staleness by waiting (in real scenario, UI might change)
        print("\nWaiting 2 seconds (simulating UI changes)...")
        time.sleep(2)

        # Try to interact with element - should auto-refresh if stale
        print("Attempting to get element name (may trigger auto-refresh)...")
        name = element.get_name()
        print(f"Successfully accessed element: {name}")

    except Exception as e:
        print(f"Error: {e}")

def test_manual_refresh():
    """Test manual refresh() method"""
    print("\n" + "=" * 60)
    print("Test 2: Manual refresh() method")
    print("=" * 60)

    driver = WinDriver(timeout_ms=5)
    print(f"Driver created: {repr(driver)}")

    # Get cursor position and element
    x, y = driver.get_curser_pos()
    element = driver.get_ui_element(x, y)
    print(f"Element found: {element.get_name()}")

    # Manual refresh
    print("\nManually refreshing UI tree...")
    driver.refresh()
    print("Refresh complete!")
    print(f"Driver after refresh: {repr(driver)}")

def test_auto_refresh_disabled():
    """Test behavior with auto-refresh disabled"""
    print("\n" + "=" * 60)
    print("Test 3: Auto-refresh disabled")
    print("=" * 60)

    driver = WinDriver(timeout_ms=5)

    # Disable auto-refresh
    driver.set_auto_refresh(False)
    print(f"Driver created: {repr(driver)}")
    print(f"Auto-refresh enabled: {driver.get_auto_refresh()}")

    # Get cursor position
    x, y = driver.get_curser_pos()
    element = driver.get_ui_element(x, y)
    print(f"Element found: {element.get_name()}")

    print("\nWith auto-refresh disabled, stale elements will raise errors")
    print("You must call driver.refresh() manually when needed")

def test_toggle_auto_refresh():
    """Test toggling auto-refresh setting"""
    print("\n" + "=" * 60)
    print("Test 4: Toggle auto-refresh setting")
    print("=" * 60)

    driver = WinDriver(timeout_ms=5)

    print(f"Initial auto-refresh: {driver.get_auto_refresh()}")

    driver.set_auto_refresh(False)
    print(f"After disabling: {driver.get_auto_refresh()}")

    driver.set_auto_refresh(True)
    print(f"After enabling: {driver.get_auto_refresh()}")

if __name__ == "__main__":
    print("Testing Bromium Staleness Handling and Auto-Refresh")
    print("=" * 60)
    print()

    try:
        test_auto_refresh()
        test_manual_refresh()
        test_auto_refresh_disabled()
        test_toggle_auto_refresh()

        print("\n" + "=" * 60)
        print("All tests completed successfully!")
        print("=" * 60)

    except Exception as e:
        print(f"\nTest failed with error: {e}")
        import traceback
        traceback.print_exc()
