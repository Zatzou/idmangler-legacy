use std::ops::RangeInclusive;

use sycamore::prelude::*;

use crate::wynn::items::{Powders, Item, Id, StatusType, Identification, IdentificationOrder};

#[derive(Prop)]
pub struct ItemRenderProps<'a> {
    item: &'a ReadSignal<Option<Item>>,
    ids: &'a ReadSignal<Vec<Id>>,
    powders: &'a ReadSignal<Vec<RcSignal<Option<Powders>>>>,
    rerolls: &'a ReadSignal<i32>,
    ordering: &'a IdentificationOrder
}

#[component]
pub fn ItemRender<'a, G: Html>(cx: Scope<'a>, props: ItemRenderProps<'a>) -> View<G> {
    // get the item
    let item = (*props.item.get()).clone().unwrap();

    // rarity color of the item
    let item_col = match item.tier {
        crate::wynn::items::Rarity::NORMAL => "mc-white",
        crate::wynn::items::Rarity::UNIQUE => "mc-yellow",
        crate::wynn::items::Rarity::RARE => "mc-light-purple",
        crate::wynn::items::Rarity::LEGENDARY => "mc-aqua",
        crate::wynn::items::Rarity::FABLED => "mc-red",
        crate::wynn::items::Rarity::MYTHIC => "mc-purple",
        crate::wynn::items::Rarity::SET => "mc-green",
    };

    let ids = create_ref(cx, props.ids);

    let lastgroup: &Signal<Option<RangeInclusive<i32>>> = create_signal(cx, None);

    view! {cx,
        div(style="background-color: black; padding: 10px; border-radius: 5px;", class="mc-gray") {
            span(class=item_col) {(item.name)}
            br {}
            // item attack speed
            (if let Some(speed) = item.speed {
                view!{cx,
                    span {(format!("{speed} Attack Speed"))}
                    br {}
                }
            } else {view!{cx,}})
            br {}
            // render the damage values
            (if let Some(damages) = item.damages.clone() {
                view!{cx,
                    (if let Some(d) = damages.neutral.clone() {
                        view!{cx,
                            span(class="mc-gold") {(format!("✣ Neutral Damage: {d}"))}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = damages.fire.clone() {
                        view!{cx,
                            span(class="mc-red") {"✹ Fire "}
                            span {(format!("Damage: {d}"))}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = damages.water.clone() {
                        view!{cx,
                            span(class="mc-aqua") {"✽ Water "}
                            span {(format!("Damage: {d}"))}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = damages.air.clone() {
                        view!{cx,
                            span(class="mc-white") {"❋ Air "}
                            span {(format!("Damage: {d}"))}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = damages.thunder.clone() {
                        view!{cx,
                            span(class="mc-yellow") {"✦ Thunder "}
                            span {(format!("Damage: {d}"))}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = damages.earth.clone() {
                        view!{cx,
                            span(class="mc-dark-green") {"✤ Earth "}
                            span {(format!("Damage: {d}"))}
                        }
                    } else {view!{cx,}})
                    br {}
                }
            } else {view!{cx,}})
            // defences
            (if let Some(defs) = item.defenses.clone() {
                view!{cx,
                    (if let Some(d) = defs.health.clone() {
                        view!{cx,
                            span(class="mc-dark-red") {(format!("❤ Health: {d}"))}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = defs.fire.clone() {
                        view!{cx,
                            span(class="mc-red") {"✹ Fire "}
                            span {(format!("Defence: {d}"))}
                            br {}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = defs.water.clone() {
                        view!{cx,
                            span(class="mc-aqua") {"✽ Water "}
                            span {(format!("Defence: {d}"))}
                            br {}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = defs.air.clone() {
                        view!{cx,
                            span(class="mc-white") {"❋ Air "}
                            span {(format!("Defence: {d}"))}
                            br {}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = defs.thunder.clone() {
                        view!{cx,
                            span(class="mc-yellow") {"✦ Thunder "}
                            span {(format!("Defence: {d}"))}
                            br {}
                        }
                    } else {view!{cx,}})
                    (if let Some(d) = defs.earth.clone() {
                        view!{cx,
                            span(class="mc-dark-green") {"✤ Earth "}
                            span {(format!("Defence: {d}"))}
                            br {}
                        }
                    } else {view!{cx,}})
                }
            } else {view!{cx,}})
            br {}
            // requirements
            (if let Some(l) = item.requirements.level {
                view!{cx,
                    span {(format!("Combat Lv. Min: {l}"))}
                    br {}
                }
            } else {view!{cx,}})
            (if let Some(l) = item.requirements.strength {
                if l != 0 {
                    view!{cx,
                        span {(format!("Strength Min: {l}"))}
                        br {}
                    }
                } else {view!{cx,}}
            } else {view!{cx,}})
            (if let Some(l) = item.requirements.dexterity {
                if l != 0 {
                    view!{cx,
                        span {(format!("Dexterity Min: {l}"))}
                        br {}
                    }
                } else {view!{cx,}}
            } else {view!{cx,}})
            (if let Some(l) = item.requirements.intelligence {
                if l != 0 {
                    view!{cx,
                        span {(format!("Intelligence Min: {l}"))}
                        br {}
                    }
                } else {view!{cx,}}
            } else {view!{cx,}})
            (if let Some(l) = item.requirements.defense {
                if l != 0 {
                    view!{cx,
                        span {(format!("Defence Min: {l}"))}
                        br {}
                    }
                } else {view!{cx,}}
            } else {view!{cx,}})
            (if let Some(l) = item.requirements.agility {
                if l != 0 {
                    view!{cx,
                        span {(format!("Agility Min: {l}"))}
                        br {}
                    }
                } else {view!{cx,}}
            } else {view!{cx,}})
            br {}

            // idents
            Keyed (
                iterable=ids,
                view=move |cx, id| {
                    let id = create_ref(cx, id);

                    // get the type of the id
                    let end = match id.idtype {
                        StatusType::PERCENTAGE => "%",
                        StatusType::INTEGER => "",
                        StatusType::TIER => "",
                        StatusType::FOUR_SECONDS => "/4s",
                        StatusType::THREE_SECONDS => "/3s",
                    };

                    // figure out if the id is fixed
                    let fixed = id.fixed || (-1 <= id.baseval && id.baseval <= 1);

                    // the % of the id
                    let percent = create_selector(cx, move || if !fixed {
                        get_percent(*id.value.get(), &id, &props.ordering.inverted)
                    } else {
                        0.0
                    });

                    // figure out the color for the id
                    let val_col = if !props.ordering.inverted.contains(&id.id) {
                        if *id.value.get() >= 0 {
                            "mc-green"
                        } else {
                            "mc-red"
                        }
                    } else {
                        if *id.value.get() <= 0 {
                            "mc-green"
                        } else {
                            "mc-red"
                        }
                    };

                    // and the color for the %
                    let percent_col = create_selector(cx, move || if *percent.get() < 30.0 {
                        "mc-red"
                    } else if *percent.get() < 80.0 {
                        "mc-yellow"
                    } else if *percent.get() < 97.0 {
                        "mc-green"
                    } else {
                        "mc-aqua"
                    });

                    // seperate id groups
                    let spacing = if let Some(group) = &*lastgroup.get() {
                        // extra <br> if we're in a new group
                        if !group.contains(props.ordering.order.get(&id.id).unwrap_or(&0)) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    // set the lastgroup
                    for group in &props.ordering.groups {
                        if group.into_range().contains(props.ordering.order.get(&id.id).unwrap_or(&0)) {
                            lastgroup.set(Some(group.into_range()));
                        }
                    }

                    // formatting stuff
                    let formattednum = create_selector(cx, move || formatnum(*id.value.get()));
                    let idname = id.id.to_string();

                    view! {cx,
                        (if spacing {
                            view!{cx, br {}}
                        } else {view!{cx,}})
                        span(class=val_col) {(format!("{formattednum}{end} "))}
                        span {(idname)}
                        (if !fixed {
                            view! {cx,
                                span(class=percent_col) {(format!(" [{:.3}%]", percent))}
                            }
                        } else {view!{cx,}})
                        br {}
                    }
                },
                key=|id| id.value.get()
            )
            br {}
            (if props.powders.get().len() != 0 {
                let powders = create_ref(cx, props.powders);

                let usedpows = create_selector(cx, || {
                    powders.get().iter().filter(|p| p.get().is_some()).count()
                });
                
                view!{cx,
                    span {
                        (format!("[{usedpows}/{}] Powder slots ", powders.get().len()))

                        (if *usedpows.get() != 0 {
                            view!{cx,
                                "["
                                Indexed (
                                    iterable=powders,
                                    view=|cx, pow| {
                                        view!{cx,
                                            (if let Some(pow) = *pow.get() {
                                                let class = match pow {
                                                    Powders::EARTH => "mc-dark-green",
                                                    Powders::THUNDER => "mc-yellow",
                                                    Powders::WATER => "mc-aqua",
                                                    Powders::FIRE => "mc-red",
                                                    Powders::AIR => "mc-white",
                                                };

                                                let symbol = match pow {
                                                    Powders::EARTH => "✤",
                                                    Powders::THUNDER => "✦",
                                                    Powders::WATER => "✽",
                                                    Powders::FIRE => "✹",
                                                    Powders::AIR => "❋",
                                                };
                                                
                                                view!{cx,
                                                    span(class=class) {(symbol)}
                                                }
                                            } else {view!{cx,}})
                                        }
                                    }
                                )
                                "]"
                            }
                        } else {view!{cx,}})
                    }
                }
            } else {view!{cx,}})
            br {}
            span(class=item_col) {
                (format!("{} {}", item.tier, item.item_info.r#type))
                (if *props.rerolls.get() > 1 {
                    view!{cx, (format!{" [{}]", props.rerolls.get()})}
                } else {view!{cx,}})
            }
        }
    }
}

/// function for adding a + to positive numbers
fn formatnum(num: i32) -> String {
    if num > 0 {
        return format!("+{}", num);
    } else {
        return format!("{}", num);
    }
}

/// % calc stuff
fn get_percent(value: i32, id: &Id, inverted: &Vec<Identification>) -> f64 {
    // something stolen from the wynntils code
    let percent =
        ((value as f64 - id.min_id() as f64) / (id.max_id() as f64 - id.min_id() as f64)) * 100.0;

    return if inverted.contains(&id.id) {
        100.0 - percent
    } else {
        percent
    };
}
