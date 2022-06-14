#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/stylesheet/button-0.rs");
    t.pass("tests/ui/stylesheet/checkbox-0.rs");
    t.pass("tests/ui/stylesheet/text_input-0.rs");
    t.pass("tests/ui/stylesheet/container-0.rs");
    t.pass("tests/ui/stylesheet/container-1.rs");
}
