use uuid::Uuid;

use crate::{
    macros::{int_enum, packets},
    play::entity_type::EntityType,
    types::{varint::VarInt, Angle},
};

packets! {
    SpawnEntity(0x00) {
        entity_id: VarInt,
        uuid: Uuid,
        ty: EntityType,
        x: f64,
        y: f64,
        z: f64,
        pitch: Angle,
        yaw: Angle,
        head_yaw: Angle,
        // TODO: Add Enum here?
        // (see https://wiki.vg/Object_Data)
        data: VarInt,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    };
    SpawnExperienceOrb(0x01) {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        amount: i16
    };
    SpawnPlayer(0x02) {
        entity_id: VarInt,
        uuid: Uuid,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle
    };
    EntityAnimation(0x03) {
        entity_id: VarInt,
        animation: EntityAnimationId
    }
}

int_enum! {
    EntityAnimationId(u8) {
        SwingMainArm = 0,
        TakeDamage = 1,
        LeaveBed = 2,
        SwingOffhand = 3,
        CriticalEffect = 4,
        MagicCriticalEffect = 5
    }
}
