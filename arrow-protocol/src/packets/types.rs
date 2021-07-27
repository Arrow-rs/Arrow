use std::{
    io::{Error, ErrorKind, Read},
    marker::PhantomData,
};

use nbt::{de::Decoder, to_writer, Blob};
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

use crate::serde::varint::{read_varint, write_varint};

/// Difficulty type
#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    /// peaceful difficulty
    Peaceful = 0,
    /// easy difficulty
    Easy = 1,
    /// normal difficulty
    Normal = 2,
    /// hard difficulty
    Hard = 3,
}

/// Gamemode type
#[derive(Serialize, Deserialize)]
pub enum Gamemode {
    /// If no previous gamemode exists
    NoPreviousMode = -1,
    /// Survival mode
    Survival = 0,
    /// Creative mode
    Creative = 1,
    /// Adventure mode
    Adventure = 2,
    /// Spectator mode
    Spectator = 3,
}

/// LevelType type
#[derive(Serialize, Deserialize)]
pub enum LevelType {
    /// default world
    Default,
    /// flat world
    Flat,
    /// largeBiomes world
    LargeBiomes,
    /// amplified world
    Amplified,
    /// customized world
    Customized,
    /// buffet world
    Buffet,
    /// default_1_1 world
    Default11,
}

///
#[allow(missing_docs)]
pub struct DimensionCodec {
    pub dimension_registry: DimensionRegistry,
    pub biome_registry: BiomeRegistry,
}

///
#[allow(missing_docs)]
pub struct DimensionRegistry {
    pub dimension_type: String,
    pub value: Vec<DimensionRegistryEntry>,
}
///
#[allow(missing_docs)]
pub struct DimensionRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: DimensionType,
}
///
#[allow(missing_docs)]
#[derive(Clone)]
pub struct DimensionType {
    pub piglin_safe: bool,
    pub natural: bool,
    pub ambient_light: f32,
    pub fixed_time: Option<i64>,
    pub infiniburn: String,
    pub respawn_anchor_works: bool,
    pub has_skylight: bool,
    pub bed_works: bool,
    pub effects: String,
    pub has_raids: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub coordinate_scale: f32,
    pub ultrawarm: bool,
    pub has_ceiling: bool,
}

