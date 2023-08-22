use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::SafeHtml;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub markdown: String,
    pub classes: Classes,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism, js_name = highlightAll)]
    fn highlightAll();
}

#[function_component(Markdown)]
pub fn markdown(props: &Props) -> Html {
    let html = match markdown_to_html::parser::parse_markdown(&props.markdown) {
        Ok((_, md)) => translator::translate(md),
        Err(e) => format!("Could not parse markdown: {}", e),
    };

    use_effect(|| {
        highlightAll();
    });

    html! {
        <div class={ classes!(props.classes.clone()) }>
        <SafeHtml html={ html }/>
        </div>
    }
}

mod translator {
    use markdown_to_html::{Markdown, MarkdownInline, MarkdownText};

    pub fn translate(md: Vec<Markdown>) -> String {
        md.iter()
            .map(|bit| match bit {
                Markdown::Heading(size, line) => translate_header(*size, line.to_vec()),
                Markdown::UnorderedList(lines) => translate_unordered_list(lines.to_vec()),
                Markdown::OrderedList(lines) => translate_ordered_list(lines.to_vec()),
                Markdown::Codeblock(lang, code) => {
                    translate_codeblock(lang.to_string(), code.to_string())
                }
                Markdown::Line(line) => translate_line(line.to_vec()),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn translate_boldtext(boldtext: String) -> String {
        format!("<b>{}</b>", boldtext)
    }

    fn translate_italic(italic: String) -> String {
        format!("<i>{}</i>", italic)
    }

    fn translate_inline_code(code: String) -> String {
        format!("<code>{}</code>", code)
    }

    fn translate_link(text: String, url: String) -> String {
        format!("<a href=\"{}\">{}</a>", url, text)
    }

    fn translate_image(text: String, url: String) -> String {
        log::info!("Image: {} {}", text, url);
        format!(
            "<a href=\"{}\" target=”_blank”><img src=\"{}\" alt=\"{}\" /></a>",
            url, url, text
        )
    }

    fn translate_list_elements(lines: Vec<MarkdownText>) -> String {
        lines
            .iter()
            .map(|line| format!("<li>{}</li>", translate_text(line.to_vec())))
            .collect::<Vec<String>>()
            .join("")
    }

    fn translate_header(size: usize, text: MarkdownText) -> String {
        format!("<h{}>{}</h{}>", size, translate_text(text), size)
    }

    fn translate_unordered_list(lines: Vec<MarkdownText>) -> String {
        format!("<ul>{}</ul>", translate_list_elements(lines.to_vec()))
    }

    fn translate_ordered_list(lines: Vec<MarkdownText>) -> String {
        format!("<ol>{}</ol>", translate_list_elements(lines.to_vec()))
    }

    // fn translate_code(code: MarkdownText) -> String {
    //     format!("<code>{}</code>", translate_text(code))
    // }

    fn translate_codeblock(lang: String, code: String) -> String {
        format!("<pre><code class=\"lang-{}\">{}</code></pre>", lang, code)
    }

    fn translate_line(text: MarkdownText) -> String {
        let line = translate_text(text);
        if line.len() > 0 {
            format!("<p>{}</p>", line)
        } else {
            format!("{}", line)
        }
    }

    fn translate_text(text: MarkdownText) -> String {
        text.iter()
            .map(|part| match part {
                MarkdownInline::Bold(text) => translate_boldtext(text.to_string()),
                MarkdownInline::Italic(text) => translate_italic(text.to_string()),
                MarkdownInline::InlineCode(code) => translate_inline_code(code.to_string()),
                MarkdownInline::Link(text, url) => {
                    translate_link(text.to_string(), url.to_string())
                }
                MarkdownInline::Image(text, url) => {
                    translate_image(text.to_string(), url.to_string())
                }
                MarkdownInline::Plaintext(text) => text.to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
