#[test]

/*
/* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let v1 = vec![1, 2];

    assert_eq!(v1.next(), __);
    assert_eq!(v1.next(), __);
    assert_eq!(v1.next(), __);
}
*/

//1
/*fn main() {
    let v1 = vec![1, 2];
    let mut iter = v1.iter(); // Створення ітератора

    assert_eq!(iter.next(), Some(&1)); // Перший елемент: 1
    assert_eq!(iter.next(), Some(&2)); // Другий елемент: 2
    assert_eq!(iter.next(), None);     // Всі елементи витягнуті, тому None
}*/

//2
fn main() {
    let v1 = vec![1, 2];

    let mut iter = v1.into_iter(); // Створення ітератора для переміщення елементів

    assert_eq!(iter.next(), Some(1)); // Перший елемент: 1
    assert_eq!(iter.next(), Some(2)); // Другий елемент: 2
    assert_eq!(iter.next(), None);    // Всі елементи витягнуті, тому None
}


/*
//1
v1.iter() створює ітератор по елементах вектора v1, який повертає елементи за посиланням.
next() повертає Some(&1) для першого елемента, Some(&2) для другого, і None, коли елементи закінчуються.
//2
v1.into_iter() створює ітератор, який переміщує елементи з вектора. У цьому випадку ви не працюєте з посиланнями на елементи, а безпосередньо з їх значеннями.
next() повертає Some(1), Some(2), і None після витягування всіх елементів

*/