#[test]
fn convert() {
    assert_eq!(name_from_path::parse!("something"), "Something");
    assert_eq!(
        name_from_path::parse!("something/other-thing"),
        "Other Thing"
    );
    assert_eq!(
        name_from_path::parse!("something/other-thing.ttf"),
        "Other Thing"
    );
    assert_eq!(
        name_from_path::parse!("something/SomeThing-else.other.stuff"),
        "Some Thing Else"
    );
    assert_eq!(
        name_from_path::parse!("this/that/ran-_dom  words- test.otf"),
        "Ran Dom Words Test"
    );
}
