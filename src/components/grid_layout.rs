use itertools::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub columns: usize,
    pub children: Children,
}

#[function_component(GridLayout)]
#[allow(unstable_name_collisions)] // allow intersperse
pub fn grid_layout(props: &Props) -> Html {
    let class = format!("col-md-{}", 12 / props.columns);

    html! {
        <div class="container">
            <div class="row g-3">
                {
                    props.children.iter()
                        .map(|child| html! { <div class={ class.clone() }>{ child }</div> })
                        .chunks(props.columns) // Split into rows
                        .into_iter()
                        .map(|c| c.collect::<Vec<_>>())
                        .intersperse(vec![html! { // Add divider after each row
                            <div class="w-100"></div>
                        }])
                        .flat_map(|c| c.into_iter()) // Merge back into single level
                        .collect::<Html>()
                }
            </div>
        </div>
    }
}
