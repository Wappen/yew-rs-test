use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<Column>,
}

#[function_component(ColumnLayout)]
pub fn column_layout(props: &Props) -> Html {
    html! {
        <div class="container">
            <div class="row">
                { props.children.clone() }
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ColumnProps {
    pub width: i32,
    pub children: Option<Children>,
}

#[function_component(Column)]
pub fn column(props: &ColumnProps) -> Html {
    let class = format!("col-sm-{}", props.width);

    match &props.children {
        Some(c) => {
            html! {
                <div class={ classes!(class) }>
                    {
                        c.clone()
                    }
                </div>
            }
        }
        None => {
            html! {
                <div class={ classes!(class) }>
                </div>
            }
        }
    }
}
