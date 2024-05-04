use std::str::FromStr;

use markdown::mdast;
use relm4::{
    adw,
    gtk::{self, prelude::*},
};
use relm4_icons::icon_names;

use super::cell_object::CellObject;

// TODO: Refactor this mess if there is a better structure.

struct TagsColorScheme {
    code_background: String,
    quote_background: String,
    quote_foreground: String,
}

pub struct MarkdownView {
    pub text_view: gtk::TextView,
    buf: gtk::TextBuffer,
    tag_table: gtk::TextTagTable,
}

impl FromStr for MarkdownView {
    type Err = ();

    fn from_str(markdown_text: &str) -> Result<Self, Self::Err> {
        let mut object = Self::new();

        // TODO: Handle and return errors.
        object.append_text(markdown_text);

        Ok(object)
    }
}

impl MarkdownView {
    pub fn new() -> Self {
        let text_view = gtk::TextView::builder()
            .wrap_mode(gtk::WrapMode::Word)
            .editable(false)
            .cursor_visible(false)
            .css_classes(["body", "body_buffer"])
            .margin_top(10)
            .margin_start(10)
            .margin_end(10)
            .hexpand(true)
            .build();

        // Set buffer text.
        let buf = text_view.buffer();

        let tag_table = buf.tag_table();

        let mut object = MarkdownView {
            text_view,
            buf,
            tag_table,
        };

        object.load_text_tags();

        object
    }

