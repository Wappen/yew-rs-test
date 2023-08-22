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
        Ok((_, md)) => markdown_to_html::translator::translate(md),
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
