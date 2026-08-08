// Regression guard: font-kit's macOS types must stay usable across threads.
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn native_font_handle_and_font_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<font_kit::handle::Handle>();
    assert_send_sync::<font_kit::font::Font>();
    assert_send_sync::<font_kit::loaders::core_text::NativeFont>();
    assert_send_sync::<font_kit::family_handle::FamilyHandle>();
}

#[cfg(all(feature = "source", any(target_os = "macos", target_os = "ios")))]
#[test]
fn handle_survives_a_thread_hop() {
    use font_kit::source::SystemSource;
    let handle = SystemSource::new()
        .select_by_postscript_name("ArialMT")
        .unwrap();
    let name = std::thread::spawn(move || handle.load().unwrap().postscript_name())
        .join()
        .unwrap();
    assert_eq!(name.as_deref(), Some("ArialMT"));
}