///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeRegistry {
    #[serde(rename = "type")]
    pub biome_type: String,
    pub value: Vec<BiomeRegistryEntry>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: BiomeProperties,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeProperties {
    pub precipitation: String,
    pub depth: f32,
    pub temperature: f32,
    pub scale: f32,
    pub downfall: f32,
    pub category: String,
    pub temperature_modifier: Option<String>,
    pub effects: BiomeEffects,
    pub particle: Option<BiomeParticles>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeEffects {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
    pub foilage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<String>,
    pub music: Option<BiomeMusicProperties>,
    pub ambient_sound: Option<String>,
    pub additions_sound: Option<AdditionSoundProperties>,
    pub mood_sound: Option<MoodSoundProperties>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeMusicProperties {
    pub replace_current_music: i8,
    pub sound: String,
    pub max_delay: i32,
    pub min_delay: i32,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct AdditionSoundProperties {
    pub sound: String,
    pub tick_chance: f64,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct MoodSoundProperties {
    pub sound: String,
    pub tick_delay: i32,
    pub offset: f64,
    pub block_search_extent: i32,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeParticles {
    pub probability: f32,
    pub options: BiomeParticleOptions,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}

/// A crafting recipe.
pub struct Recipe {
    /// The recipe id.
    pub(crate) id: String,
    /// The type of the recipe.
    pub(crate) ty: String,
    /// The data for the recipe.
    pub(crate) data: Option<RecipeData>,
}

impl Recipe {
    /// Create a new recipe.
    pub fn new(id: String, ty: String, data: Option<RecipeData>) -> Self {
        Self { id, ty, data }
    }

    /// Get a reference to the recipe's id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Get a reference to the recipe's ty.
    pub fn ty(&self) -> &str {
        self.ty.as_str()
    }

    /// Get a mutable reference to the recipe's data.
    pub fn data(&self) -> &Option<RecipeData> {
        &self.data
    }
}

/// The recipe data.
#[allow(missing_docs)]
pub enum RecipeData {
    CraftingShapeless {
        group: String,
        ingridients: Vec<Ingridient>,
        result: Slot,
    },
    CraftingShaped {
        width: i32,
        height: i32,
        group: String,
        ingridients: Vec<Ingridient>,
        result: Slot,
    },
    CraftingSpecialArmorDye,
    CraftingSpecialBookCloning,
    CraftingSpecialMapCloning,
    CraftingSpecialMapExtending,
    CraftingSpecialFireworkRocket,
    CraftingSpecialFireworkStar,
    CraftingSpecialFireworkStarFade,
    CraftingSpecialRepairItem,
    CraftingSpecialTippedArrow,
    CraftingSpecialBannerDuplicate,
    CraftingSpecialBannerAddPattern,
    CraftingSpecialShieldDecoration,
    CraftingSpecialShulkerBoxColoring,
}

/// A crafting ingridient.
pub struct Ingridient {
    pub(crate) items: Vec<Slot>,
}

impl Ingridient {
    /// Create a new [`Ingridient`].
    pub fn new(items: Vec<Slot>) -> Self {
        Self { items }
    }

    /// Get a mutable reference to the ingridient's items.
    pub fn items(&self) -> &Vec<Slot> {
        &self.items
    }
}

/// A slot.
pub struct Slot {
    pub(crate) data: Option<SlotData>,
}

/// The data for a slot.
pub struct SlotData {
    pub(crate) id: i16,
    pub(crate) count: u8,
    pub(crate) damage: i16,
    pub(crate) nbt: Blob,
}

/// A struct to serialize and deserialize NBT data.
pub struct Nbt<'a, T>(PhantomData<&'a T>, pub T);
struct NbtVisitor<'a, T>(PhantomData<&'a T>);

/// A struct serializing to a length prefixed [`Vec`] of `T`s.
pub struct LengthPrefixedVec<'a, T>(PhantomData<&'a T>, pub Vec<T>);
struct LengthPrefixedVecVisitor<'a, T>(PhantomData<&'a T>);

impl LevelType {
    /// used to convert enum value to String
    pub fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("default"),
            Self::Flat => String::from("flat"),
            Self::LargeBiomes => String::from("largeBiomes"),
            Self::Amplified => String::from("amplified"),
            Self::Customized => String::from("customized"),
            Self::Buffet => String::from("buffet"),
            Self::Default11 => String::from("default_1_1"),
        }
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Nbt<'a, T> {
    /// Returns a new NBT type.
    pub fn new(t: T) -> Self {
        Self(PhantomData, t)
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Serialize for Nbt<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = vec![];

        to_writer(&mut bytes, &self.1, None).unwrap();

        bytes.serialize(serializer)
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Deserialize<'de> for Nbt<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(NbtVisitor(PhantomData))
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Visitor<'de> for NbtVisitor<'a, T> {
    type Value = Nbt<'a, T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected seq")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut decoder = Decoder::new(SeqReader(PhantomData, seq));
        let value = T::deserialize(&mut decoder).unwrap();

        Ok(Nbt(PhantomData, value))
    }
}

impl<'a, 'de: 'a, T: Into<R>, R: Serialize + Deserialize<'de>> From<Vec<T>>
    for LengthPrefixedVec<'a, R>
{
    fn from(v: Vec<T>) -> Self {
        Self::new(v.into_iter().map(|v| v.into()).collect())
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> LengthPrefixedVec<'a, T> {
    /// Returns a new LengthPrefixedVec type.
    pub fn new(t: Vec<T>) -> Self {
        Self(PhantomData, t)
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Serialize for LengthPrefixedVec<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = vec![];
        write_varint(self.1.len() as i32, &mut bytes).unwrap();
        let mut seq = serializer.serialize_seq(Some(self.1.len())).unwrap();

        seq.serialize_element(&bytes)?;
        seq.serialize_element(&self.1)?;

        seq.end()
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Deserialize<'de> for LengthPrefixedVec<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(LengthPrefixedVecVisitor(PhantomData))
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Visitor<'de>
    for LengthPrefixedVecVisitor<'a, T>
{
    type Value = LengthPrefixedVec<'a, T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let reader = SeqReader(PhantomData, &mut seq);
        let len = read_varint(reader).unwrap();
        let mut data = Vec::with_capacity(len as usize);

        for _ in 0..len {
            data.push(seq.next_element()?.unwrap());
        }

        Ok(LengthPrefixedVec(PhantomData, data))
    }
}

struct SeqReader<'de, A: SeqAccess<'de>>(PhantomData<&'de ()>, pub A);

impl<'de, A: SeqAccess<'de>> Read for SeqReader<'de, A> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for b in buf.iter_mut() {
            *b = self
                .1
                .next_element()
                .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?
                .ok_or(Error::new(ErrorKind::UnexpectedEof, ""))?;
        }

        Ok(buf.len())
    }
}
