use crate::Stat;

pub enum PlayerAbility {
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

pub fn slash() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Slash",
        damage: 10,
        effect: None,
        damage_type: DamageType::Physical,
    }
}

pub fn shield_bash() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Shield Bash",
        damage: 5,
        effect: Some(StatusEffect::Stun { duration: 2 }),
        damage_type: DamageType::Physical,
    }
}

pub fn block() -> PlayerAbility {
    PlayerAbility::Shielding {
        name: "Block",
        amount: 10,
        duration: 3,
    }
}

// Mage Abilities

pub fn fireball() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Fireball",
        damage: 10,
        effect: Some(StatusEffect::DoT {
            duration: 2,
            power: 1,
        }),
        damage_type: DamageType::Magic,
    }
}

pub fn ice_spike() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Ice Spike",
        damage: 5,
        effect: Some(StatusEffect::Freeze { duration: 1 }),
        damage_type: DamageType::Magic,
    }
}

pub fn mana_shield() -> PlayerAbility {
    PlayerAbility::Shielding {
        name: "Mana Shield",
        amount: 10,
        duration: 3,
    }
}

// Archer Abilities

pub fn arrow_shot() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Arrow Shot",
        damage: 10,
        effect: None,
        damage_type: DamageType::Physical,
    }
}

pub fn caltrops() -> PlayerAbility {
    PlayerAbility::Status {
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

pub fn dodge_stance() -> PlayerAbility {
    PlayerAbility::Buff {
        name: "Dodge Stance",
        target: Stat::Dex,
        amount: 10,
    }
}

// Rogue Abilities

pub fn stab() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Stab",
        damage: 5,
        effect: Some(StatusEffect::Vulnerability {
            duration: 3,
            power: 3,
        }),
        damage_type: DamageType::Physical,
    }
}

pub fn poison_barb() -> PlayerAbility {
    PlayerAbility::Damage {
        name: "Poison Barb",
        damage: 5,
        effect: Some(StatusEffect::DoT {
            duration: 2,
            power: 4,
        }),
        damage_type: DamageType::Physical,
    }
}

pub fn smokescreen() -> PlayerAbility {
    PlayerAbility::Buff {
        name: "Smokescreen",
        target: Stat::Dex,
        amount: 20,
    }
}
