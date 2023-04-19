use markdown::mdast;
use relm4::prelude::gtk::{self, prelude::*};

fn md_paragraph2buf(text_view: &gtk::TextView, buf: &gtk::TextBuffer, nodes: &Vec<mdast::Node>) {
    for node in nodes {
        match node {
            mdast::Node::InlineCode(code) => {
                buf.insert_with_tags_by_name(&mut buf.end_iter(), &code.value, &["inline_code"]);
            }
            mdast::Node::Text(text) => buf.insert(&mut buf.end_iter(), &text.value),
            mdast::Node::Strong(strong) => {
                for node in &strong.children {
                    match node {
                        mdast::Node::Text(text) => buf.insert_with_tags_by_name(
                            &mut buf.end_iter(),
                            &text.value,
                            &["bold"],
                        ),
                        // FIX: Make other children bold also.
                        _ => md_paragraph2buf(text_view, buf, &vec![node.to_owned()]),
                    }
                }
            }
            mdast::Node::Emphasis(emphasis) => {
                for node in &emphasis.children {
                    match node {
                        mdast::Node::Text(text) => buf.insert_with_tags_by_name(
                            &mut buf.end_iter(),
                            &text.value,
                            &["emphasis"],
                        ),
                        // FIX: Make other children emphasis also.
                        _ => md_paragraph2buf(text_view, buf, &vec![node.to_owned()]),
                    }
                }
            }
            mdast::Node::Link(link) => {
                let anchor = gtk::TextChildAnchor::new();

                buf.insert_child_anchor(&mut buf.end_iter(), &anchor);

                // TODO: Overwrite opener to open stackexchange link inside the app.
                // FIX: Improve style.

                let link_button = gtk::LinkButton::builder()
                    // FIX: Show its children.
                    .label("{LINK HOLDER}")
                    .uri(&link.url)
                    .build();

                text_view.add_child_at_anchor(&link_button, &anchor);
            }
            mdast::Node::LinkReference(link_ref) => {
                todo!("LinkRefrence")
            }
            mdast::Node::Image(image) => {
                todo!("Image")
            }
            mdast::Node::ImageReference(image_ref) => {
                todo!("ImageReference")
            }
            mdast::Node::Paragraph(paragraph) => {
                md_paragraph2buf(text_view, buf, &paragraph.children);
            }
            _ => unimplemented!(),
        }
    }
}

fn md_list2buf(
    text_view: &gtk::TextView,
    buf: &gtk::TextBuffer,
    list: &mdast::List,
    indent_level: u8,
) -> gtk::Box {
    let layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    for (num, node) in list.children.iter().enumerate() {
        for _ in 0..indent_level {
            buf.insert(&mut buf.end_iter(), "\t");
        }
        if list.ordered {
            buf.insert(&mut buf.end_iter(), &format!(" {}", num));
        } else {
            buf.insert(&mut buf.end_iter(), "• ")
        }

        match node {
            mdast::Node::List(list) => {
                md_list2buf(text_view, buf, list, indent_level + 1);
            }
            mdast::Node::ListItem(list_item) => {
                md_paragraph2buf(text_view, buf, &list_item.children)
            }
            _ => unimplemented!(),
        }

        buf.insert(&mut buf.end_iter(), "\n");
    }

    layout
}

pub fn md2gtk(markdown_text: &str) -> gtk::TextView {
    // https://stackoverflow.com/editing-help
    // https://github.com/wooorm/markdown-rs
    // https://github.com/syntax-tree/mdast
    // https://docs.rs/markdown/1.0.0-alpha.7/markdown/mdast/enum.Node.html
    let tree = markdown::to_mdast(markdown_text, &markdown::ParseOptions::default()).unwrap();

    let text_view = gtk::TextView::builder()
        .wrap_mode(gtk::WrapMode::Word)
        .editable(false)
        .css_classes(
            ["body", "body_buffer"]
                .iter()
                .map(|class| class.to_string())
                .collect(),
        )
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .hexpand(true)
        .build();

    // Set buffer text.
    let buf = text_view.buffer();

    load_text_tags(&buf);

    for node in tree.children().unwrap() {
        match node {
            mdast::Node::BlockQuote(quote) => {
                todo!("BlockQuote")
            }
            mdast::Node::List(list) => {
                buf.insert(&mut buf.end_iter(), "\n\n");
                md_list2buf(&text_view, &buf, list, 1);
                buf.insert(&mut buf.end_iter(), "\n");
            }
            mdast::Node::Heading(header) => {
                // FIX: Applay style to heading, and show children.
                md_paragraph2buf(&text_view, &buf, node.children().unwrap());
            }
            mdast::Node::Table(table) => {
                todo!("Table")
            }
            mdast::Node::Paragraph(paragraph) => {
                md_paragraph2buf(&text_view, &buf, &paragraph.children);
            }
            mdast::Node::Code(code) => {
                // let lang = code.lang.clone();
                // let meta = code.meta.clone();
                buf.insert(&mut buf.end_iter(), "\n\n");

                let anchor = gtk::TextChildAnchor::new();

                buf.insert_child_anchor(&mut buf.end_iter(), &anchor);

                let frame = gtk::Frame::builder().hexpand(true).build();

                // TODO: Apply monospace font.
                let code_text = gtk::Label::builder()
                    .use_markup(true)
                    .label(&code.value)
                    .hexpand(true)
                    .selectable(true)
                    .build();

                frame.set_child(Some(&code_text));

                text_view.add_child_at_anchor(&frame, &anchor);

                buf.insert(&mut buf.end_iter(), "\n\n");
            }
            mdast::Node::Html(html) => {}
            _ => dbg!(),
        }
    }

    text_view
}

fn load_text_tags(buf: &gtk::TextBuffer) {
    let tag_table = buf.tag_table();

    tag_table.add(&gtk::TextTag::builder().name("bold").weight(700).build());

    tag_table.add(
        &gtk::TextTag::builder()
            .name("emphasis")
            .underline(gtk::pango::Underline::Single)
            .build(),
    );

    tag_table.add(
        &gtk::TextTag::builder()
            .name("inline_code")
            .font("monospace")
            .background("#050505")
            .build(),
    );
}
