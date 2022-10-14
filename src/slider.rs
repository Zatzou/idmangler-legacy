use sycamore::prelude::*;

#[derive(Prop)]
pub struct SliderProps {
    name: String,
    min: i32,
    max: i32,
    value: RcSignal<i32>
}

#[component]
pub fn Slider<G: Html>(cx: Scope, props: SliderProps) -> View<G> {
    let val = create_ref(cx, props.value);

    let view = create_signal(cx, val.get().to_string());

    let tryset = move |_| {
        if let Ok(value) = view.get().parse::<i32>() {
            val.set(value);
        }
    };

    view! {cx,
        form(class="pure-form") {
            label(style="float: left;") {(props.name)}
            div(class="control") {
                input(
                    style="float: right; width: 100px;",
                    prop:type="number",
                    bind:value=view,
                    on:input=tryset
                )
            }
            div(class="control") {
                input(
                    style="width: 100%;",
                    prop:type="range",
                    prop:min=props.min,
                    prop:max=props.max,
                    bind:value=view,
                    on:input=tryset
                )
            }
        }
    }
}