use sycamore::prelude::*;

use crate::wynn::items::{Item, Id, Powders};

#[derive(Prop)]
pub struct IdViewProps<'a> {
    item: &'a ReadSignal<Option<Item>>,
    ids: &'a ReadSignal<Vec<Id>>,
    powders: &'a ReadSignal<Vec<RcSignal<Option<Powders>>>>,
    rerolls: &'a ReadSignal<i32>
}

/// Offset for the wynntils id strings
const OFFSET: i32 = 0xF5000;

#[component]
pub fn IdView<'a, G: Html>(cx: Scope<'a>, props: IdViewProps<'a>) -> View<G> {
    // create the id string
    let idstring = create_selector(cx, || {
        let mut idstr = String::new();

        // encode the ids
        for id in props.ids.get().iter() {
            // check if the id is fixed
            if !(id.fixed || -1 <= id.baseval && id.baseval <= 1) {
                // check if we will save a value or a %
                if i32::abs(id.baseval) > 100 {
                    // calculate the %
                    idstr.push(
                        char::from_u32(
                            (OFFSET
                                + (f64::round(*id.value.get() as f64 * 100.0 / id.baseval as f64) as i32 - 30)
                                    * 4) as u32,
                        )
                        .unwrap_or('?'),
                    );
                } else {
                    // just push the value
                    idstr.push(char::from_u32((OFFSET + (*id.value.get() - id.min_id()) * 4) as u32).unwrap_or('?'));
                }
            } else {
                // I have no idea why this is here
                if !id.fixed {
                    idstr.push(char::from_u32(OFFSET as u32).unwrap_or('?'));
                }
            }
        }
        

        // do the powders
        let mut powders = String::new();
        let mut currpowder = 0;
        let mut appended = 0;

        for powder in props.powders.get().iter() {
            if appended == 4 {
                powders.push(char::from_u32(OFFSET as u32 + currpowder).unwrap());
                appended = 0;
                currpowder = 0;
            }
            
            if let Some(powder) = *powder.get() {
                currpowder *= 6;
                currpowder += powder.to_i32() as u32;
                appended += 1;
            }
        }
        if currpowder != 0 {
            powders.push(char::from_u32(OFFSET as u32 + currpowder).unwrap());
        }
        if !powders.is_empty() {
            idstr.push(char::from_u32(0xF5FF2).unwrap());
            idstr.push_str(&powders);
        }

        // add the rerolls
        idstr.push(char::from_u32((OFFSET + *props.rerolls.get()) as u32).unwrap());
        
        let name = if let Some(item) = &*props.item.get() {
            item.name.clone()
        } else {
            String::from("oops something went wrong")
        };

        // return the id string
        format!("󵿰{}󵿲{}󵿱", name, idstr)
    });

    view! {cx,
        div {
            code {(idstring.get())}
    
            button(
                class="pure-button pure-button-primary",
                style="width:100%",
                on:click= move |_| {
                    let clip = web_sys::window().unwrap().navigator().clipboard().unwrap();
    
                    let _ = clip.write_text(&idstring.get());
                }
            ) {"Copy to clipboard"}
        }
    }
}
