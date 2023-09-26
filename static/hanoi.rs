fn hanoi(n: i32, source: char, auxiliary: char, target: char) {
    if n == 1 {
        println!("Move disk 1 from {} to {}", source, target);
        return;
    }

    hanoi(n - 1, source, target, auxiliary);
    println!("Move disk {} from {} to {}", n, source, target);
    hanoi(n - 1, auxiliary, source, target);
}