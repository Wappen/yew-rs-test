use yew::prelude::*;

use crate::SafeHtml;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub markdown: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &Props) -> Html {
    let html = match markdown_to_html::parser::parse_markdown(&props.markdown) {
        Ok((_, md)) => markdown_to_html::translator::translate(md),
        Err(e) => format!("Could not parse html: {}", e),
    };

    html! {
        <SafeHtml html={ html }/>
    }
}
