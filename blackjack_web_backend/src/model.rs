use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    rank: String,
    suit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    deck: Vec<Card>,
    message: String,
}

impl GameState {
    pub fn new() -> Self {
        let initial_deck = get_initial_deck();
        GameState {
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            deck: initial_deck,
            message: "".to_string(),
        }
    }
    pub fn first_deal(&mut self) {
        let initial_deck = get_initial_deck();
        self.player_hand = Vec::new();
        self.dealer_hand = Vec::new();
        self.deck = initial_deck;
        self.message = "".to_string();
        // 游戏初始化：player 和 dealer 各发两张手牌
        let (player_cards, remaining_deck) = get_random_cards(self.deck.clone(), 2);
        let (dealer_cards, new_deck) = get_random_cards(remaining_deck, 2);
        self.player_hand = player_cards;
        self.dealer_hand = dealer_cards;
        // 更新剩余手牌状态
        self.deck = new_deck;
    }
}

pub fn get_initial_deck() -> Vec<Card> {
    let ranks = vec![
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];
    let suits = vec!["♥️", "♠️", "♣️", "♦️"];
    // let initial_deck : Vec<Card> = ranks.iter().flat_map(| rank | suits.iter().map( move | suit | Card { rank: rank.to_string(), suit: suit.to_string()})).collect();
    let initial_deck: Vec<Card> = ranks
        .iter()
        .flat_map(|&rank| {
            suits.iter().map(move |&suit| Card {
                rank: rank.to_string(),
                suit: suit.to_string(),
            })
        })
        .collect();
    initial_deck
}

// 获取随机手牌并更新剩余牌堆
pub fn get_random_cards(deck: Vec<Card>, count: i32) -> (Vec<Card>, Vec<Card>) {
    let mut random_index_set = HashSet::new();
    while random_index_set.len() < count as usize {
        match generate_random_by_length(deck.len() as u32) {
            Ok(random_index) => {
                random_index_set.insert(random_index);
            }
            Err(_) => {
                break;
            }
        }
    }
    let random_cards = deck
        .iter()
        .enumerate()
        .filter(|(index, _)| random_index_set.contains(&(*index as u32)))
        .map(|(_, card)| card.clone())
        .collect();
    let remaining_cards = deck
        .iter()
        .enumerate()
        .filter(|(index, _)| !random_index_set.contains(&(*index as u32)))
        .map(|(_, card)| card.clone())
        .collect();
    (random_cards, remaining_cards)
}

fn generate_random_by_length(length: u32) -> Result<u32, Box<dyn Error>> {
    if length == 0 {
        return Err("长度不能为0".into());
    }

    // 生成范围内的随机数
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..=length);

    Ok(random_num)
}
