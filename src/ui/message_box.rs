use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct MessageBoxProps<'a> {
    pub subject: &'a str,
    pub body: Option<&'a str>,
    pub provider: &'a str,
    pub model: &'a str,
}

#[component]
pub fn MessageBox<'a>(props: &MessageBoxProps<'a>) -> impl Into<AnyElement<'a>> {
    element! {
        View(
            flex_direction: FlexDirection::Column,
            border_style: BorderStyle::Round,
            border_color: Color::Cyan,
            padding_left: 1,
            padding_right: 1,
            padding_top: 1,
            padding_bottom: 1,
            margin: 1,
        ) {
            // Header
            View(flex_direction: FlexDirection::Row, margin_bottom: 1) {
                Text(
                    content: "✨ Generated Commit Message",
                    color: Color::White,
                    weight: Weight::Bold,
                )
            }

            // Provider info
            View(margin_bottom: 1) {
                Text(
                    content: format!("via {} ({})", props.provider, props.model),
                    color: Color::DarkGrey,
                )
            }

            // Subject line
            View(margin_bottom: 1) {
                Text(
                    content: props.subject,
                    color: Color::Green,
                    weight: Weight::Bold,
                )
            }

            // Body (if present)
            #(props.body.map(|body| element! {
                View(margin_top: 1) {
                    Text(
                        content: body,
                        color: Color::White,
                        wrap: TextWrap::Wrap,
                    )
                }
            }))
        }
    }
}

/// Display a commit message using iocraft
pub fn display_commit_message(subject: &str, body: Option<&str>, provider: &str, model: &str) {
    element! {
        MessageBox(
            subject: subject,
            body: body,
            provider: provider,
            model: model,
        )
    }.print();
}
