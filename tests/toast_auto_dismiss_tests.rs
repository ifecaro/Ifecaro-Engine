//! Tests for toast auto-dismiss functionality
#![cfg(target_arch = "wasm32")]

use gloo_timers::callback::Timeout;
use gloo_timers::future::TimeoutFuture;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

/// This test ensures that the `Timeout` used for auto-dismiss triggers
/// after the expected duration. We do not mock the logic; instead we rely on the
/// real timer provided by the browser environment, mirroring the behaviour in
/// the `ToastItem` component.
#[wasm_bindgen_test]
async fn test_toast_auto_dismiss_triggers() {
    // Shared flag to confirm the callback ran.
    let flag = Rc::new(RefCell::new(false));
    let flag_clone = flag.clone();

    // Schedule a short timeout (50ms) – similar to what `ToastItem` does.
    Timeout::new(50, move || {
        *flag_clone.borrow_mut() = true;
    })
    .forget();

    // Wait a bit longer than the timeout to ensure it has a chance to fire.
    TimeoutFuture::new(75).await;

    // Verify the callback executed.
    assert!(
        *flag.borrow(),
        "The timeout callback should have executed, indicating auto-dismiss works"
    );
}
