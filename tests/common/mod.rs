use std::sync::Once;

static INIT: Once = Once::new();

/// Initializes the test environment.
/// Guarantees that any global state (like loggers or environment variables)
/// is only set up once per test run, avoiding race conditions in multithreaded tests.
pub fn setup() {
  INIT.call_once(|| {
    // We could initialize the global logger here once the macro routing is fully implemented.
    // For now, it simply serves as a synchronization point.
  });
}

/// Helper function to temporarily set an environment variable for testing
/// and restore it afterward (though env::set_var in multithreaded tests
/// should be used carefully, it's fine for simple isolated integration tests).
#[allow(dead_code)]
pub fn with_env_var<F>(key: &str, value: &str, test: F)
where
  F: FnOnce(),
{
  let original = std::env::var_os(key);
  std::env::set_var(key, value);

  test();

  match original {
    Some(val) => std::env::set_var(key, val),
    None => std::env::remove_var(key),
  }
}
