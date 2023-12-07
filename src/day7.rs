use std::{cmp::Ordering, collections::HashMap, fs};

struct Card {
    label: char,
    p1: bool,
}

impl Card {
    fn get_value(&self) -> u8 {
        match self.label {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if self.p1 {
                    11
                } else {
                    1
                }
            }
            'T' => 10,
            _ => self.label.to_digit(10).unwrap().try_into().unwrap(),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(&other.get_value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
    p1: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            Ordering::Equal => {} //same hand, cmp inv cards,
            o => return o,
        }

        self.cards.iter().cmp(other.cards.iter())
    }
}

impl Hand {
    fn group_cards(&self) -> HashMap<char, u8> {
        let mut gp: HashMap<char, u8> = HashMap::new();
        self.cards.iter().map(|c| c.label).for_each(|c| {
            if !gp.contains_key(&c) {
                gp.insert(c, 1);
            } else {
                gp.insert(c, gp.get(&c).unwrap() + 1);
            }
        });
        gp
    }

    fn get_card_type_counts(&self) -> u8 {
        let gp = self.group_cards();
        let has_j = gp.contains_key(&'J');
        let true_count = gp.into_iter().count().try_into().unwrap();
        if self.p1 || true_count == 1 {
            return true_count;
        }
        //p2, handle J
        if has_j {
            return true_count - 1;
        }
        true_count
    }

    fn get_high_card_type_count(&self) -> u8 {
        let gp = self.group_cards();
        let j_count: u8 = if !gp.contains_key(&'J') {
            0
        } else {
            *gp.get(&'J').unwrap()
        };
        let max = *gp.iter().map(|g| g.1).max().unwrap();
        if self.p1 || j_count == 0 || j_count == 5 {
            return max;
        }
        let non_j_max = *gp
            .iter()
            .filter(|g| *g.0 != 'J')
            .map(|g| g.1)
            .max()
            .unwrap();
        non_j_max + j_count
    }

    fn get_hand_type(&self) -> u8 {
        let type_count = self.get_card_type_counts();
        let highest_count = self.get_high_card_type_count();
        match (type_count, highest_count) {
            (1, 5) => 7,
            (2, 4) => 6,
            (2, 3) => 5,
            (3, 3) => 4,
            (3, 2) => 3,
            (4, 2) => 2,
            (5, 1) => 1,
            _ => 0,
        }
    }
}

fn load_hands(file: &str, p1: bool) -> Vec<Hand> {
    let data = fs::read_to_string(file).expect("Cannot open file");
    let mut hands: Vec<Hand> = Vec::new();
    for line in data.split('\n') {
        if line.len() < 2 {
            continue;
        }
        let l: Vec<&str> = line.split_whitespace().collect();
        let hand = l[0];
        let bid = l[1].parse::<u32>().unwrap();

        let mut cards = Vec::new();
        for c in hand.chars() {
            cards.push(Card { label: c, p1 })
        }
        hands.push(Hand { cards, bid, p1 });
    }
    hands
}

