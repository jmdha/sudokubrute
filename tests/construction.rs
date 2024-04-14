use rstest::rstest;
use sudokubrute::board::Board;

#[rstest]
#[case("157468293238971645469532781926153478741896532583724916674385129892617354315249867")]
#[case("927384165684915237531672489769231548453768921218459673175826394392147856846593712")]
#[case("398256417476918253512743698185674329924835761763129845257361984831492576649587132")]
pub fn from_string(#[case] input: &str) {
    let board = Board::generate(input);
    assert!(board.is_some());
    let board = board.unwrap();
    input.as_bytes().into_iter().enumerate().for_each(|(i, c)| {
        let num = (*c as char).to_digit(10).unwrap();
        let cell = board.get_i(i);
        assert_eq!(cell as u32, num);
    })
}

#[rstest]
#[case("11...............................................................................")]
#[case("1........1.......................................................................")]
#[case("1.........1......................................................................")]
pub fn from_string_invalid(#[case] input: &str) {
    let board = Board::generate(input);
    assert!(board.is_none());
}
