use iced_core::Font;
use sleet::fonts;

const fn give() -> Font {
    Font::External { name: "Give", bytes: include_bytes!("res/dummy.ttf") }
}

fonts! {
    ROBOTO_MONO_0: "res/dummy.ttf"
}

fonts! {
    ROBOTO_MONO_1: "res/dummy.ttf",
    pub ROBOTO_MONO_2: "res/dummy-textConvert.otf.ttf",
    pub ROBOTO_MONO_3: give()
}

#[test]
fn check() {
    match ROBOTO_MONO_0 {
        Font::External { name, bytes: _ } => assert_eq!(name, "Dummy"),
        _ => panic!()
    }
    match ROBOTO_MONO_1 {
        Font::External { name, bytes: _ } => assert_eq!(name, "Dummy"),
        _ => panic!()
    }
    match ROBOTO_MONO_2 {
        Font::External { name, bytes: _ } => assert_eq!(name, "Dummy Text Convert"),
        _ => panic!()
    }
    match ROBOTO_MONO_3 {
        Font::External { name, bytes: _ } => assert_eq!(name, "Give"),
        _ => panic!()
    }
}
