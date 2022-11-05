//! Module for wynncraft type definitions
#![allow(clippy::upper_case_acronyms)]

/// Item information and data
pub mod items {
    use serde::Deserialize;
    use sycamore::reactive::RcSignal;
    use std::{collections::BTreeMap, ops::RangeInclusive, fmt::Display};

    /// All possible rarities of items
    #[derive(Deserialize, Clone, PartialEq, Eq)]
    pub enum Rarity {
        NORMAL,
        UNIQUE,
        RARE,
        LEGENDARY,
        FABLED,
        MYTHIC,
        SET,
    }

    // implement display for rarity
    impl Display for Rarity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Rarity::NORMAL => "Normal".fmt(f),
                Rarity::UNIQUE => "Unique".fmt(f),
                Rarity::RARE => "Rare".fmt(f),
                Rarity::LEGENDARY => "Legendary".fmt(f),
                Rarity::FABLED => "Fabled".fmt(f),
                Rarity::MYTHIC => "Mythic".fmt(f),
                Rarity::SET => "Set".fmt(f),
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
            }
        }
    }

    /// all wynncraft identifications
    /// 
    /// This enum offers the Other([String]) variant for ids that get updated and don't exist currently for compatibility
    #[allow(non_camel_case_types)]
    #[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub enum Identification {
        rawStrength,
        rawDexterity,
        rawIntelligence,
        rawDefence,
        rawAgility,
        attackSpeed,
        rawMainAttackDamage,
        mainAttackDamage,
        rawSpellDamage,
        spellDamage,
        rawThunderSpellDamage,
        rawFireSpellDamage,
        rawAirSpellDamage,
        rawEarthSpellDamage,
        rawWaterSpellDamage,
        rawHealth,
        rawHealthRegen,
        healthRegen,
        lifeSteal,
        manaRegen,
        manaSteal,
        earthDamage,
        thunderDamage,
        waterDamage,
        fireDamage,
        airDamage,
        earthDefence,
        thunderDefence,
        waterDefence,
        fireDefence,
        airDefence,
        exploding,
        poison,
        thorns,
        reflection,
        walkSpeed,
        sprint,
        sprintRegen,
        rawJumpHeight,
        soulPointRegen,
        lootBonus,
        lootQuality,
        stealing,
        xpBonus,
        gatherXpBonus,
        gatherSpeed,
        raw1stSpellCost,
        #[serde(rename = "1stSpellCost")]
        SpellCost1,
        raw2ndSpellCost,
        #[serde(rename = "2ndSpellCost")]
        SpellCost2,
        raw3rdSpellCost,
        #[serde(rename = "3rdSpellCost")]
        SpellCost3,
        raw4thSpellCost,
        #[serde(rename = "4thSpellCost")]
        SpellCost4,
        /// This variant is intended to help with any weird api incompatibilities
        Other(String)
    }

    impl Display for Identification {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Identification::rawStrength => "Strength".fmt(f),
                Identification::rawDexterity => "Dexterity".fmt(f),
                Identification::rawIntelligence => "Intelligence".fmt(f),
                Identification::rawDefence => "Defence".fmt(f),
                Identification::rawAgility => "Agility".fmt(f),
                Identification::attackSpeed => "Attack Speed".fmt(f),
                Identification::rawMainAttackDamage => "Main Attack Neutral Damage".fmt(f),
                Identification::mainAttackDamage => "Main Attack Damage".fmt(f),
                Identification::rawSpellDamage => "Spell Damage".fmt(f),
                Identification::spellDamage => "Spell Damage".fmt(f),
                Identification::rawHealth => "Health".fmt(f),
                Identification::rawHealthRegen => "Health Regen".fmt(f),
                Identification::healthRegen => "Health Regen".fmt(f),
                Identification::lifeSteal => "Life Steal".fmt(f),
                Identification::manaRegen => "Mana Regen".fmt(f),
                Identification::manaSteal => "Mana Steal".fmt(f),
                Identification::earthDamage => "Earth Damage".fmt(f),
                Identification::thunderDamage => "Thunder Damage".fmt(f),
                Identification::waterDamage => "Water Damage".fmt(f),
                Identification::fireDamage => "Fire Damage".fmt(f),
                Identification::airDamage => "Air Damage".fmt(f),
                Identification::earthDefence => "Earth Defence".fmt(f),
                Identification::thunderDefence => "Thunder Defence".fmt(f),
                Identification::waterDefence => "Water Defence".fmt(f),
                Identification::fireDefence => "Fire Defence".fmt(f),
                Identification::airDefence => "Air Defence".fmt(f),
                Identification::exploding => "Exploding".fmt(f),
                Identification::poison => "Poison".fmt(f),
                Identification::thorns => "Thorns".fmt(f),
                Identification::reflection => "Reflection".fmt(f),
                Identification::walkSpeed => "Walk Speed".fmt(f),
                Identification::sprint => "Sprint".fmt(f),
                Identification::sprintRegen => "Sprint Regen".fmt(f),
                Identification::rawJumpHeight => "Jump Height".fmt(f),
                Identification::soulPointRegen => "Soul Point Regen".fmt(f),
                Identification::lootBonus => "Loot Bonus".fmt(f),
                Identification::lootQuality => "Loot Quality".fmt(f),
                Identification::stealing => "Stealing".fmt(f),
                Identification::xpBonus => "XP Bonus".fmt(f),
                Identification::gatherXpBonus => "Gather XP Bonus".fmt(f),
                Identification::gatherSpeed => "Gather Speed".fmt(f),
                Identification::raw1stSpellCost => "1st Spell Cost".fmt(f),
                Identification::SpellCost1 => "1st Spell Cost".fmt(f),
                Identification::raw2ndSpellCost => "2nd Spell Cost".fmt(f),
                Identification::SpellCost2 => "2nd Spell Cost".fmt(f),
                Identification::raw3rdSpellCost => "3rd Spell Cost".fmt(f),
                Identification::SpellCost3 => "3rd Spell Cost".fmt(f),
                Identification::raw4thSpellCost => "4th Spell Cost".fmt(f),
                Identification::SpellCost4 => "4th Spell Cost".fmt(f),
                Identification::rawThunderSpellDamage => "Thunder Spell Damage".fmt(f),
                Identification::rawFireSpellDamage => "Fire Spell Damage".fmt(f),
                Identification::rawAirSpellDamage => "Air Spell Damage".fmt(f),
                Identification::rawEarthSpellDamage => "Earth Spell Damage".fmt(f),
                Identification::rawWaterSpellDamage => "Water Spell Damage".fmt(f),
                // just pass other possible strings thru
                Identification::Other(s) => s.fmt(f),
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
        #[serde(rename = "powderAmount")]
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
            let parts = self.0.split_once('-').unwrap_or(("0","0"));
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
