use std::collections::HashMap;

pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    let mut hp: HashMap<i32, i32> = HashMap::new();
    if hand.len() as i32 % group_size != 0 {
        return false;
    }
    for i in &hand {
        // hp.insert(*i, *hp.get(&i).unwrap_or(&0) + 1);
        *hp.entry(*i).or_insert(0) += 1;
    }
    for card in &hand {
        let mut start_card = *card;
        while *hp.get(&(start_card - 1)).unwrap_or(&0) > 0 {
            start_card -= 1
        }
        while start_card < *card {
            while *hp.get(&start_card).unwrap_or(&0) > 0 {
                for i in start_card..(start_card + group_size) {
                    if *hp.get(&i).unwrap_or(&0) == 0 {
                        return false;
                    }
                    hp.insert(i, *hp.get(&i).unwrap_or(&0) - 1);
                }
            }
            start_card += 1;
        }
    }
    true
}
