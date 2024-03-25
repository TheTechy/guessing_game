// Guessing Game
// 1. Generate a number between 0 & 100
// 2. User has 15 guess attempts to try and guess the number
// 3. On start, clear the screen and display the number of guesses left with instrutions to guess
// 4. On each guess, clear the screen and display a hint if the guess was too high or too low and guesses left
// 5. On correct guess, clear the screen and display win message

//** Includes */
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Instant;

// Struct to hold game state
struct GameState {
	max_guesses:	i32,
	guess_hint:		String,
	secret_number:	u8
}

//** Struct to hold player state */
struct PlayerState {
	player_name: 	String,
	guess_attempts: i32
}

//** Clears the termnal */
fn clr() {
	// clear the screen
	std::process::Command::new("clear").status().unwrap();
}

//** Draws th game screen */
fn ui_screen(player: &PlayerState, guess_made: u8, guess_hint: &str, max_guesses: &i32) {
	// Clear console
	clr();

	//** Print the UI */
	println!("*****************\nGuess the number!\n*****************\nThe number is 0 -  100\n");
	println!("{}You have {} guesses left", player.player_name, (max_guesses - player.guess_attempts));
	if guess_hint.len() > 0 {
		println!("Your last guess was {} and was {}", guess_made, guess_hint);
	}
}

fn main() {
	// Instantiate a player
	let mut player1 = PlayerState {
		player_name: 	"".to_string(),
		guess_attempts:	0
	};

	let mut game = GameState {
		max_guesses:	15,
		guess_hint:		"".to_string(),
		secret_number:	rand::thread_rng().gen_range(1..=100)
	};

	let start_timer = Instant::now();

	clr();
	println!("Enter your name:");
	io::stdin()
	.read_line(&mut player1.player_name)
	.expect("Failed to read line");

	ui_screen(&player1, 0, &game.guess_hint, &game.max_guesses);

	loop {
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u8 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&game.secret_number) {
			Ordering::Less => {
				if (player1.guess_attempts + 1) < 15 {
					game.guess_hint = "too small!".to_string();
					player1.guess_attempts = player1.guess_attempts + 1;
				} else {
					clr();
					println!("Too many attempts\nYou lose :(\nThe number was: {}", game.secret_number);
					break;
				}
			},
			Ordering::Greater => {
				if (player1.guess_attempts + 1) < 15 {
					game.guess_hint = "too big!".to_string();
					player1.guess_attempts = player1.guess_attempts + 1;
				} else {
					clr();
					println!("Too many attempts\nYou lose :(\nThe number was: {}", game.secret_number);
					break;
				}
			},
			Ordering::Equal => {
				let end_timer = start_timer.elapsed();
				clr();
				println!("You win!");
				println!("It took you {} attempts and {:.2?} seconds to guess.", (player1.guess_attempts+1), end_timer);
				break;
			}
		}
		ui_screen(&player1, guess, &game.guess_hint, &game.max_guesses);
	}
}