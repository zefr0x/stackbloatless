use relm4::gtk::{self, prelude::*};

use std::str::FromStr;

use super::markdown2gtk::MarkdownView;
use crate::api::stackexchange::{Answer, Comment, DateExt, Question, User};

// TODO: Restructure this part of code as a relm component.

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

    {
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

        question_header.append(
            &gtk::Label::builder()
                .use_markup(true)
                .label(format!("<b>View Count:</b> {}", question.view_count))
                .margin_start(20)
                .build(),
        );

        question_header.append(
            &gtk::Label::builder()
                .use_markup(true)
                .label(format!(
                    "<b>Created:</b> {}",
                    question.creation_date.formate_date_time_string()
                ))
                .margin_start(20)
                .build(),
        );

        question_header.append(
            &gtk::Label::builder()
                .use_markup(true)
                .label(format!(
                    "<b>Last Active:</b> {}",
                    question.last_activity_date.formate_date_time_string()
                ))
                .margin_start(20)
                .build(),
        );

        {
            let owner_avatar = st_user(&question.owner, true);
            owner_avatar.set_halign(gtk::Align::End);
            owner_avatar.set_hexpand(true);

            question_header.append(&owner_avatar);
        }
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
        .tooltip_markup(format!("<b>Question ID:</b> {}", question.question_id))
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
            // TODO: Implement show-more button for comments.
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
    let main_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Answer Block
    let answer_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    main_layout.append(&answer_layout);

    // Answer sidebar
    let answer_sidebar_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .tooltip_markup(format!("<b>Answer ID:</b> {}", answer.answer_id))
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

    let answer_body = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    answer_layout.append(&answer_body);

    let answer_header = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    answer_body.append(&answer_header);

    answer_body.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    if answer.is_accepted {
        answer_header.append(
            &gtk::Label::builder()
                .label("Accepted")
                .css_classes(["success", "heading"])
                .build(),
        );
    }

    answer_header.append(
        &gtk::Label::builder()
            .use_markup(true)
            .label(format!(
                "<b>Created:</b> {}",
                answer.creation_date.formate_date_time_string()
            ))
            .margin_start(20)
            .build(),
    );

    answer_header.append(
        &gtk::Label::builder()
            .use_markup(true)
            .label(format!(
                "<b>Last Active:</b> {}",
                answer.last_activity_date.formate_date_time_string()
            ))
            .margin_start(20)
            .build(),
    );

    {
        let owner_avatar = st_user(&answer.owner, true);
        owner_avatar.set_halign(gtk::Align::End);
        owner_avatar.set_hexpand(true);

        answer_header.append(&owner_avatar);
    }

    {
        let md_view = MarkdownView::from_str(&answer.body_markdown).unwrap();

        answer_body.append(&md_view.text_view);
    }

    match &answer.comments {
        Some(comments) => {
            main_layout.append(
                &gtk::Label::builder()
                    // FIX: Use plural form for `Comments`.
                    .label(format!("{} Comments", answer.comment_count))
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

    gtk::Frame::builder()
        .child(&main_layout)
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
        .tooltip_markup(format!(
            "<b>Comment ID:</b> {}\n<b>Post ID:</b> {}",
            comment.comment_id, comment.post_id
        ))
        .build();
    comment_layout.append(&comment_sidebar_layout);

    comment_sidebar_layout.append(
        &gtk::Label::builder()
            .label(comment.score.to_string())
            .css_classes(if comment.score >= 0 {
                ["success"]
            } else {
                ["error"]
            })
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build(),
    );

    comment_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

    let comment_body_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    comment_layout.append(&comment_body_layout);

    match &comment.body_markdown {
        Some(body_markdown) => {
            let md_view = MarkdownView::from_str(body_markdown).unwrap();

            comment_body_layout.append(&md_view.text_view);
        }
        None => comment_body_layout.append(
            &gtk::Label::builder()
                .label("No content")
                .css_classes(["dim-label"])
                .build(),
        ),
    };

    // TODO: Make it adabtable with small window width.
    let comment_footer_layout = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    comment_body_layout.append(&comment_footer_layout);

    comment_footer_layout.append(&st_user(&comment.owner, true));

    comment_footer_layout.append(
        &gtk::Label::builder()
            .use_markup(true)
            .label(format!(
                " <b>at</b> {}",
                comment.creation_date.formate_date_time_string()
            ))
            .margin_end(10)
            .build(),
    );

    gtk::Frame::builder()
        .child(&comment_layout)
        .margin_top(5)
        .margin_bottom(5)
        .margin_start(5)
        .margin_end(5)
        .build()
}

fn st_user(user: &User, display_name: bool) -> gtk::LinkButton {
    let user_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    // TODO: Show user image.
    user_box.append(
        &adw::Avatar::builder()
            .name(&user.display_name)
            .show_initials(true)
            .margin_start(3)
            .margin_end(3)
            .margin_top(3)
            .margin_bottom(3)
            .build(),
    );

    if display_name {
        user_box.append(&gtk::Label::builder().label(&user.display_name).build());
    }

    gtk::LinkButton::builder()
        .child(&user_box)
        .uri(match &user.link {
            Some(link) => link,
            None => "",
        })
        .tooltip_markup(format!(
            "<b>Name:</b> {}\n<b>ID:</b> {}\n<b>Reputation:</b> {}",
            user.display_name,
            match user.user_id {
                Some(user_id) => user_id.to_string(),
                None => "Not Available".to_owned(),
            },
            user.reputation.unwrap()
        ))
        .build()
}
