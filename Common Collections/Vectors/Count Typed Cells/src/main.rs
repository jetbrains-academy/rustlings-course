use count_typed_cells::{*};
use count_typed_cells::SpreadsheetCell::{*};

fn main() {
    let vec: Vec<SpreadsheetCell> = vec![
        Int(0), Int(1), Int(2),
        Float(0f64),
        Text("aa".to_string()), Text("bb".to_string())
    ];
    let cc = count_cells(&vec);
    println!("{:?}", cc)
}
