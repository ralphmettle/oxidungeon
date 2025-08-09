use crate::{enemy::{self, EnemyType}, Class};

pub struct Stats {
    hp: i16,
    atk: i16,
    def: i16,
    int: i16,
    res: i16,
    dex: i16,
    spd: i16,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            hp: 25,
            atk: 15,
            def: 15,
            int: 15,
            res: 15,
            dex: 15,
            spd: 15,
        }
    }

    pub fn from_class(class: &Class) -> Self {
        match class {
            Class::Knight => Stats {
                hp: 25,
                atk: 20,
                def: 25,
                int: 5,
                res: 15,
                dex: 10,
                spd: 15,
            },
            Class::Mage => Stats {
                hp: 20,
                atk: 5,
                def: 15,
                int: 25,
                res: 20,
                dex: 10,
                spd: 15,
            },
            Class::Archer => Stats {
                hp: 15,
                atk: 25,
                def: 15,
                int: 5,
                res: 15,
                dex: 20,
                spd: 20,
            },
            Class::Rogue => Stats {
                hp: 15,
                atk: 20,
                def: 15,
                int: 5,
                res: 15,
                dex: 25,
                spd: 20,
            },
        }
    }

    pub fn from_enemy_type(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Goblin => Stats {
                hp: 15,
                atk: 15,
                def: 10,
                int: 0,
                res: 5,
                dex: 10,
                spd: 20,
            },
        }
    }
}
