use prettify::{print, Doc, DocCommand};

#[test]
fn join_zero_elements() {
    // assert_eq!(
    //     print(&Doc::Command(DocCommand::Join(
    //         Box::new(Doc::String(",")),
    //         vec!()
    //     ))),
    //     "".to_string()
    // );
}

#[test]
fn join_one_element() {
    // assert_eq!(
    //     print(&Doc::Command(DocCommand::Join(
    //         Box::new(Doc::String(",")),
    //         vec!(Doc::String("a"))
    //     ))),
    //     "a".to_string()
    // );
}

#[test]
fn join_two_elements() {
    // assert_eq!(
    //     print(&Doc::Command(DocCommand::Join(
    //         Box::new(Doc::String(",")),
    //         vec!(Doc::String("a"), Doc::String("b"))
    //     ))),
    //     "a,b".to_string()
    // );
}

#[test]
fn join_three_elements() {
    // assert_eq!(
    //     print(&Doc::Command(DocCommand::Join(
    //         Box::new(Doc::String(",")),
    //         vec!(Doc::String("a"), Doc::String("b"), Doc::String("c"))
    //     ))),
    //     "a,b,c".to_string()
    // );
}

#[test]
fn join_custom_separator() {
    // assert_eq!(
    //     print(&Doc::Command(DocCommand::Join(
    //         Box::new(Doc::String("•")),
    //         vec!(Doc::String("a"), Doc::String("b"))
    //     ))),
    //     "a•b".to_string()
    // );
}
