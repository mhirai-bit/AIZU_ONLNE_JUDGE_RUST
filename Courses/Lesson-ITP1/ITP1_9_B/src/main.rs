fn main() {
    let mut result = vec![];
    loop {
        let mut deck = String::new();
        std::io::stdin().read_line(&mut deck).unwrap();
        let mut deck = deck.trim().to_string();
        if deck == "-" {
            break;
        }
        let mut num_of_shuffle = String::new();
        std::io::stdin().read_line(&mut num_of_shuffle).unwrap();
        let num_of_shuffle = num_of_shuffle.trim().parse::<u64>().unwrap();
        // println!("**num of shuffle = {}**", num_of_shuffle);
        for i in 0..num_of_shuffle {
            // println!("{} times",i+1);
            //read the number of cards to shuffle
            let mut num_of_cards = String::new();
            std::io::stdin().read_line(&mut num_of_cards).unwrap();
            let num_of_cards = num_of_cards.trim().parse::<usize>().unwrap();
            // println!("**num of cards = {}**", num_of_cards);
            //identify the cards to move
            let mut copy_deck = deck.clone();
            let cards_to_move = copy_deck.get(..num_of_cards).unwrap();
            // println!("**cards to move = {}**", cards_to_move);
            // println!("**deck before trim left = {}**", deck);
            deck = deck.get(num_of_cards..).unwrap().to_string();

            // println!("**deck after trim left = {}**", deck);
            deck.push_str(cards_to_move);
            // println!("**deck after push = {}**", deck);
        }
        result.push(deck);
    }
    for deck in result {
        println!("{}", deck);
    }
}
