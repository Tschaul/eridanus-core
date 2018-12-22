use crate::model::base_types::Amount;
use crate::model::player::PlayerToken;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct WorldKey(u8);

impl WorldKey {
    pub fn new(key_value: u8) -> Self {
        WorldKey(key_value)
    }
}

impl std::fmt::Display for WorldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct World {
    pub owner: Option<PlayerToken>,
    pub industry: Amount,
    pub metal: Amount,
    pub mines: Amount,
    pub population: Amount,
    pub limit: Amount,
    pub turns: Amount,
    pub i_ships: Amount,
    pub p_ships: Amount,
}