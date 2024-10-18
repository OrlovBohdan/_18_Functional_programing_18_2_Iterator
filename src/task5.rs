#[test]

/*
/* Fill in the blank */
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.__{
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
*/

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match *name {
            "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        };
    }

    println!("names: {:?}", names);
}


/*
iter_mut() дозволяє отримати мутабельні посилання на елементи вектора. Тому можно змінювати елементи вектора безпосередньо в циклі.
match *name: За допомогою оператора розіменування (*name), ми отримуємо значення елемента, яке є рядком,
і порівнюємо його з "Ferris". Якщо ім'я "Ferris", ми заміняємо його на "There is a rustacean among us!", а в іншому випадку — на "Hello".
println!("names: {:?}", names); виводить змінений вектор.
*/