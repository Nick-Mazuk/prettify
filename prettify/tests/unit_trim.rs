use prettify::{concat, print, string, trim};

#[test]
fn trim_command() {
    assert_eq!(
        print(concat(vec![string("    hello    "), trim()])),
        "    hello".to_string()
    );
}
