use std::io;
use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("I'm thinking of a fake five-letter word... guess it!");

    // Generate five letters using the Uniform distribution and the local generator
    let lowerchars = Uniform::from('a'..'z');
    let mut rng = rand::thread_rng();
    let mut blurd = String::new();
    
    for _ in 0..5 {
        blurd.push(lowerchars.sample(&mut rng));
    }

    //println!("The secret word is: {}", blurd);
    let mut guess_count = 0;

    loop {
        let mut guess = String::new();
        let mut output = String::new();

        // Read their guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        if guess.len() != 6 {
            println!("Error, please enter a five-letter word.");
            continue
        }
        
        guess_count += 1;
        guess.pop();
        guess.make_ascii_lowercase();
        if guess.eq(&blurd) {
            println!("You actually won! It took you {} guesses.", guess_count);
            break
        }
        
        for (i, ch) in guess.chars().enumerate() {
            if ch == blurd.chars().nth(i).unwrap() {
                output.push('#');   // Right character, right spot
            } else if blurd.contains(ch) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        
        println!("{}", output);
    }
}
