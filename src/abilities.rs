use crate::Stat;

pub enum Ability {
    Damage {
        name: &'static str,
        damage: u8,
        effect: Option<StatusEffect>,
        damage_type: DamageType,
    },
    Status {
        name: &'static str,
        effects: Vec<StatusEffect>,
    },
    Healing {
        name: &'static str,
        amount: u8,
    },
    Shielding {
        name: &'static str,
        amount: u8,
        duration: u8,
    },
    Buff {
        name: &'static str,
        target: Stat,
        amount: u8,
    },
}

pub enum StatusEffect {
    DoT { duration: u8, power: u8 },
    Slow { duration: u8 },
    Stun { duration: u8 },
    Freeze { duration: u8 },
    Drain,
    Vulnerability { duration: u8, power: u8 },
}

pub enum DamageType {
    Physical,
    Magic,
}

// Knight Abilities

pub fn slash() -> Ability {
    Ability::Damage {
        name: "Slash",
        damage: 10,
        effect: None,
        damage_type: DamageType::Physical,
    }
}

pub fn shield_bash() -> Ability {
    Ability::Damage {
        name: "Shield Bash",
        damage: 5,
        effect: Some(StatusEffect::Stun { duration: 2 }),
        damage_type: DamageType::Physical,
    }
}

pub fn block() -> Ability {
    Ability::Shielding {
        name: "Block",
        amount: 10,
        duration: 3,
    }
}

// Mage Abilities

pub fn fireball() -> Ability {
    Ability::Damage {
        name: "Fireball",
        damage: 10,
        effect: Some(StatusEffect::DoT {
            duration: 2,
            power: 1,
        }),
        damage_type: DamageType::Magic,
    }
}

pub fn ice_spike() -> Ability {
    Ability::Damage {
        name: "Ice Spike",
        damage: 5,
        effect: Some(StatusEffect::Freeze { duration: 1 }),
        damage_type: DamageType::Magic,
    }
}

pub fn mana_shield() -> Ability {
    Ability::Shielding {
        name: "Mana Shield",
        amount: 10,
        duration: 3,
    }
}

// Archer Abilities

pub fn arrow_shot() -> Ability {
    Ability::Damage {
        name: "Arrow Shot",
        damage: 10,
        effect: None,
        damage_type: DamageType::Physical,
    }
}

pub fn caltrops() -> Ability {
    Ability::Status {
        name: "Caltrops",
        effects: vec![
            StatusEffect::DoT {
                duration: 2,
                power: 2,
            },
            StatusEffect::Slow { duration: 2 },
        ],
    }
}

pub fn dodge_stance() -> Ability {
    Ability::Buff {
        name: "Dodge Stance",
        target: Stat::Dex,
        amount: 10,
    }
}

// Rogue Abilities

pub fn stab() -> Ability {
    Ability::Damage {
        name: "Stab",
        damage: 5,
        effect: Some(StatusEffect::Vulnerability {
            duration: 3,
            power: 3,
        }),
        damage_type: DamageType::Physical,
    }
}

pub fn poison_barb() -> Ability {
    Ability::Damage {
        name: "Poison Barb",
        damage: 5,
        effect: Some(StatusEffect::DoT {
            duration: 2,
            power: 4,
        }),
        damage_type: DamageType::Physical,
    }
}

pub fn smokescreen() -> Ability {
    Ability::Buff {
        name: "Smokescreen",
        target: Stat::Dex,
        amount: 20,
    }
}
