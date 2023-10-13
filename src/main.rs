mod game_state;
mod gui;

// Importing game_state module
use game_state::GameState;
// Importing app
use gui::App;


fn main() {
    // let base_matrix = vec![
    //     vec![false, false, false, false, false, false, false, false, false, false],
    //     vec![false, false, false, false, true, false, false, false, false, false],
    //     vec![false, false, false, false, false, false, false, false, false, false],
    //     vec![false, false, false, false, true, false, false, false, false, false],
    //     vec![false, false, false, false, true, false, false, false, false, false],
    //     vec![false, false, false, true, true, true, false, false, false, false],
    //     vec![false, false, false, false, false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false, false, false, false, false],
    //     vec![false, false, false, false, false, false, false, false, false, false],
    // ];

    // let mut game_state = GameState::from(base_matrix);
    
    let mut game_state = GameState::new_random(69, 125);

    let mut app = App::new(game_state);
    app.run();

    // Glider 
    // let mut game_state = GameState::new(50, 150);
    // game_state.matrix[0][2] = true;
    // game_state.matrix[1][0] = true;
    // game_state.matrix[1][2] = true;
    // game_state.matrix[2][1] = true;
    // game_state.matrix[2][1] = true;
    // game_state.matrix[2][2] = true;
    
    // loop {
    //     print!("{}[2J", 27 as char);
    //     println!("{}", game_state);
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    //     game_state.forward_one_step();
    // }
}


