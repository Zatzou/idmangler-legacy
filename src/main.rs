use reqwasm::http::Request;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use wynn::items::{Id, IdentificationOrder, Item, ItemList};

use crate::{idview::IdView, itemrender::ItemRender, slider::Slider, wynn::items::Powders};

mod idview;
mod itemrender;
mod slider;
mod wynn;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            div(class="pure-g") {
                Suspense(fallback = view! {cx,
                    div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
                        div(class="box") {
                            h1(class="title") {"idMangler"}
                        }
                    }
                }) {
                    App {}
                }
            }
            div(style="position: fixed; bottom: 0; right: 0; z-index: -999; text-align: right") {
                h4(style="margin: 5px; color: #aaa") {(format!("idMangler v{}", VERSION))}
            }
        }
    });
}

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let item_list = get_itemlist().await;

    // render an error if we fail to load the item list
    if let Err(e) = item_list {
        return match e {
            reqwasm::Error::JsError(e) => view! {cx,
                div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
                    div(class="box") {
                        h1(class="title") {"Loading item data failed"}
                        p {(e.message)}
                        p {"Contact Zatzou#3457 on discord and I will look into fixing this (and if they are gone contact Super Probably Ender Non#2041)"}
                    }
                }
            },
            reqwasm::Error::SerdeError(e) => view! {cx,
                div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
                    div(class="box") {
                        h1(class="title") {"Loading item data failed"}
                        p {(e)}
                        p {"Contact Zatzou#3457 on discord and I will look into fixing this (and if they are gone contact Super Probably Ender Non#2041)"}
                    }
                }
            },
        };
    }

    // unwrap is safe here
    // we also leak the itemlist to make it live for 'static
    let item_list = Box::leak(Box::new(item_list.unwrap()));

    let itemnames = View::new_fragment(
        item_list
            .items
            .iter()
            .cloned()
            .map(|item| view! {cx, option(value=(item.name))})
            .collect(),
    );

    let searchtext = create_signal(cx, String::new());

    // rerolls
    let rerolls = create_rc_signal(1);
    let rerolls = create_ref(cx, rerolls);

    // the currently selected item
    let selected_item = create_selector(cx, || {
        let item = item_list
            .items
            .iter()
            .find(|item| item.name.eq_ignore_ascii_case(&searchtext.get()));

        if item.is_some() {
            rerolls.set(1);
        }
        item.cloned()
    });

    // the identifications of the currently selected item
    let selected_item_ids = create_selector(cx, || {
        if let Some(item) = &*selected_item.get() {
            read_ids(item, &item_list.order)
        } else {
            Vec::new()
        }
    });

    // powders of the current item
    let selected_item_powders = create_selector(cx, || {
        if let Some(item) = &*selected_item.get() {
            let mut powders = Vec::new();

            for _ in 0..item.max_powders {
                powders.push(create_rc_signal(None))
            }

            powders
        } else {
            Vec::<RcSignal<Option<Powders>>>::new()
        }
    });

    let ordering = &item_list.order;

    view! {cx,
        // main search box
        div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
            div(class="box") {
                h1(class="title") {"idMangler"}

                form(class="pure-form pure-form-stacked") {
                    // main input box
                    input(style="width: 100%;", prop:type="search", placeholder="Item name", list="items", bind:value=searchtext)

                    datalist(id="items") {(itemnames)}
                }
            }
        }

        // id editing box
        div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
            (if selected_item.get().is_some() {
                view! {cx,
                    div(class="box") {
                        h3 {"ID values"}
                        Keyed (
                            iterable=selected_item_ids,
                            view=|cx, id| {
                                // don't render if the id is fixed
                                if id.fixed || (-1 <= id.baseval && id.baseval <= 1) {
                                    return view! {cx,}
                                }

                                view! {cx,
                                    Slider(name=id.id.to_string(), min=id.min_id(), max=id.max_id(), value=id.value)
                                }
                            },
                            key=|id| id.baseval
                        )

                        // powder editor
                        (if selected_item_powders.get().len() != 0 {
                            view!{cx,
                                br {}
                                h3 {"Powders"}
                                div(style="display: flex; width: 100%; flex-wrap: wrap; justify-content: space-between; gap: 10px;") {
                                    Indexed (
                                        iterable=selected_item_powders,
                                        view=|cx, powder| {
                                            let powderval = if let Some(pow) = *powder.get() {
                                                pow.to_i32()
                                            } else { 0 };
                                            let value = create_signal(cx, powderval.to_string());

                                            let changeval = move |_| {
                                                powder.set(Powders::from_i32(value.get().parse().unwrap_or_default()))
                                            };

                                            view!(cx,
                                                form(class="pure-form", style="flex-grow: 1;") {
                                                    select(
                                                        style="width: 100%;",
                                                        on:change=changeval,
                                                        bind:value=value
                                                    ) {
                                                        option(value="6") {"None"}
                                                        option(class="mc-dark-green", value="0") {"Earth"}
                                                        option(class="mc-gold", value="1") {"Thunder"}
                                                        option(class="mc-blue", value="2") {"Water"}
                                                        option(class="mc-red", value="3") {"Fire"}
                                                        option(value="4") {"Air"}
                                                    }
                                                }
                                            )
                                        }
                                    )
                                }
                            }
                        } else {view!{cx,}})
                    }
                }
            } else {view!{cx,}})
        }

        // id viewer
        div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
            (if selected_item.get().is_some() {
                view! {cx,
                    div(class="box") {
                        h3 {"Result"}
                        IdView(item=selected_item, ids=selected_item_ids, powders=selected_item_powders, rerolls=rerolls)
                        br {}
                        Slider(name=String::from("Rolls"), min=1, max=1000, value=rerolls.clone())
                    }
                }
            } else {view!{cx,}})
        }

        // item viewer box
        div(class="pure-u-1 pure-u-sm-1-2 pure-u-md-1-3 pure-u-lg-1-4") {
            (if selected_item.get().is_some() {
                view! {cx,
                    div(class="box") {
                        h3 {"Preview"}
                        ItemRender(item=selected_item, ids=selected_item_ids, powders=selected_item_powders, rerolls=rerolls, ordering=ordering)
                    }
                }
            } else {view!{cx,}})
        }
    }
}

/// function to read ids into a btreemap from an item
fn read_ids(item: &Item, ord: &IdentificationOrder) -> Vec<Id> {
    let mut finalids = Vec::new();

    // ensure sorting
    let mut ordering = ord.order.iter().collect::<Vec<_>>();
    ordering.sort_by(|a, b| a.1.cmp(b.1));

    for (id, _) in ordering {
        if let Some(sid) = item.statuses.get(id) {
            finalids.push(Id {
                id: id.clone(),
                idtype: sid.r#type,
                fixed: sid.fixed,
                baseval: sid.base,
                value: create_rc_signal(sid.base),
            })
        }
    }

    finalids
}

/// Function to fetch the itemlist from wynntils
/// This function should probably have a fallback incase of api downtime but yea
async fn get_itemlist() -> Result<ItemList, reqwasm::Error> {
    let resp = Request::get("https://athena.wynntils.com/cache/get/itemList")
        .send()
        .await?;

    let item_list: ItemList = resp.json().await?;

    Ok(item_list)
}
