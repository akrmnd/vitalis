# Testing Strategy for Vitalis Studio

## Overview
Vitalis Studio is a Tauri-based desktop application that combines React frontend with Rust backend. Due to the nature of Tauri applications, we use different testing strategies for different components.

## Test Types

### 1. Unit Tests (Rust Backend)
- **Location**: `vitalis-core/src/` and `vitalis-core/tests/`
- **Command**: `cargo test`
- **Docker**: `docker-compose --profile test run --rm test-rust`
- **Coverage**: Core business logic, parsers, statistics calculations, data structures

### 2. UI Tests (React Frontend)
- **Location**: `vitalis-app/tests/`
- **Command**: `npx playwright test` (run locally)
- **Coverage**: Basic UI rendering, input validation, responsive design
- **Limitation**: Cannot test Tauri API interactions in browser context

### 3. Type Checking
- **Command**: `npx tsc --noEmit`
- **Coverage**: TypeScript type safety across the frontend

## Why Limited E2E Testing?

Tauri applications run in a native WebView with access to system APIs through `window.__TAURI_INTERNALS__`. This context is not available when running tests in a regular browser with Playwright.

Options considered:
1. **Mock Tauri APIs**: Complex to maintain and doesn't test real integration
2. **Tauri Driver**: Requires native application build and display server
3. **Current Approach**: Focus on unit tests for logic, basic UI tests for interface ✅

## Running Tests

### Quick Test
```bash
# Rust backend tests
cargo test

# Basic UI tests
cd vitalis-app && npx playwright test
```

### Docker Test Suite
```bash
# Backend tests (Rust)
docker-compose --profile test run --rm test-rust

# Complete CI pipeline (Rust tests + linting)
docker-compose --profile ci run --rm ci-test
```

### Local Test Suite
```bash
# Frontend type checking
cd vitalis-app && npx tsc --noEmit

# UI tests
cd vitalis-app && npx playwright test
```

## Test Coverage

- ✅ **Backend Logic**: Comprehensive unit tests for all Rust modules
- ✅ **UI Components**: Basic rendering and interaction tests
- ✅ **Type Safety**: Full TypeScript checking
- ⚠️ **Integration**: Limited due to Tauri context requirement
- ❌ **E2E Workflow**: Not feasible in browser-based test environment

## Future Improvements

If full E2E testing becomes necessary:
1. Implement tauri-driver for native app testing
2. Set up headless display server (Xvfb) in Docker
3. Build and test actual Tauri binaries in CI