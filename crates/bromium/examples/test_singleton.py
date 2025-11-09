"""
Test script for WinDriver singleton pattern (Bug #6 fix)

This demonstrates that:
1. Only one WinDriver instance can exist at a time
2. Attempting to create a second instance raises an error
3. close() allows creating a new instance
4. The singleton pattern prevents state desynchronization
"""

from bromium import WinDriver
import time

def test_singleton_enforcement():
    """
    Test that only one WinDriver instance can exist
    """
    print("=" * 70)
    print("Test 1: Singleton Enforcement")
    print("=" * 70)

    # Create first driver
    print("Creating first WinDriver instance...")
    driver1 = WinDriver(timeout_ms=5000)
    print(f"✓ Driver 1 created: {repr(driver1)}")

    # Try to create second driver - should fail
    print("\nAttempting to create second WinDriver instance...")
    try:
        driver2 = WinDriver(timeout_ms=5000)
        print("✗ ERROR: Second driver should not have been created!")
        return False
    except RuntimeError as e:
        print(f"✓ Correctly prevented second instance: {e}")

    print("\n✅ Singleton enforcement test PASSED")
    return True

def test_close_and_recreate():
    """
    Test that close() allows creating a new instance
    """
    print("\n" + "=" * 70)
    print("Test 2: Close and Recreate")
    print("=" * 70)

    # Create first driver
    print("Creating first WinDriver instance...")
    driver1 = WinDriver(timeout_ms=5000)
    print(f"✓ Driver 1 created: {repr(driver1)}")

    # Get an element to verify driver works
    x, y = driver1.get_curser_pos()
    print(f"✓ Cursor position: ({x}, {y})")

    # Close the first driver
    print("\nClosing first driver...")
    driver1.close()
    print("✓ Driver 1 closed")

    # Now we should be able to create a new driver
    print("\nCreating second WinDriver instance after close...")
    try:
        driver2 = WinDriver(timeout_ms=5000)
        print(f"✓ Driver 2 created successfully: {repr(driver2)}")
    except RuntimeError as e:
        print(f"✗ ERROR: Should be able to create driver after close: {e}")
        return False

    # Verify the new driver works
    x2, y2 = driver2.get_curser_pos()
    print(f"✓ New driver working, cursor at: ({x2}, {y2})")

    # Clean up
    driver2.close()
    print("✓ Driver 2 closed")

    print("\n✅ Close and recreate test PASSED")
    return True

def test_singleton_prevents_state_issues():
    """
    Test that singleton pattern prevents the state desynchronization
    that was the original Bug #6
    """
    print("\n" + "=" * 70)
    print("Test 3: Singleton Prevents State Desynchronization")
    print("=" * 70)

    print("This test verifies that Bug #6 (state desynchronization)")
    print("cannot occur because we prevent multiple drivers.\n")

    # Create driver
    driver = WinDriver(timeout_ms=5000)
    print(f"✓ Driver created: {repr(driver)}")

    # Get an element
    x, y = driver.get_curser_pos()
    element1 = driver.get_ui_element(x, y)
    print(f"✓ Element 1: {element1.get_name()}")

    # Try to create second driver - will fail
    print("\nAttempting to create second driver (which would cause Bug #6)...")
    try:
        driver2 = WinDriver(timeout_ms=5000)
        print("✗ ERROR: Should not allow second driver!")
        return False
    except RuntimeError as e:
        print(f"✓ Second driver prevented: Bug #6 cannot occur")

    # The original element still works with the only driver
    print("\nVerifying element still works with original driver...")
    try:
        name = element1.get_name()
        print(f"✓ Element still accessible: {name}")
    except Exception as e:
        print(f"✗ ERROR: Element should still work: {e}")
        return False

    # Clean up
    driver.close()

    print("\n✅ State desynchronization prevention test PASSED")
    return True

def test_proper_usage_pattern():
    """
    Demonstrate the proper usage pattern
    """
    print("\n" + "=" * 70)
    print("Test 4: Proper Usage Pattern")
    print("=" * 70)

    print("Demonstrating recommended usage:\n")

    # Create one driver
    print("1. Create one WinDriver for your script")
    driver = WinDriver(timeout_ms=5000)
    print(f"   ✓ {repr(driver)}\n")

    # Use it for multiple tasks
    print("2. Use it for multiple automation tasks")
    x, y = driver.get_curser_pos()
    elem1 = driver.get_ui_element(x, y)
    print(f"   ✓ Task 1: Found element '{elem1.get_name()}'")

    # Refresh when needed
    print("\n3. Refresh when UI changes significantly")
    driver.refresh()
    elem2 = driver.get_ui_element(x, y)
    print(f"   ✓ After refresh: Found element '{elem2.get_name()}'")

    # Use auto-refresh for staleness
    print("\n4. Let auto-refresh handle staleness automatically")
    print(f"   ✓ Auto-refresh enabled: {driver.get_auto_refresh()}")

    # Close when done
    print("\n5. Close when completely done")
    driver.close()
    print("   ✓ Driver closed\n")

    print("✅ Proper usage pattern demonstrated")
    return True

def test_context_manager_pattern():
    """
    Show that manual close is needed (no __enter__/__exit__ yet)
    """
    print("\n" + "=" * 70)
    print("Test 5: Resource Management")
    print("=" * 70)

    print("Note: WinDriver does not yet support context managers.")
    print("You must call close() explicitly.\n")

    # Manual cleanup pattern
    print("Recommended pattern:")
    print("```python")
    print("driver = None")
    print("try:")
    print("    driver = WinDriver(5000)")
    print("    # ... do work ...")
    print("finally:")
    print("    if driver:")
    print("        driver.close()")
    print("```\n")

    # Demonstrate
    driver = None
    try:
        driver = WinDriver(timeout_ms=5000)
        print(f"✓ Driver created: {repr(driver)}")
        # Do some work
        x, y = driver.get_curser_pos()
        print(f"✓ Did some work (cursor at {x}, {y})")
    finally:
        if driver:
            driver.close()
            print("✓ Driver cleaned up in finally block")

    print("\n✅ Resource management test PASSED")
    return True

if __name__ == "__main__":
    print("\n" + "=" * 70)
    print("BROMIUM SINGLETON PATTERN TESTS (BUG #6 FIX)")
    print("=" * 70 + "\n")

    results = []

    try:
        results.append(("Singleton Enforcement", test_singleton_enforcement()))
        results.append(("Close and Recreate", test_close_and_recreate()))
        results.append(("State Desynchronization Prevention", test_singleton_prevents_state_issues()))
        results.append(("Proper Usage Pattern", test_proper_usage_pattern()))
        results.append(("Resource Management", test_context_manager_pattern()))

        print("\n" + "=" * 70)
        print("TEST SUMMARY")
        print("=" * 70)

        for test_name, passed in results:
            status = "✅ PASSED" if passed else "❌ FAILED"
            print(f"{test_name}: {status}")

        all_passed = all(result[1] for result in results)

        if all_passed:
            print("\n🎉 All singleton tests passed!")
            print("\n✅ Bug #6 is FIXED - Single driver pattern enforced")
        else:
            print("\n❌ Some tests failed")

    except Exception as e:
        print(f"\n❌ Test suite failed with error: {e}")
        import traceback
        traceback.print_exc()
