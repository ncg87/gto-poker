use std::io;
use std::collections::VecDeque;
use poker::players::player::{PlayerType, HumanPlayer};
use poker::players::base::PlayerAction;
use poker::players::action::Action;

fn main() {
    println!("Hello, world! Enter the number of players: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    
    let num_players = input.trim().parse::<u32>().expect("Please enter a valid number!");


    // Print the number of players
    println!("Let initialize a {} player game of Texas Hold'em!", num_players);


    // Create a vector of players
    let mut players: Vec<PlayerType> = Vec::new();
    for i in 0..num_players {
        input.clear();
        // Get the name of the player
        println!("Enter the name of player {}: ", i + 1);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let name = input.trim().to_string(); // Convert the input to a string so it isn't a reference

        // Get the buy in for the player
        input.clear();
        let chips: u32;
        println!("Enter the buy in for {}: ", name);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        chips = input.trim().parse::<u32>().expect("Please enter a valid number!");

        // Create a new human player
        let player = PlayerType::new_human_player(name.trim().to_string(), chips);
        players.push(player);
    }

    let mut queue = VecDeque::from(players);
    let mut num_players_in_round = queue.len();
    // Initialize the current bet
    let mut current_bet = 0;
    // Flag to check if the round is complete
    let mut round_complete = false;
    // Main game loop
    while !round_complete {
        if let Some(mut player) = queue.pop_front() {

            if player.get_chips() == 0 {
                round_complete = true;
            }

            let action = player.get_action::<HumanPlayer>(current_bet);
            match action.action() {
                Action::Check => {
                    println!("{} checks", player.get_name());
                    queue.push_back(player);
                }
                Action::Fold => {
                    println!("{} folds", player.get_name());
                }
                Action::Call => {
                    println!("{} calls for {}", player.get_name(), current_bet);
                    queue.push_back(player);
                }
                Action::Raise(amount) => {
                    current_bet = amount;
                    println!("{} raises from {} to {}", player.get_name(), amount - current_bet, amount);
                    queue.push_back(player);
                }
                Action::AllIn(amount) => {
                    current_bet = amount;
                    println!("{} goes all in to {}", player.get_name(), current_bet);
                    queue.push_back(player);
                }
            }
            num_players_in_round -= 1;
            if num_players_in_round == 0 {
                round_complete = true;
            }
        }
    }



}