# Guessing_Game in <ins>*RUST*</ins>

This is a simple console-based number guessing game written in Rust. The objective of the game is to guess a randomly generated number between 1 and 100 in as few guesses as possible. The program provides feedback on each guess, telling you whether your guess is too high or too low. When you guess correctly, the game ends and displays the number of attempts it took to win.

## **Features**

A randomly generated secret number between 1 and 100.
Input validation: The program ensures that only valid numeric guesses are accepted.
Feedback on each guess: The game tells you whether your guess is too high or too low.
Tracks the number of attempts it takes to guess the secret number.
Prerequisites
Before running this code, ensure you have the following installed:

**Rust**: A systems programming language.
**Cargo**: Rust's package manager and build tool.
*Installation*
Clone the repository:
```
git clone https://github.com/yourusername/guessing-game.git
cd guessing-game
```
Build and run the project:
```
cargo run
```

## **How to Play**

When you run the program, it will prompt you to "Guess a number!".
Type a number between 1 and 100 and hit Enter.
The program will tell you if your guess is too high, too low, or correct.
Keep guessing until you correctly guess the number. The program will show you how many attempts it took.

Example Output
```
Guess a number!
Please type your guess: 
50
Too Big
Please type your guess: 
30
Too Small
Please type your guess: 
40
You win!
Counts required to WIN: 3
```

## **Code Explanation**

Random number generation: The rand::Rng trait is used to generate a random number between 1 and 100.
Input handling: The user is prompted to enter a guess, and the input is validated to ensure it's a number.
Feedback: The program compares the guess to the secret number and gives feedback whether the guess is too high, too low, or correct.
License
This project is licensed under the MIT License - see the LICENSE file for details.

## *Contributions*

Feel free to fork the repository, make changes, and submit pull requests. Issues and suggestions are also welcome.

