use super::super::doc::PrettifyConfig;
use super::shared::{Indent, IndentKind};

pub fn make_indent(indent: Indent, config: &PrettifyConfig) -> Indent {
    generate_indent(
        indent,
        Indent {
            kind: Some(IndentKind::Indent),
            length: 0,
            queue: Vec::new(),
            value: String::new(),
        },
        config,
    )
}

pub fn generate_indent(indent: Indent, new_indent: Indent, config: &PrettifyConfig) -> Indent {
    // true implementation https://sourcegraph.com/github.com/prettier/prettier/-/blob/src/document/doc-printer.js?L19:10

    let queue = generate_initial_queue(&indent, new_indent);
    let mut value = String::new();
    let mut length = 0;
    let mut last_spaces = 0;

    for item in &queue {
        match &item.kind {
            Some(IndentKind::Indent) => {
                flush_spaces(&mut value, &mut length, &mut last_spaces);
                add_spaces(&mut value, config.tab_width, &mut length);
            }
            Some(IndentKind::StringAlign(text)) => {
                flush_spaces(&mut value, &mut length, &mut last_spaces);
                value.push_str(&text);
                length += text.len();
            }
            Some(IndentKind::NumberAlign(width)) => {
                last_spaces += width;
            }
            Some(IndentKind::Dedent) | None => {
                panic!("Unexpected indent kind");
            }
        }
    }

    flush_spaces(&mut value, &mut length, &mut last_spaces);

    Indent {
        value,
        length,
        queue,
        kind: indent.kind,
    }
}

fn generate_initial_queue(indent: &Indent, new_indent: Indent) -> Vec<Indent> {
    match new_indent.kind {
        Some(IndentKind::Dedent) => {
            let mut cloned_queue = indent.queue.clone();
            cloned_queue.pop();
            cloned_queue
        }
        _ => {
            let mut cloned_queue = indent.queue.clone();
            cloned_queue.push(new_indent);
            cloned_queue
        }
    }
}

fn add_spaces(value: &mut String, mut count: usize, length: &mut usize) {
    *length += count;
    while count > 0 {
        value.push(' ');
        count -= 1;
    }
}

fn flush_spaces(value: &mut String, last_spaces: &mut usize, length: &mut usize) {
    if (*last_spaces) > 0 {
        add_spaces(value, *last_spaces, length);
        *last_spaces = 0;
    }
}
