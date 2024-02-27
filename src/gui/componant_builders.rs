use std::str::FromStr;

use adw::prelude::*;
use relm4::prelude::*;

use super::markdown2gtk::MarkdownView;
use crate::api::stackexchange::{Answer, Comment, DateExt, Question, User};

pub struct QuestionPageModel {
    question: Question,
}

pub struct QuestionPageInit {
    pub question: Question,
}

pub struct QuestionPageWidgets;

// TODO: Make the question page async.
impl FactoryComponent for QuestionPageModel {
    type ParentWidget = adw::TabView;
    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = QuestionPageInit;
    type Root = gtk::ScrolledWindow;
    type Widgets = QuestionPageWidgets;
    type Index = DynamicIndex;

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        QuestionPageModel {
            question: init.question,
        }
    }

    fn init_root(&self) -> Self::Root {
        gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .build()
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        retured_widget: &adw::TabPage,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        root.set_child(Some(&Self::st_question(&self.question)));

        retured_widget.set_title(&self.question.title);

        // TODO: Pass question tags as keywords.
        // retured_widget.set_keyword(keyword);

        QuestionPageWidgets {}
    }
}

// TODO: Use grid layout for some cases.

impl QuestionPageModel {
    fn st_question(question: &Question) -> gtk::Box {
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

        // Header
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
                let owner_avatar = Self::st_user(&question.owner, true);
                owner_avatar.set_halign(gtk::Align::End);
                owner_avatar.set_hexpand(true);

                question_header.append(&owner_avatar);
            }

            // TODO: Show question tags
            // // Only 5 tags at a maximum
            // // Ref: https://stackoverflow.com/help/tagging
            // for tag in question.tags.iter() {
            //     question_header.append(&gtk::Label::new(Some(tag)));
            // }
        }

        // Separator between header and question
        main_layout.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

        // Question (sidebar and body)
        let question_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        main_layout.append(&question_layout);

        // Question sidebar
        {
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
        }

        // Separator between sidebar and question body
        question_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

        // Body
        {
            let md_view = MarkdownView::from_str(&question.body_markdown).unwrap();

            question_layout.append(&md_view.text_view);
        }

        // Comments
        if let Some(comments) = &question.comments {
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
                main_layout.append(&Self::st_comment(comment));
            }
        }

        // Separator between question area and answers
        main_layout.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

        // Answers
        if let Some(answers) = &question.answers {
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
                main_layout.append(&Self::st_answer(answer));
            }
        }

        main_layout
    }

    fn st_answer(answer: &Answer) -> gtk::Frame {
        // Main layout for answer area
        let main_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        // Answer Header
        {
            let answer_header = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            main_layout.append(&answer_header);

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
                let owner_avatar = Self::st_user(&answer.owner, true);
                owner_avatar.set_halign(gtk::Align::End);
                owner_avatar.set_hexpand(true);

                answer_header.append(&owner_avatar);
            }
        }

        // Separator between answer header and answer
        main_layout.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

        // Answer Block (sidebar and body, without comments)
        let answer_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        main_layout.append(&answer_layout);

        // Answer sidebar
        {
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
        }

        // Separator between sidebar and body
        answer_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

        // Body
        {
            let md_view = MarkdownView::from_str(&answer.body_markdown).unwrap();

            answer_layout.append(&md_view.text_view);
        }

        // Comments
        if let Some(comments) = &answer.comments {
            main_layout.append(
                &gtk::Label::builder()
                    // FIX: Use plural form for `Comments`.
                    .label(format!("{} Comments", answer.comment_count))
                    .css_classes(["heading"])
                    .halign(gtk::Align::Start)
                    .build(),
            );
            for comment in comments {
                main_layout.append(&Self::st_comment(comment));
            }
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
        let main_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        // Comment sidebar
        {
            let sidebar_layout = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .tooltip_markup(format!(
                    "<b>Comment ID:</b> {}\n<b>Post ID:</b> {}",
                    comment.comment_id, comment.post_id
                ))
                .build();
            main_layout.append(&sidebar_layout);

            sidebar_layout.append(
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
        }

        // Separator between sidebar and body
        main_layout.append(&gtk::Separator::new(gtk::Orientation::Vertical));

        let comment_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        main_layout.append(&comment_layout);

        match &comment.body_markdown {
            Some(body_markdown) => {
                let md_view = MarkdownView::from_str(body_markdown).unwrap();

                comment_layout.append(&md_view.text_view);
            }
            None => comment_layout.append(
                &gtk::Label::builder()
                    .label("No Content")
                    .css_classes(["dim-label"])
                    .build(),
            ),
        };

        // Meta data layout
        {
            // TODO: Make it adabtable with small window width.
            let comment_meta_layout = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            comment_layout.append(&comment_meta_layout);

            comment_meta_layout.append(&Self::st_user(&comment.owner, true));

            comment_meta_layout.append(
                &gtk::Label::builder()
                    .use_markup(true)
                    .label(format!(
                        " <b>at</b> {}",
                        comment.creation_date.formate_date_time_string()
                    ))
                    .margin_end(10)
                    .build(),
            );
        }

        gtk::Frame::builder()
            .child(&main_layout)
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
}
