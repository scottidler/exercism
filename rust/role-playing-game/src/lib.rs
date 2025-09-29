pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {

    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Self {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut damage = 0;
        if let Some(mana) = self.mana {
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);
                damage = mana_cost * 2
            }
        } else {
            self.health = self.health.saturating_sub(mana_cost);
        }
        damage
    }
}
