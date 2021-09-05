use serde::{Deserialize, Serialize};

use crate::packets::types::{self, BiomeRegistry};

///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_registry: DimensionRegistry,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome_registry: BiomeRegistry,
}

///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionRegistry {
    #[serde(rename = "type")]
    pub dimension_type: String,
    pub value: Vec<DimensionRegistryEntry>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: DimensionType,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
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

impl From<types::DimensionCodec> for DimensionCodec {
    fn from(codec: types::DimensionCodec) -> Self {
        Self {
            dimension_registry: codec.dimension_registry.into(),
            biome_registry: codec.biome_registry,
        }
    }
}

impl From<types::DimensionRegistry> for DimensionRegistry {
    fn from(registry: types::DimensionRegistry) -> Self {
        Self {
            dimension_type: registry.dimension_type,
            value: registry.value.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<types::DimensionRegistryEntry> for DimensionRegistryEntry {
    fn from(entry: types::DimensionRegistryEntry) -> Self {
        Self {
            name: entry.name,
            id: entry.id,
            element: entry.element.into(),
        }
    }
}

impl From<types::DimensionType> for DimensionType {
    fn from(ty: types::DimensionType) -> Self {
        Self {
            piglin_safe: ty.piglin_safe,
            natural: ty.natural,
            ambient_light: ty.ambient_light,
            fixed_time: ty.fixed_time,
            infiniburn: ty.infiniburn,
            respawn_anchor_works: ty.respawn_anchor_works,
            has_skylight: ty.has_skylight,
            bed_works: ty.bed_works,
            effects: ty.effects,
            has_raids: ty.has_raids,
            min_y: ty.min_y,
            height: ty.height,
            logical_height: ty.logical_height,
            coordinate_scale: ty.coordinate_scale,
            ultrawarm: ty.ultrawarm,
            has_ceiling: ty.has_ceiling,
        }
    }
}
