use super::super::doc::PrettifyConfig;
use super::shared::{Indent, IndentKind};
use std::rc::Rc;

pub fn make_indent(indent: Rc<Indent>, config: &PrettifyConfig) -> Rc<Indent> {
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

pub fn generate_indent(
    indent: Rc<Indent>,
    new_indent: Indent,
    config: &PrettifyConfig,
) -> Rc<Indent> {
    let queue = generate_initial_queue(&indent, Rc::new(new_indent));
    let mut value = String::new();

    for item in &queue {
        match &item.kind {
            Some(IndentKind::Indent) => {
                value.push_str(&" ".repeat(config.tab_width));
            }
            Some(IndentKind::StringAlign(text)) => {
                value.push_str(text);
            }
            Some(IndentKind::NumberAlign(width)) => {
                value.push_str(&" ".repeat(*width));
            }
            Some(IndentKind::Dedent) | None => {
                panic!("Unexpected indent kind");
            }
        }
    }

    Rc::new(Indent {
        length: value.len(),
        value,
        queue,
        kind: indent.as_ref().clone().kind,
    })
}

fn generate_initial_queue(indent: &Indent, new_indent: Rc<Indent>) -> Vec<Rc<Indent>> {
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
