use prettify::{fill, join_to_vector, line, print, string};

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
    for i in 0..30 {
        items.push(string(format!("item {},", i)));
    }
    assert_eq!(print(
        fill(
            join_to_vector(items, line())
        )
    ),
    "item 0, item 1, item 2, item 3, item 4, item 5, item 6, item 7, item 8, item 9,\nitem 10, item 11, item 12, item 13, item 14, item 15, item 16, item 17, item 18,\nitem 19, item 20, item 21, item 22, item 23, item 24, item 25, item 26, item 27,\nitem 28, item 29,".to_string());
}
