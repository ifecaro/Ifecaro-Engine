use std::fs;
use std::path::PathBuf;

#[test]
fn toast_animation_css_classes_exist() {
    // Build path to the tailwind.css file inside the project.
    let mut css_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    css_path.push("public/tailwind.css");

    let css_content = fs::read_to_string(&css_path)
        .expect("Failed to read tailwind.css. The file must exist for this test.");

    assert!(
        css_content.contains(".toast-animate-in"),
        "tailwind.css should include the .toast-animate-in class"
    );
    assert!(
        css_content.contains(".toast-animate-out"),
        "tailwind.css should include the .toast-animate-out class"
    );
}
