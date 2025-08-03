pub fn calc_physical_dmg(attacker_atk: u8, defender_def: u8, base_damage: u8) -> u8 {
    let damage = base_damage as i16 + attacker_atk as i16 - defender_def as i16;
    damage.max(0) as u8
}

pub fn calc_magical_dmg(attacker_int: u8, defender_res: u8, base_damage: u8) -> u8 {
    let damage = base_damage as i16 + attacker_int as i16 - defender_res as i16;
    damage.max(0) as u8
}

pub fn calc_hit_chance(attacker_dex: u8, defender_dex: u8) -> f32 {
    let chance = attacker_dex as f32 / (attacker_dex as f32 + defender_dex as f32);
    chance
}
