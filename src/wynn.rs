//! Module for wynncraft type definitions
#![allow(clippy::upper_case_acronyms)]

/// Item information and data
pub mod items {
    use serde::Deserialize;
    use std::{collections::BTreeMap, fmt::Display, ops::RangeInclusive};
    use sycamore::reactive::RcSignal;

    /// All possible rarities of items
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub enum Rarity {
        COMMON,
        UNIQUE,
        RARE,
        LEGENDARY,
        FABLED,
        MYTHIC,
        SET,

        // TODO: fix once api issues are fixed
        #[serde(other)]
        INVALID,
    }

    // implement display for rarity
    impl Display for Rarity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Rarity::COMMON => "Common".fmt(f),
                Rarity::UNIQUE => "Unique".fmt(f),
                Rarity::RARE => "Rare".fmt(f),
                Rarity::LEGENDARY => "Legendary".fmt(f),
                Rarity::FABLED => "Fabled".fmt(f),
                Rarity::MYTHIC => "Mythic".fmt(f),
                Rarity::SET => "Set".fmt(f),
                Rarity::INVALID => "".fmt(f),
            }
        }
    }

    /// Item types
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub enum Type {
        SPEAR,
        WAND,
        BOW,
        DAGGER,
        RELIK,
        HELMET,
        CHESTPLATE,
        LEGGINGS,
        BOOTS,
        RING,
        BRACELET,
        NECKLACE,

        /// TODO: fix once api issues are resolved
        #[serde(other)]
        INVALID,
    }

    impl Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Type::SPEAR => "Spear".fmt(f),
                Type::WAND => "Wand".fmt(f),
                Type::BOW => "Bow".fmt(f),
                Type::DAGGER => "Dagger".fmt(f),
                Type::RELIK => "Relik".fmt(f),
                Type::HELMET => "Helmet".fmt(f),
                Type::CHESTPLATE => "Chestplate".fmt(f),
                Type::LEGGINGS => "Leggings".fmt(f),
                Type::BOOTS => "Boots".fmt(f),
                Type::RING => "Ring".fmt(f),
                Type::BRACELET => "Bracelet".fmt(f),
                Type::NECKLACE => "Necklace".fmt(f),

                Type::INVALID => "".fmt(f),
            }
        }
    }

    /// struct representing identification types
    #[allow(non_camel_case_types)]
    #[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct Identification(String);

    impl Display for Identification {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // handle some ids seperately and use general case for everything that works
            match self.0.to_lowercase().as_str() {
                "rawstrength" => "Strength".fmt(f),
                "rawdexterity" => "Dexterity".fmt(f),
                "rawintelligence" => "Intelligence".fmt(f),
                "rawdefence" => "Defence".fmt(f),
                "rawagility" => "Agility".fmt(f),
                "xpbonus" => "XP Bonus".fmt(f),
                _ => {
                    let mut s = String::new();
                    let mut itr = self.0.chars();

                    s.push(itr.next().unwrap().to_ascii_uppercase());

                    for c in itr {
                        if c.is_uppercase() {
                            s.push(' ');
                        }
                        s.push(c);
                    }

                    s.fmt(f)
                }
            }
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
    pub enum AttackSpeed {
        SUPER_SLOW,
        VERY_SLOW,
        SLOW,
        NORMAL,
        FAST,
        VERY_FAST,
        SUPER_FAST,
    }

    impl Display for AttackSpeed {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AttackSpeed::SUPER_SLOW => "Super Slow".fmt(f),
                AttackSpeed::VERY_SLOW => "Very Slow".fmt(f),
                AttackSpeed::SLOW => "Slow".fmt(f),
                AttackSpeed::NORMAL => "Normal".fmt(f),
                AttackSpeed::FAST => "Fast".fmt(f),
                AttackSpeed::VERY_FAST => "Very Fast".fmt(f),
                AttackSpeed::SUPER_FAST => "Super Fast".fmt(f),
            }
        }
    }

    #[derive(Deserialize, Clone)]
    pub struct ItemList {
        pub items: Vec<Item>,
        #[serde(rename = "identificationOrder")]
        pub order: IdentificationOrder,
    }

    /// Representation of a wynntils api item
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub struct Item {
        /// Name of the item
        #[serde(rename = "displayName")]
        pub name: String,
        /// Rarity of the item
        pub tier: Rarity,
        /// number of powders on the item
        #[serde(rename = "powderAmount", default)]
        pub max_powders: u8,
        /// Information about the item
        #[serde(rename = "itemInfo")]
        pub item_info: ItemInfo,
        /// Requirements to wear the item
        pub requirements: Requirements,
        /// Damage values of the item
        #[serde(rename = "damageTypes")]
        pub damages: Option<DamageTypes>,
        /// Defence values of the item
        #[serde(rename = "defenseTypes")]
        pub defenses: Option<DefenseTypes>,
        /// Attack speed of the item
        #[serde(rename = "attackSpeed")]
        pub speed: Option<AttackSpeed>,
        /// Statuses or the ids of the item
        pub statuses: BTreeMap<Identification, StatusId>,
    }

    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub struct ItemInfo {
        pub r#type: Type,
    }

    /// requirements of an item
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub struct Requirements {
        pub level: Option<i32>,
        pub strength: Option<i32>,
        pub dexterity: Option<i32>,
        pub intelligence: Option<i32>,
        pub defense: Option<i32>,
        pub agility: Option<i32>,
    }

    /// damagetypes of the item
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub struct DamageTypes {
        pub neutral: Option<String>,
        pub earth: Option<String>,
        pub thunder: Option<String>,
        pub water: Option<String>,
        pub fire: Option<String>,
        pub air: Option<String>,
    }

    /// defensetypes of the item
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub struct DefenseTypes {
        pub health: Option<i32>,
        pub earth: Option<i32>,
        pub thunder: Option<i32>,
        pub water: Option<i32>,
        pub fire: Option<i32>,
        pub air: Option<i32>,
    }

    /// Type of id
    #[allow(non_camel_case_types)]
    #[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
    pub enum StatusType {
        PERCENTAGE,
        INTEGER,
        FOUR_SECONDS,
        THREE_SECONDS,
        TIER,
    }

    /// Struct containing a single id for an item.
    ///
    /// This format is intended for deserialisation and does not contain the actual id type.
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub struct StatusId {
        pub r#type: StatusType,
        #[serde(rename = "isFixed")]
        pub fixed: bool,
        #[serde(rename = "baseValue")]
        pub base: i32,
    }

    /// Struct for holding the order of identifications as defied by the wynntils api
    #[derive(Deserialize, Clone)]
    pub struct IdentificationOrder {
        pub order: BTreeMap<Identification, i32>,
        pub groups: Vec<WynntilsRange>,
        pub inverted: Vec<Identification>,
    }

    /// Custom range type for deserializing the ranges from the wynntils json
    #[derive(Deserialize, Clone)]
    pub struct WynntilsRange(String);

    impl WynntilsRange {
        pub fn as_range(&self) -> RangeInclusive<i32> {
            let parts = self.0.split_once('-').unwrap_or(("0", "0"));
            let first = parts.0.parse().unwrap_or(0);
            let second = parts.1.parse().unwrap_or(0);

            first..=second
        }
    }

    /// Powder types
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Powders {
        EARTH,
        THUNDER,
        WATER,
        FIRE,
        AIR,
    }

    impl Powders {
        pub fn from_i32(n: i32) -> Option<Self> {
            match n {
                0 => Some(Powders::EARTH),
                1 => Some(Powders::THUNDER),
                2 => Some(Powders::WATER),
                3 => Some(Powders::FIRE),
                4 => Some(Powders::AIR),
                _ => None,
            }
        }

        pub fn to_i32(self) -> i32 {
            match self {
                Powders::EARTH => 1,
                Powders::THUNDER => 2,
                Powders::WATER => 3,
                Powders::FIRE => 4,
                Powders::AIR => 5,
            }
        }
    }

    #[derive(PartialEq, Eq, Clone)]
    pub struct Id {
        pub id: Identification,
        pub idtype: StatusType,
        pub fixed: bool,
        pub baseval: i32,
        pub value: RcSignal<i32>,
    }

    impl Id {
        pub fn max_id(&self) -> i32 {
            if self.fixed || (-1 <= self.baseval && self.baseval <= 1) {
                self.baseval
            } else if self.baseval < 1 {
                f64::round(self.baseval as f64 * 0.7) as i32
            } else {
                f64::round(self.baseval as f64 * 1.3) as i32
            }
        }
        pub fn min_id(&self) -> i32 {
            if self.fixed || (-1 <= self.baseval && self.baseval <= 1) {
                self.baseval
            } else if self.baseval < 1 {
                f64::round(self.baseval as f64 * 1.3) as i32
            } else {
                f64::round(self.baseval as f64 * 0.3) as i32
            }
        }
    }
}
