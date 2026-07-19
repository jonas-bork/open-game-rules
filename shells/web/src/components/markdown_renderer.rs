use leptos::prelude::*;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};

#[component]
pub fn MarkdownRenderer(markdown: String) -> impl IntoView {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&markdown, options);

    let shifted_parser = parser.map(|event| match event {
        Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }) => Event::Start(Tag::Heading {
            level: shift_heading(level),
            id,
            classes,
            attrs,
        }),
        Event::End(TagEnd::Heading(level)) => Event::End(TagEnd::Heading(shift_heading(level))),
        other => other,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, shifted_parser);

    let safe_html = ammonia::clean(&html_output);

    view! {
        <div
            class="markdown-body"
            inner_html=safe_html
        />
    }
}

fn shift_heading(level: HeadingLevel) -> HeadingLevel {
    match level {
        HeadingLevel::H1 => HeadingLevel::H2,
        HeadingLevel::H2 => HeadingLevel::H3,
        HeadingLevel::H3 => HeadingLevel::H4,
        HeadingLevel::H4 => HeadingLevel::H5,
        // H6 is the maximum level in HTML, so we cap it there
        HeadingLevel::H5 | HeadingLevel::H6 => HeadingLevel::H6,
    }
}
