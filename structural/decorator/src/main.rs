use decorator::*;

fn main() {
    let beverage = Espresso; // stack allocation
    println!("Espresso: {}", beverage.cost());

    let mut mocha = Mocha::new(Box::new(beverage)); // heap allocation
    mocha = Mocha::new(Box::new(mocha));
    println!("Espresso with 2 mocha: {}", mocha.cost());

    let whip = Whip::new(Box::new(mocha));
    println!("Espresso with 2 mocha and 1 whip: {}", whip.cost());

}
