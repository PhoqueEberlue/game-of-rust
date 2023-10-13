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
    
    // Glider 
    // let mut game_state = GameState::new(69, 120);
    // let matrix = game_state.get_mut_matrix();
    // matrix[0][2] = true;
    // matrix[1][0] = true;
    // matrix[1][2] = true;
    // matrix[2][1] = true;
    // matrix[2][1] = true;
    // matrix[2][2] = true;
    
    // Simple terminal mode
    // loop {
    //     print!("{}[2J", 27 as char);
    //     println!("{}", game_state);
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    //     game_state.forward_one_step();
    // }

    let mut game_state = GameState::new_random(69, 123);

    let mut app = App::new(game_state);
    app.run();

    }


