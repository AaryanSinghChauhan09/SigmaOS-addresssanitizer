# SigmaOS-addresssanitizer Component Wiki

## Overview
SigmaOS-addresssanitizer provides address sanitizer and memory error detection for the SigmaOS operating system. It demonstrates the memory safety subsystem and runtime error detection within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic sanitizer initialization.

## Principles
- Single Responsibility
- Memory Safety Abstraction
- Fail-Fast on Sanitizer Initialization Errors
- Deterministic Sanitizer Setup
- Clear Abstraction Over Sanitizer APIs
- Composition over Inheritance for sanitizer extensions

## Architecture
SigmaOS-addresssanitizer follows a sanitizer pattern:
- **Sanitizer Core**: Provides memory error detection, buffer overflow detection, and use-after-free detection
- **Runtime Instrumentation**: Supports compile-time and runtime instrumentation for error tracking
- **Error Reporting**: Supports structured error output and integration with debugging tools
- **Error Handling**: Fail-fast on invalid sanitizer configurations or unsupported architectures

## API
The component provides the following public APIs:
- `sanitize::enable()` - Enable address sanitizer for the current session
- `sanitize::disable()` - Disable address sanitizer
- `sanitize::report::current()` - Get the current sanitizer report
- `sanitize::check::memory(ptr, size)` - Check memory region for errors
- Error types for sanitizer failure scenarios or unsupported architectures

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific sanitizer configuration
2. **Act**: Execute the code under test (memory enable, error checking)
3. **Assert**: Verify the expected sanitizer results

Example:
```rust
#[test]
fn test_sanitize_enable_aaa() {
    // Arrange
    
    // Act
    let result = sanitize::enable();
    
    // Assert
    assert!(result.is_ok());
    assert!(sanitize::is_enabled());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Sanitizer initialization is best-effort; gracefully degrades on unsupported architectures
- Comprehensive error types for all sanitizer failure scenarios
- Unit tests provided for sanitizer enable/disable and error checking
- Integration tests for full memory safety subsystem with actual error detection

## TODO
- Add support for additional sanitizer types (thread sanitizer, loop sanitizer)
- Expand error reporting capabilities (custom reporters, integration with IDEs)
- Add architecture support (x86_64, aarch64, embedded targets)
- Create integration tests with actual memory error scenarios
- Implement sanitizer configuration and suppression features