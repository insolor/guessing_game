use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Введите ваш вариант: ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Нужно ввести число!");
    
        println!("Вы ввели: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Вы угадали!");
                break;
            }
        }
    }
}
