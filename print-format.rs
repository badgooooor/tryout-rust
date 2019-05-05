fn main() {
    println!("{}, {}!", "Hello", "world");
    println!("{1}, {0}!", "Hello", "world");
    println!("{greeting}, {name}!", greeting="Kuy rai sus", name="Bier");

    println!("{:?}", [1,2,3]);
    println!("{:#?}", [1,2,3]);

    let x = format!("{}, {}", "Aw shit", "here we go again");
    println!("{}", x);

    let y = String::from("Sup ") + "douchebag";
    println!("{}", y);
}