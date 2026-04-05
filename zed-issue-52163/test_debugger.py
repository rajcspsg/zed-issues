#!/usr/bin/env python3
"""
Test file to reproduce Zed issue #52163: justMyCode not working in Python debugger.

This file demonstrates the issue where setting justMyCode=false should allow
stepping into external library code, but it doesn't work as expected.
"""

import json
from pathlib import Path


def process_data(data):
    """Process some data using external library functions."""
    print(f"Processing data: {data}")

    # This uses json.dumps from the standard library
    # With justMyCode=false, we should be able to step into json.dumps
    json_string = json.dumps(data, indent=2)
    print(f"JSON output: {json_string}")

    # This uses Path from pathlib
    # With justMyCode=false, we should be able to step into Path methods
    current_path = Path.cwd()
    print(f"Current path: {current_path}")

    return json_string


def main():
    """Main function to test the debugger."""
    print("Starting debugger test...")

    # Set a breakpoint here (line 31)
    test_data = {
        "name": "Test",
        "value": 42,
        "items": [1, 2, 3]
    }

    # Try to step into process_data, then step into json.dumps
    # Expected: With justMyCode=false, you should be able to step into json.dumps
    # Actual: Debugger acts as if justMyCode=true and skips over library code
    result = process_data(test_data)

    print(f"Result: {result}")
    print("Test complete!")


if __name__ == "__main__":
    main()
