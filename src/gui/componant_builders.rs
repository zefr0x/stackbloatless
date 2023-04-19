use relm4::prelude::gtk::{self, prelude::*};

use super::markdown2gtk::md2gtk;
use crate::api::stackexchange::{Answer, Comment, Question, User};

pub fn st_question(question: &Question) -> gtk::Box {
    let main_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Question title
    main_layout.append(
        &gtk::Label::builder()
            .selectable(true)
            .label(&question.title)
            .css_classes(["title-1"])
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::Char)
            .margin_start(5)
            .margin_end(5)
            .margin_bottom(10)
            .build(),
    );

    // Question info bar
    let question_header = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_bottom(20)
        .margin_start(10)
        .margin_end(10)
        .build();
    main_layout.append(&question_header);

    // Answered or not indecator
    if question.is_answered {
        question_header.append(
            &gtk::Label::builder()
                .label("Answered")
                .css_classes(["success", "heading"])
                .build(),
        );
    } else {
        question_header.append(
            &gtk::Label::builder()
                .label("Not Answered")
                .css_classes(["warning", "heading"])
                .build(),
        )
    }

    // Separator between header and question body
    main_layout.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    // Question Body
    let question_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    main_layout.append(&question_layout);

    // Question sidebar
    let question_sidebar_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    question_layout.append(&question_sidebar_layout);

    question_sidebar_layout.append(
        &gtk::Label::builder()
            .label(&question.score.to_string())
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .css_classes(if question.score >= 0 {
                ["success"]
            } else {
                ["error"]
            })
            .build(),
    );

    question_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    question_layout.append(&md2gtk(&question.body_markdown));

    main_layout
}

fn st_answer(answer: &Answer) {
    todo!("Answer")
}

fn st_comment(comment: &Comment) {
    todo!("Comment")
}
