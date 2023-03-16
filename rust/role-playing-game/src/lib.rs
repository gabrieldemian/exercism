use std::cmp::{max, min};

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        };
        Some(Player {
            health: 100,
            level: self.level,
            mana: if self.level >= 10 { Some(100) } else { None },
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(max(mana - mana_cost, 0));
                    return mana_cost * 2;
                };
                0
            }
            None => {
                self.health = self.health - min(self.health, mana_cost);
                0
            }
        }
    }
}
