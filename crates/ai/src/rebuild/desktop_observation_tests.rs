use super::*;

/// A local Clean-room task must retain live editor observations across a reply such as "done".
/// Uploaded pictures of the home screen do not establish that the editor works or matches.
#[test]
fn live_desktop_observations_continue_beyond_uploaded_startup_pictures() {
    let d = tempfile::tempdir().unwrap();
    let ws = Workspace::create(d.path(), "Layout", "Recreate the document editor", TargetRef::Local { path: d.path().join("original.exe") }).unwrap();
    let kind = AppKind;
    let cx = BriefCx { ws: &ws, deep: false, ghidra: false, editor: None, assets: false, image_model: None, sees_pictures: true };
    let b = brief::brief(&kind, &cx, None);
    for tool in ["runtime_ui_read", "runtime_ui_walkthrough_record", "core_investigation_show", "core_handoff_write"] {
        assert!(b.contains(tool), "missing {tool}");
    }
    assert!(b.contains("after their reply") && b.contains("consent") && b.contains("process memory"));
    let deep = BriefCx { deep: true, ..cx };
    assert!(!brief::brief(&kind, &deep, None).contains(live_ui_reminder()));
    let pic = d.path().join("home.png");
    std::fs::write(&pic, "x").unwrap();
    ws.add_references(&[pic]).unwrap();
    let reminder = pictures_reminder(&ws, true);
    assert!(reminder.contains("latest request") && reminder.contains("corresponding evidence"));
    assert!(reminder.contains("only that screen") && reminder.contains("live UI evidence"));
}