fn main() {
    let file = "data/day7.txt";
    let mut hands: Vec<Hand> = load_hands(file, true);
    let mut p1_result: u32 = 0;
    hands.sort();
    let mut rank = 1;
    for h in hands {
        p1_result += rank * h.bid;
        rank += 1;
    }
    println!("p1: {}", p1_result);

    let mut hands: Vec<Hand> = load_hands(file, false);
    let mut p2_result: u32 = 0;
    hands.sort();
    let mut rank = 1;
    for h in hands {
        p2_result += rank * h.bid;
        rank += 1;
    }
    println!("p2: {}", p2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card() {
        let card = Card {
            label: 'A',
            p1: true,
        };
        assert_eq!(card.get_value(), 14);
        let card = Card {
            label: 'K',
            p1: true,
        };
        assert_eq!(card.get_value(), 13);
        let card = Card {
            label: 'Q',
            p1: true,
        };
        assert_eq!(card.get_value(), 12);
        let card = Card {
            label: 'J',
            p1: true,
        };
        assert_eq!(card.get_value(), 11);
        let card = Card {
            label: 'T',
            p1: true,
        };
        assert_eq!(card.get_value(), 10);
        let card = Card {
            label: '9',
            p1: true,
        };
        assert_eq!(card.get_value(), 9);
        let card = Card {
            label: '1',
            p1: true,
        };
        assert_eq!(card.get_value(), 1);
    }

    #[test]
    fn test_hand() {
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 7);
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 6);
        let hand = Hand {
            cards: vec![
                Card {
                    label: '2',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '2',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 5);
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'T',
                    p1: true,
                },
                Card {
                    label: 'T',
                    p1: true,
                },
                Card {
                    label: 'T',
                    p1: true,
                },
                Card {
                    label: '9',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 4);
        let hand = Hand {
            cards: vec![
                Card {
                    label: '2',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '4',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '2',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 3);
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: '2',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: '4',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 2);
        let hand = Hand {
            cards: vec![
                Card {
                    label: '2',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '4',
                    p1: true,
                },
                Card {
                    label: '5',
                    p1: true,
                },
                Card {
                    label: '6',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };
        assert_eq!(hand.get_hand_type(), 1);

        //p2
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'T',
                    p1: false,
                },
                Card {
                    label: '5',
                    p1: false,
                },
                Card {
                    label: '5',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: '5',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };
        assert_eq!(hand.get_hand_type(), 6);
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'K',
                    p1: false,
                },
                Card {
                    label: 'T',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'T',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };
        assert_eq!(hand.get_hand_type(), 6);
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'A',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };
        assert_eq!(hand.get_hand_type(), 6);
        let hand = Hand {
            cards: vec![
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };
        assert_eq!(hand.get_hand_type(), 7);
    }

    #[test]
    fn test_cmp_hands() {
        let hand1 = Hand {
            cards: vec![
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '3',
                    p1: true,
                },
                Card {
                    label: '2',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };

        let hand2 = Hand {
            cards: vec![
                Card {
                    label: '2',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
                Card {
                    label: 'A',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };

        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
        let hand1 = Hand {
            cards: vec![
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };

        let hand2 = Hand {
            cards: vec![
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };

        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
        let hand1 = Hand {
            cards: vec![
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '7',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
                Card {
                    label: '8',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };

        let hand2 = Hand {
            cards: vec![
                Card {
                    label: '1',
                    p1: true,
                },
                Card {
                    label: '1',
                    p1: true,
                },
                Card {
                    label: '1',
                    p1: true,
                },
                Card {
                    label: '1',
                    p1: true,
                },
                Card {
                    label: '1',
                    p1: true,
                },
            ],
            bid: 99,
            p1: true,
        };

        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
    }

    #[test]
    fn test_cmp_hands_p2() {
        let hand1 = Hand {
            cards: vec![
                Card {
                    label: 'T',
                    p1: false,
                },
                Card {
                    label: '5',
                    p1: false,
                },
                Card {
                    label: '5',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: '5',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };
        let hand2 = Hand {
            cards: vec![
                Card {
                    label: 'K',
                    p1: false,
                },
                Card {
                    label: 'T',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'T',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };

        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
        let hand1 = Hand {
            cards: vec![
                Card {
                    label: 'J',
                    p1: false,
                },
                Card {
                    label: 'K',
                    p1: false,
                },
                Card {
                    label: 'K',
                    p1: false,
                },
                Card {
                    label: 'K',
                    p1: false,
                },
                Card {
                    label: '2',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };
        let hand2 = Hand {
            cards: vec![
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: 'Q',
                    p1: false,
                },
                Card {
                    label: '2',
                    p1: false,
                },
            ],
            bid: 99,
            p1: false,
        };

        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
    }
}
