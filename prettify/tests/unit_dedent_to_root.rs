// use indoc::indoc;
// use prettify::{
//     concat, group, if_break, indent, join, line, mark_as_root, print, soft_line, string,
// };

// #[test]
// fn dedent_to_root_command() {
//     let mut items = Vec::new();
//     for i in 0..12 {
//         items.push(string(format!("item{}", i)));
//     }
// items.push(concat(vec![
//     string("["),
//     indent(concat(vec![
//         soft_line(),
//         join(
//             vec![string("itemA"), string("itemB")],
//             concat(vec![string(","), line()]),
//         ),
//         if_break(string(","), string(""), String::from("comma")),
//     ])),
//     soft_line(),
//     string("]"),
// ]));
//     let result = print(mark_as_root(group(concat(vec![
//         string("["),
//         indent(concat(vec![
//             soft_line(),
//             join(items, concat(vec![string(","), line()])),
//             if_break(string(","), string(""), String::from("comma")),
//         ])),
//         soft_line(),
//         string("]"),
//     ]))));
//     println!("---\n{}\n---", result);
//     assert_eq!(
//         result,
//         indoc! {r#"
//             [
//                 item0,
//                 item1,
//                 item2,
//                 item3,
//                 item4,
//                 item5,
//                 item6,
//                 item7,
//                 item8,
//                 item9,
//                 item10,
//                 item11,
//                 [
//                         itemA,
//             itemB,
//                 ],
//             ]"#}
//     );
// }
