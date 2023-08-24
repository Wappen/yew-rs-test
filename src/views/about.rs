use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container">
            <h1>{"About Me"}</h1>
            <p>
                { "I am an experienced joiner with well developed skills and experience in groundwork, concrete finishing and steel fixing and have worked in the construction industry since 1982. I am also a skilled labourer who has supported many different trades over the years. I have a full clean UK driving licence with entitlement of up to 7.5 tonne. I am keen to return to work after a period of training and personal development which has broadened my skills and experiences." }
            </p>
        </div>
    }
}
