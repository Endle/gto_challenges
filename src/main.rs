
#[derive(Debug, PartialEq, PartialOrd)]
enum PokerCard {


    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}


fn main() {
    println!("Hello, world!");

    let a = PokerCard::Ace;
    let fiv = PokerCard::Five;

    assert!(a > fiv);

    println!("{:?}", &a);
}
