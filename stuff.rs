fn main() {
    sycamore::render(|cx| {
        let values = create_signal(cx, vec![create_rc_signal(1), create_rc_signal(2), create_rc_signal(3)]);

        view! { cx,
            Keyed(
                iterable = values,
                view = |cx, val| {
                    let val = create_ref(cx, val);

                    let value = create_signal(cx, val.to_string());

                    let incr = |_| val.set(*val.get() + 1);

                    create_effect(cx, || val.set(value.get().parse::<i32>().unwrap_or(0)));

                    view! { cx,
                        input(bind:value=value)
                        button(on:click=incr) { "+" }
                    }
                },
                key = |val| val.get()
            )

            Keyed(
                iterable = values,
                view = |cx, val| view! { cx,
                    p { (val.get()) }
                },
                key = |val| val.get()
            )
        }
    });
}