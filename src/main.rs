use std::iter::IntoIterator;

// Define the Matrix9x9 struct
#[derive(Debug, Clone, Copy)]
struct Matrix9x9 {
    data: [[i32; 9]; 9],
}


impl IntoIterator for Matrix9x9 {
    type Item = i32;  // Assuming the matrix contains i32 values
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // Flatten the 2D array into a 1D vector and create an iterator
        self.data.into_iter().flatten().collect::<Vec<_>>().into_iter()
    }
}



// Implement methods for Matrix9x9
impl Matrix9x9 {
    // Create a new matrix
    fn new(data: [[i32; 9]; 9]) -> Self {
        Matrix9x9 { data }
    }

    // Print the matrix
    fn print(&self) {
        for row in &self.data {
            for val in row {
                print!("{} ", val);
            }
            println!();
        }
    }
}

fn main() {
    // Create a 9x9 matrix with some values
    let data = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [9, 8, 7, 6, 5, 4, 3, 2, 1],
        [1, 3, 5, 7, 9, 2, 4, 6, 8],
        [8, 6, 4, 2, 9, 7, 5, 3, 1],
        [1, 4, 7, 9, 2, 5, 8, 3, 6],
        [6, 3, 9, 7, 4, 1, 8, 5, 2],
        [2, 5, 8, 1, 3, 6, 9, 7, 4],
        [4, 7, 1, 3, 6, 9, 2, 5, 8],
        [8, 9, 1, 2, 3, 4, 5, 6, 7],
    ];
    
     let matrix_all = Matrix9x9::new(data);
    // let matrix_1x1 = matrix_all.data[0][0];
    // println!("{}",matrix_1x1);
    matrix_all.print();

    for i in matrix_all {
        println!("The value of i is: {}", i);
    }
}
