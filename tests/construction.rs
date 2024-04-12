use rstest::rstest;
use sudokubrute::board::Board;

#[rstest]
#[case("123456789123456789123456789123456789123456789123456789123456789123456789123456789")]
#[case("987654321987654321987654321987654321987654321987654321987654321987654321987654321")]
pub fn from_string(#[case] input: &str) {
    let board = Board::from(input);

    input.as_bytes().into_iter().enumerate().for_each(|(i, c)| {
        let num = (*c as char).to_digit(10).unwrap();
        let cell = board.get_i(i);
        assert!(cell.is_some());
        let cell = cell.unwrap();
        assert_eq!(cell as u32, num);
    })
}
