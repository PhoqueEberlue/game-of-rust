// Neighbor const
const NEIGHBOR_NUMBER: usize = 8;
// Moore neighborhood
const NEIGHBOR_POSITIONS: [(i8, i8); NEIGHBOR_NUMBER] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1)
    //Y,  X
];

pub struct GameState {
    matrix: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

/// The Game state is stored as a matrix represented with the following conventions:
///
///  ----------- X ---------> AKA width
/// |[[ false, false, false ],
/// |
/// Y [ false, true , false ],
/// |
/// V [ false, false, false ]]
/// AKA height
///
impl GameState {
    /// Create an empty game state
    pub fn new(height: usize, width: usize) -> GameState {
        GameState {
            matrix: GameState::init_matrix(height, width),
            height, width
        }
    }

    /// Create a game state from a given matrix
    pub fn from(matrix: Vec<Vec<bool>>) -> GameState {
        let height = matrix.len();
        let width = matrix[0].len();

        for row in &matrix {
            if width != row.len() { panic!("Matrix is not valid") }
        }

        GameState { matrix, height, width }
    }

    /// Create a random game state
    pub fn new_random(height: usize, width: usize) -> GameState {
        GameState {
            matrix: GameState::init_random_matrix(height, width),
            height, width
        }
    }

    /// init empty matrix
    fn init_matrix(height: usize, width: usize) -> Vec<Vec<bool>> {
        let mut matrix = Vec::with_capacity(height);

        for _ in 0..height {
            matrix.push(vec![false; width]);
        }

        matrix
    }

    /// init random matrix
    fn init_random_matrix(height: usize, width: usize) -> Vec<Vec<bool>> {
        let mut matrix = GameState::init_matrix(height, width);
        
        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                matrix[y][x] = rand::random::<bool>();
            }
        }

        matrix
    }

    /// Gets the neighbor of a cell
    fn get_neighbors_values(&self, x: usize, y: usize) -> Vec<bool> {
        let mut res = Vec::with_capacity(NEIGHBOR_NUMBER);

        // py and px are the values to be summed on the current coordinates to get a given neighbor
        for (py, px) in NEIGHBOR_POSITIONS {
            let (neighbor_x, neighbor_y) = self.get_neighbor_index(x, y, px, py);
            res.push(self.matrix[neighbor_y][neighbor_x]);
        }

        res
    }

    fn get_neighbor_index(&self, x: usize, y: usize, px: i8, py: i8) -> (usize, usize) {
        // indexes of the neighbor
        let (neighbor_x, neighbor_y): (usize, usize);
        // temp indexes to handle border cases
        let (tmp_x, tmp_y) = (x as i16 + px as i16, y as i16 + py as i16) ;

        // On the borders we just take the value at the opposite side
        if tmp_x < 0 {
            neighbor_x = self.width - 1;
        } else if tmp_x >= self.width as i16 { 
            neighbor_x = 0
        } else {
            // Keep the value computed with px
            neighbor_x = tmp_x as usize;
        }

        if tmp_y < 0 {
            neighbor_y = self.height - 1;
        } else if tmp_y >= self.height as i16 { 
            neighbor_y = 0
        } else {
            neighbor_y = tmp_y as usize;
        }

        (neighbor_x, neighbor_y)
    }

    pub fn forward_one_step(&mut self) {
        let mut new_game_state = self.matrix.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let cell_value = self.matrix[y][x];
                let neighbors_values = self.get_neighbors_values(x, y);

                match cell_value {
                    true => {
                        // if more than 4 alive neighbors the cell dies
                        if neighbors_values.iter().filter(|&x| *x).count() >= 4 {
                            new_game_state[y][x] = false;
                        // if less than 1 alive neighbors the cell dies
                        } else if neighbors_values.iter().filter(|&x| *x).count() <= 1 {
                            new_game_state[y][x] = false;
                        }
                        // otherwise, the cell stays alive 
                    },
                    false => {
                        // if exactly 3 alive neighbors the cell come to life 
                        if neighbors_values.iter().filter(|&x| *x).count() == 3 {
                            new_game_state[y][x] = true;
                        }
                        // otherwise, the cell stays dead 
                    }
                };
            }
        }

        self.matrix = new_game_state;
    }

    pub fn get_matrix(&self) -> &Vec<Vec<bool>>{
        &self.matrix
    }

    pub fn get_mut_matrix(&mut self) -> &mut Vec<Vec<bool>>{
        &mut self.matrix
    }
}

// Implementing display trait for the structure GameState
impl std::fmt::Display for GameState {
    // Overriding formating function
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        res.push_str(&"-".repeat(self.width));
        res.push('\n');
        // Loop through the matrix to display the values
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.matrix[y][x];

                let display_value = match value {
                    true => "█",
                    false => " ",
                };

                res.push_str(display_value);
            }
            res.push('\n');
        }

        res.push_str(&"-".repeat(self.width));
        res.push('\n');

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::GameState;

    #[test]
    fn test_new() {
        let game_state = GameState::new(100, 100);

        assert_eq!(game_state.matrix.len(), 100);

        for i in 0..100 {
            assert_eq!(game_state.matrix[i].capacity(), 100);
        }
    }

    #[test]
    fn test_from() {
        let matrix = vec![
            vec![false, false, false],
            vec![true , true , false],
            vec![true , false, false]
        ];

        let game_state = GameState::from(matrix);

        assert_eq!(game_state.matrix[1][0], true);
        assert_eq!(game_state.matrix[2][0], true);
        assert_eq!(game_state.matrix[1][1], true);

        println!("{}", game_state);
    }

    #[test]
    fn test_get_neighbour_index() {
        let game_state = GameState::new(10, 10);

        assert_eq!(game_state.get_neighbor_index(1, 2, -1, -1), (0, 1));
        assert_eq!(game_state.get_neighbor_index(3, 5, 0, 1), (3, 6));
        assert_eq!(game_state.get_neighbor_index(0, 0, -1, -1), (9, 9));
        assert_eq!(game_state.get_neighbor_index(9, 9, 1, 1), (0, 0));
    }

    #[test]
    fn test_get_neighbors_values() {
        let matrix = vec![
            vec![false, false, false, false],
            vec![true , true , false, false],
            vec![true , false, false, false],
            vec![true , false, false, false]
        ];

        let game_state = GameState::from(matrix);
        let res = game_state.get_neighbors_values(0, 0);

        assert_eq!(res, vec![false, true, false, false, false, false, true, true]);
    }
}
