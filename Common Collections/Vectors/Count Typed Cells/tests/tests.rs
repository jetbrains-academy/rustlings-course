use count_typed_cells::{count_cells, SpreadsheetCell};
use count_typed_cells::SpreadsheetCell::{Float, Int, Text};

#[test]
fn test_empty() {
    let vec: Vec<SpreadsheetCell> = vec![];
    let cc = count_cells(&vec);
    assert_eq!(cc.ints, 0,  "There are no cells in empty vector");
    assert_eq!(cc.floats, 0,  "There are no cells in empty vector");
    assert_eq!(cc.texts, 0,  "There are no cells in empty vector");
}

#[test]
fn test_non_empty() {
    let vec: Vec<SpreadsheetCell> = vec![
        Int(0), Int(1), Int(2),
        Float(0f64),
        Text("aa".to_string()), Text("bb".to_string())
    ];
    let cc = count_cells(&vec);
    assert_eq!(cc.ints, 3,  "There should be some cells in non-empty vector");
    assert_eq!(cc.floats, 1,  "There should be some cells in non-empty vector");
    assert_eq!(cc.texts, 2,  "There should be some cells in non-empty vector");
}

#[test]
fn test_non_empty_2() {
    let vec: Vec<SpreadsheetCell> = vec![
        Int(0), Int(1),
        Text("aa".to_string())
    ];
    let cc = count_cells(&vec);
    assert_eq!(cc.ints, 2,  "There should be some cells in non-empty vector");
    assert_eq!(cc.floats, 0,  "There should be some cells in non-empty vector");
    assert_eq!(cc.texts, 1,  "There should be some cells in non-empty vector");
}