    fn load_text_tags(&mut self) {
        let style_manager = adw::StyleManager::default();

        // FIX: Reload tags dynamicly when the system theme is changed.
        let color_scheme = if style_manager.is_dark() {
            // Dark mode colors
            TagsColorScheme {
                code_background: "#050505".to_owned(),
                quote_background: "#050505".to_owned(),
                quote_foreground: "#696969".to_owned(),
            }
        } else {
            // Light mode colors
            TagsColorScheme {
                code_background: "#b0b0b0".to_owned(),
                quote_background: "#505050".to_owned(),
                quote_foreground: "#ffffff".to_owned(),
            }
        };

        // Heading stylies
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("HEADING1")
                .size_points(30.0)
                .build(),
        );
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("HEADING2")
                .size_points(28.0)
                .build(),
        );
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("HEADING3")
                .size_points(26.0)
                .build(),
        );
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("HEADING4")
                .size_points(24.0)
                .build(),
        );
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("HEADING5")
                .size_points(22.0)
                .build(),
        );
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("HEADING6")
                .size_points(20.0)
                .build(),
        );

        self.tag_table
            .add(&gtk::TextTag::builder().name("BOLD").weight(700).build());

        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("EMPHASIS")
                .underline(gtk::pango::Underline::Single)
                .build(),
        );

        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("BLOCK_QUOTE")
                .background(&color_scheme.quote_background)
                .paragraph_background(&color_scheme.quote_background)
                .foreground(&color_scheme.quote_foreground)
                .left_margin(20)
                .indent(10)
                .pixels_below_lines(10)
                .pixels_above_lines(10)
                .style(gtk::pango::Style::Oblique)
                .build(),
        );

        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("INLINE_CODE")
                .font("monospace")
                .background(&color_scheme.code_background)
                .build(),
        );

        // FIX: When a line has no text, background color will not be applied.
        self.tag_table.add(
            &gtk::TextTag::builder()
                .name("CODE_BLOCK")
                .font("monospace")
                .background(&color_scheme.code_background)
                .paragraph_background(&color_scheme.code_background)
                .background_full_height(true)
                .indent(10)
                .pixels_below_lines(10)
                .pixels_above_lines(10)
                .build(),
        );
    }

    pub fn append_text(&mut self, markdown_text: &str) {
        // https://stackoverflow.com/editing-help
        // https://github.com/wooorm/markdown-rs
        // https://github.com/syntax-tree/mdast
        // https://docs.rs/markdown/1.0.0-alpha.7/markdown/mdast/enum.Node.html
        let tree = markdown::to_mdast(markdown_text, &markdown::ParseOptions::gfm()).unwrap();

        for node in tree.children().unwrap() {
            match node {
                mdast::Node::BlockQuote(_quote) => {
                    self.buf.insert(&mut self.buf.end_iter(), "\n");
                    self.buf.insert_with_tags_by_name(
                        &mut self.buf.end_iter(),
                        &node.to_string(),
                        &["BLOCK_QUOTE"],
                    );
                    self.buf.insert(&mut self.buf.end_iter(), "\n");
                }
                mdast::Node::List(list) => {
                    self.buf.insert(&mut self.buf.end_iter(), "\n\n");
                    self.md_list2buf(list, 1);
                    self.buf.insert(&mut self.buf.end_iter(), "\n");
                }
                mdast::Node::Heading(header) => {
                    self.buf.insert(&mut self.buf.end_iter(), "\n");

                    self.buf.insert_with_tags_by_name(
                        &mut self.buf.end_iter(),
                        &node.to_string(),
                        &[
                            "BOLD",
                            match header.depth {
                                1 => "HEADING1",
                                2 => "HEADING2",
                                3 => "HEADING3",
                                4 => "HEADING4",
                                5 => "HEADING5",
                                6 => "HEADING6",
                                _ => unreachable!(),
                            },
                        ],
                    );

                    self.buf.insert(&mut self.buf.end_iter(), "\n");
                }
                mdast::Node::Table(table) => {
                    self.buf.insert(&mut self.buf.end_iter(), "\n");
                    self.md_table2grid_view(table);
                    self.buf.insert(&mut self.buf.end_iter(), "\n");
                }
                mdast::Node::Paragraph(paragraph) => {
                    self.md_paragraph2buf(&paragraph.children);
                }
                mdast::Node::Code(code) => {
                    // let lang = code.lang.clone();
                    // let meta = code.meta.clone();
                    self.buf.insert(&mut self.buf.end_iter(), "\n\n");

                    self.buf.insert_with_tags_by_name(
                        &mut self.buf.end_iter(),
                        &code.value,
                        &["CODE_BLOCK"],
                    );

                    self.buf.insert(&mut self.buf.end_iter(), "\n\n");
                }
                mdast::Node::Html(_html) => {
                    // TODO: Convert simple HTML to GTK.
                    unimplemented!("HTML parsing")
                }
                _ => unimplemented!(),
            }
        }
    }

    fn md_paragraph2buf(&mut self, nodes: &Vec<mdast::Node>) {
        for node in nodes {
            match node {
                mdast::Node::InlineCode(code) => {
                    self.buf.insert_with_tags_by_name(
                        &mut self.buf.end_iter(),
                        &code.value,
                        &["INLINE_CODE"],
                    );
                }
                mdast::Node::Text(text) => self.buf.insert(&mut self.buf.end_iter(), &text.value),
                mdast::Node::Strong(strong) => {
                    for node in &strong.children {
                        match node {
                            mdast::Node::Text(text) => self.buf.insert_with_tags_by_name(
                                &mut self.buf.end_iter(),
                                &text.value,
                                &["BOLD"],
                            ),
                            // FIX: Make other children bold also.
                            _ => self.md_paragraph2buf(&vec![node.to_owned()]),
                        }
                    }
                }
                mdast::Node::Emphasis(emphasis) => {
                    for node in &emphasis.children {
                        match node {
                            mdast::Node::Text(text) => self.buf.insert_with_tags_by_name(
                                &mut self.buf.end_iter(),
                                &text.value,
                                &["EMPHASIS"],
                            ),
                            // FIX: Make other children emphasis also.
                            _ => self.md_paragraph2buf(&vec![node.to_owned()]),
                        }
                    }
                }
                mdast::Node::Link(link) => {
                    // Show icon along side the hyperlink
                    let anchor = gtk::TextChildAnchor::new();
                    self.buf
                        .insert_child_anchor(&mut self.buf.end_iter(), &anchor);

                    let icon = gtk::Image::builder()
                        .icon_name(icon_names::EARTH)
                        .margin_end(3)
                        .tooltip_text(&link.url)
                        .build();

                    self.text_view.add_child_at_anchor(&icon, &anchor);

                    // Create a link tag.
                    let tag = gtk::TextTag::builder()
                        .foreground("#90c2ff")
                        .underline(gtk::pango::Underline::Single)
                        .build();
                    // TODO: Save and asociate URI with the tag.
                    // TODO: Asociate a click event to this tag to open the URI.
                    // TODO: Open stackexchange links inside the app.
                    self.buf.tag_table().add(&tag);

                    self.buf
                        .insert_with_tags(&mut self.buf.end_iter(), &node.to_string(), &[&tag]);
                }
                mdast::Node::LinkReference(_link_ref) => {
                    todo!("LinkRefrence")
                }
                mdast::Node::Image(_image) => {
                    todo!("Image")
                }
                mdast::Node::ImageReference(_image_ref) => {
                    todo!("ImageReference")
                }
                mdast::Node::Paragraph(paragraph) => {
                    self.md_paragraph2buf(&paragraph.children);
                }
                _ => unimplemented!(),
            }
        }
    }

    fn md_list2buf(&mut self, list: &mdast::List, indent_level: u8) {
        let tag = gtk::TextTag::builder()
            .indent(20 * indent_level as i32)
            .style(gtk::pango::Style::Italic)
            .scale(1.3)
            .build();
        self.buf.tag_table().add(&tag);

        for (num, node) in list.children.iter().enumerate() {
            if list.ordered {
                self.buf.insert_with_tags(
                    &mut self.buf.end_iter(),
                    &format!("{}. ", num + 1),
                    &[&tag],
                )
            } else {
                self.buf
                    .insert_with_tags(&mut self.buf.end_iter(), "• ", &[&tag])
            }

            match node {
                mdast::Node::List(list) => {
                    self.md_list2buf(list, indent_level + 1);
                }
                mdast::Node::ListItem(list_item) => self.md_paragraph2buf(&list_item.children),
                _ => unimplemented!(),
            }

            self.buf.insert(&mut self.buf.end_iter(), "\n");
        }
    }

    fn md_table2grid_view(&mut self, table: &mdast::Table) {
        let model = gtk::gio::ListStore::new::<CellObject>();
        let mut columns_count = 0;
        let mut max_row_size = 0;

        // PERF: Find better way to find row max size.
        for row in &table.children {
            let size = row.children().unwrap().iter().len();

            if size > max_row_size {
                max_row_size = size;
            }
        }

        for (i, row) in table.children.iter().enumerate() {
            let row_iter = row.children().unwrap().iter();
            let row_size = row_iter.len();

            for (j, cell) in row_iter.enumerate() {
                // Create new column when needed
                if columns_count <= j as u32 {
                    columns_count += 1;
                }

                let mut is_header = false;
                // First row is for headers
                if i == 0 {
                    is_header = true;
                }

                // TODO: Accept cells alighnment in columns option.
                model.append(&CellObject::new(Some(&cell.to_string()), is_header));
            }

            // Add empty cells if the row is smaller the the longest row.
            if row_size < max_row_size {
                for _ in 0..(max_row_size - row_size) {
                    model.append(&CellObject::new(None, false));
                }
            }
        }

        let selection_model = gtk::MultiSelection::new(Some(model));

        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(|_factory, list_item| {
            let label = gtk::Label::new(None);
            list_item
                .downcast_ref::<gtk::ListItem>()
                .unwrap()
                .set_child(Some(&label));
        });

        factory.connect_bind(move |_, list_item| {
            // Get `CellObject` from `ListItem`
            let cell_object = list_item
                .downcast_ref::<gtk::ListItem>()
                .unwrap()
                .item()
                .and_downcast::<CellObject>()
                .unwrap();

            if let Some(string) = cell_object.string() {
                // Get `Label` from `ListItem`
                let label = list_item
                    .downcast_ref::<gtk::ListItem>()
                    .unwrap()
                    .child()
                    .and_downcast::<gtk::Label>()
                    .unwrap();

                // Set label from cell object string
                if cell_object.isheader() {
                    label.set_markup(&format!("<b>{}</b>", string));
                } else {
                    label.set_label(&string);
                }
            }
        });

        let view = gtk::GridView::builder()
            .model(&selection_model)
            .factory(&factory)
            .min_columns(columns_count)
            .max_columns(columns_count)
            .build();

        let anchor = gtk::TextChildAnchor::new();

        self.buf
            .insert_child_anchor(&mut self.buf.end_iter(), &anchor);

        self.text_view.add_child_at_anchor(&view, &anchor);
    }
}
