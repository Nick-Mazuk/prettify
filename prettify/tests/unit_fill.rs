use prettify::{fill, line, print, string};

#[test]
fn fill_single_item() {
    assert_eq!(print(fill(vec![string("hello")])), "hello".to_string());
}

#[test]
fn fill_long_item() {
    assert_eq!(print(fill(vec![string("this is a very, very long line that is definitely over the eighty character limit for a single line.")])),
    "this is a very, very long line that is definitely over the eighty character limit for a single line.".to_string());
}

#[test]
fn fill_list_of_items() {
    let mut items = Vec::new();
    let count = 30;
    for i in 0..count {
        items.push(string(format!("item {},", i)));
        if i < count - 1 {
            items.push(line());
        }
    }
    assert_eq!(print(
        fill(
            items
        )
    ),
    "item 0, item 1, item 2, item 3, item 4, item 5, item 6, item 7, item 8, item 9, item 10,\nitem 11, item 12, item 13, item 14, item 15, item 16, item 17, item 18, item 19,\nitem 20, item 21, item 22, item 23, item 24, item 25, item 26, item 27, item 28,\nitem 29,".to_string());
}
