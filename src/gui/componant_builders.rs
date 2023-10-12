use relm4::gtk::{self, prelude::*};

use std::str::FromStr;

use super::markdown2gtk::MarkdownView;
use crate::api::stackexchange::{Answer, Comment, Question, User};

pub fn st_question(question: &Question) -> gtk::Box {
    let main_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Question title
    main_layout.append(
        &gtk::Label::builder()
            .selectable(true)
            .can_focus(false)
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
            .label(question.score.to_string())
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

    {
        let md_view = MarkdownView::from_str(&question.body_markdown).unwrap();

        question_layout.append(&md_view.text_view);
    }

    match &question.comments {
        Some(comments) => {
            main_layout.append(
                &gtk::Label::builder()
                    // FIX: Use plural form for `Comments`.
                    .label(format!("{} Comments", question.comment_count))
                    .css_classes(["heading"])
                    .halign(gtk::Align::Start)
                    .build(),
            );
            for comment in comments {
                main_layout.append(&st_comment(comment));
            }
        }
        None => {}
    }

    main_layout.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    match &question.answers {
        Some(answers) => {
            main_layout.append(
                &gtk::Label::builder()
                    // FIX: Use plural form for `Answers`.
                    .label(format!("{} Answers", question.answer_count))
                    .css_classes(["title-1"])
                    .margin_start(5)
                    .margin_end(5)
                    .margin_top(15)
                    .margin_bottom(10)
                    .halign(gtk::Align::Start)
                    .build(),
            );

            for answer in answers {
                main_layout.append(&st_answer(answer));
            }
        }
        None => {}
    }

    main_layout
}

fn st_answer(answer: &Answer) -> gtk::Frame {
    // Answer main area
    let answer_area_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Answer Body
    let answer_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    answer_area_layout.append(&answer_layout);

    // Answer sidebar
    let answer_sidebar_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    answer_layout.append(&answer_sidebar_layout);

    answer_sidebar_layout.append(
        &gtk::Label::builder()
            .label(answer.score.to_string())
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .css_classes(if answer.score >= 0 {
                ["success"]
            } else {
                ["error"]
            })
            .build(),
    );

    answer_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    {
        let md_view = MarkdownView::from_str(&answer.body_markdown).unwrap();

        answer_layout.append(&md_view.text_view);
    }

    match &answer.comments {
        Some(comments) => {
            for comment in comments {
                answer_area_layout.append(&st_comment(comment));
            }
        }
        None => {}
    }

    gtk::Frame::builder()
        .child(&answer_area_layout)
        .margin_top(15)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(15)
        .build()
}

fn st_comment(comment: &Comment) -> gtk::Frame {
    // Comment Body
    let comment_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    // Comment sidebar
    let comment_sidebar_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    comment_layout.append(&comment_sidebar_layout);

    comment_sidebar_layout.append(
        &gtk::Label::builder()
            .label(comment.score.to_string())
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .css_classes(if comment.score >= 0 {
                ["success"]
            } else {
                ["error"]
            })
            .build(),
    );

    comment_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    match &comment.body_markdown {
        Some(body_markdown) => {
            let md_view = MarkdownView::from_str(body_markdown).unwrap();

            comment_layout.append(&md_view.text_view);
        }
        None => comment_layout.append(
            &gtk::Label::builder()
                .label("No content")
                .css_classes(["dim-label"])
                .build(),
        ),
    };

    gtk::Frame::builder()
        .child(&comment_layout)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build()
}
