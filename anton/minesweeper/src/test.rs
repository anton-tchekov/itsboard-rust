#[test]
fn simple_field() {
	let expected = ["111", "1*1", "111"].map(|s| s.to_owned()).to_vec();
	let actual = minesweeper::annotate(&["   ", " * ", "   "]).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn empty_input() {
	let expected: Vec<String> = Vec::new();
	let actual = minesweeper::annotate(&[]).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn single_mine() {
	let input = ["*"];
	let expected = vec!["*"];
	let actual = minesweeper::annotate(&input).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn single_field() {
	let input = [" "];
	let expected = vec![" "];
	let actual = minesweeper::annotate(&input).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn sandwhich() {
	let input = ["* *"];
	let expected = vec!["*2*"];
	let actual = minesweeper::annotate(&input).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn star() {
	let input = [" * ", "***", " * "];
	let expected = vec!["3*3", "***", "3*3"];
	let actual = minesweeper::annotate(&input).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn empty_string() {
	let input = [""];
	let expected = vec![""];
	let actual = minesweeper::annotate(&input).expect("Should be valid field.");
	assert_eq!(expected, actual);
}

#[test]
fn invalid_character() {
	let input = [" *."];
	let expected = MinesweeperError::UnexpectedCharacter {
		line_number: 1,
		column_number: 3,
		character: '.',
	};
	let actual = minesweeper::annotate(&input).expect_err("Should be invalid field");
	assert_eq!(expected, actual);
}

#[test]
fn different_line_lengths() {
	let input = [" * ", "* "];
	let expected = MinesweeperError::LinesOfDifferentLengths { line_number: 2 };
	let actual = minesweeper::annotate(&input).expect_err("Should be invalid field");
	assert_eq!(expected, actual);
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
	board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
	row.as_bytes()
		.iter()
		.map(|&ch| match ch {
			b'*' => '*',
			_ => ' ',
		})
		.collect()
}

#[cfg(test)]
pub fn run_test(test_case: &[&str]) {
	let cleaned = remove_annotations(test_case);
	let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
	let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
	assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
	run_test(&[]);
}

#[test]
fn no_columns() {
	run_test(&[
		"",
	]);
}

#[test]
fn no_mines() {
	run_test(&[
		"   ",
		"   ",
		"   ",
	]);
}

#[test]
fn board_with_only_mines() {
	run_test(&[
		"***",
		"***",
		"***",
	]);
}

#[test]
fn mine_surrounded_by_spaces() {
	run_test(&[
		"111",
		"1*1",
		"111",
	]);
}

#[test]
fn space_surrounded_by_mines() {
	run_test(&[
		"***",
		"*8*",
		"***",
	]);
}

#[test]
fn horizontal_line() {
	run_test(&[
		"1*2*1",
	]);
}

#[test]
fn horizontal_line_mines_at_edges() {
	run_test(&[
		"*1 1*",
	]);
}

#[test]
fn vertical_line() {
	run_test(&[
		"1",
		"*",
		"2",
		"*",
		"1",
	]);
}

#[test]
fn vertical_line_mines_at_edges() {
	run_test(&[
		"*",
		"1",
		" ",
		"1",
		"*",
	]);
}

#[test]
fn cross() {
	run_test(&[
		" 2*2 ",
		"25*52",
		"*****",
		"25*52",
		" 2*2 ",
	]);
}

#[test]
fn large_board() {
	run_test(&[
		"1*22*1",
		"12*322",
		" 123*2",
		"112*4*",
		"1*22*2",
		"111111",
	]);
}

#[test]
fn test_read_file_to_string_invalid_filename(){
	let result = read_file_to_string("TestFiles/invalidfilename.txt");
	assert_eq!(result,Err("Failed to open the file."));
}

#[test]
fn test_read_file_to_string_with_empty_file(){
	let result = read_file_to_string("TestFiles/Test_empty.txt");
	assert_eq!(result,Err("The file is empty."));
}

#[test]
fn test_read_file_to_string_with_invalid_characters(){
	let result1 = read_file_to_string("TestFiles/Test_invalid_char.txt" );
	let result2 = read_file_to_string("TestFiles/Test_invalid_char2.txt" );
	assert_eq!(result1,Err("Invalid character detected in the file."));
	assert_eq!(result2,Err("Invalid character detected in the file."));
}

#[test]
fn test_read_file_to_string_with_valid_input(){

	let result1 = read_file_to_string("TestFiles/Test_valid_small1.txt");
	match result1 {
		Ok(content) => {
			assert_eq!(content, "* ");
		}
		Err(_) => panic!("Test failed"),
	}

	let result2 = read_file_to_string("TestFiles/Test_valid_small2.txt");
	match result2 {
		Ok(content) => {
			assert_eq!(content, "* \n  ");
		}
		Err(_) => panic!("Test failed"),
	}


	let result3 = read_file_to_string("TestFiles/Test_valid_small3.txt");
	match result3 {
		Ok(content) => {
			assert_eq!(content, "* * * * * \n      *   \n      *   \n      *   \n      *   ");
		}
		Err(_) => panic!("Test failed"),
	}

	let result4 = read_file_to_string("TestFiles/Test_valid_small4.txt");
	match result4 {
		Ok(content) => {
			assert_eq!(content, "* * * * * * * * * * * * * * * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			*                           * \n\
			* * * * * * * * * * * * * * * ")
		}
		Err(_) => panic!("Test failed"),
	}

}

#[test]
fn test_read_file_to_string_with_valid_large_input(){
	let result = read_file_to_string("TestFiles/Test_100_100.txt");
	match result {
		Ok(content) => {
			assert_eq!(content, "** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n** * *  * ** * * * ** *  * * * * * * ** ** ** *  * * ** *  * * * **  ** * * * ** * *  *  ** *  * * * *  * * *  * ** *\n");
		}
		Err(_) => panic!("Test failed"),
	}
}

#[test]
fn test_transform_minesweeper_succ_board_empty_board() {
	let mut empty_board: Vec<Vec<char>> = vec![
		vec![' ', ' ', ' '],
		vec![' ', ' ', ' '],
		vec![' ', ' ', ' ']
	];
	let empty_board_test: Vec<Vec<char>> = vec![
		vec![' ', ' ', ' '],
		vec![' ', ' ', ' '],
		vec![' ', ' ', ' ']
	];
	transform_minesweeper_board(&mut empty_board);
	assert_eq!(empty_board_test, empty_board);
}

#[test]
fn test_transform_minesweeper_board_succ_every_field_a_mine() {
	let mut board_full_of_mines: Vec<Vec<char>> = vec![
		vec!['*', '*', '*'],
		vec!['*', '*', '*'],
		vec!['*', '*', '*']
	];
	let board_full_of_mines_test: Vec<Vec<char>> = vec![
		vec!['*', '*', '*'],
		vec!['*', '*', '*'],
		vec!['*', '*', '*']
	];
	transform_minesweeper_board(&mut board_full_of_mines);
	assert_eq!(board_full_of_mines_test, board_full_of_mines);
}

#[test]
fn test_transform_minesweeper_board_succ_mine_in_a_corner() {
	let mut board_mine_in_corner: Vec<Vec<char>> = vec![
		vec!['*', ' ', ' '],
		vec![' ', ' ', ' '],
		vec![' ', ' ', ' ']
	];
	let board_expected_result: Vec<Vec<char>> = vec![
		vec!['*', '1', ' '],
		vec!['1', '1', ' '],
		vec![' ', ' ', ' ']
	];
	transform_minesweeper_board(&mut board_mine_in_corner);
	assert_eq!(board_expected_result, board_mine_in_corner);
}

#[test]
fn test_transform_minesweeper_board_succ_mine_on_edge() {
	let mut board_mine_on_edge: Vec<Vec<char>> = vec![
		vec![' ', ' ', ' '],
		vec!['*', ' ', ' '],
		vec![' ', ' ', ' ']
	];
	let board_expected_result: Vec<Vec<char>> = vec![
		vec!['1', '1', ' '],
		vec!['*', '1', ' '],
		vec!['1', '1', ' ']
	];
	transform_minesweeper_board(&mut board_mine_on_edge);
	assert_eq!(board_expected_result, board_mine_on_edge);
}

#[test]
fn test_transform_minesweeper_board_succ_valid_board() {
	let mut valid_board: Vec<Vec<char>> = vec![
		vec![' ', ' ', '*', ' ', ' '],
		vec![' ', '*', '*', ' ', ' '],
		vec![' ', ' ', ' ', ' ', ' '],
		vec![' ', ' ', ' ', '*', '*'],
		vec!['*', '*', ' ', ' ', ' '],
	];

	let board_expected_result: Vec<Vec<char>> = vec![
		vec!['1', '3', '*', '2', ' '],
		vec!['1', '*', '*', '2', ' '],
		vec!['1', '2', '3', '3', '2'],
		vec!['2', '2', '2', '*', '*'],
		vec!['*', '*', '2', '2', '2'],
	];

	transform_minesweeper_board(&mut valid_board);
	assert_eq!(board_expected_result, valid_board);
}

#[test]
fn test_create_board_from_data_succ() {
	let data = " * * \n  *  \n  *  \n     ";
	let expected_result: Vec<Vec<char>> = data
		.split('\n')
		.map(|line| line.chars().collect())
		.collect();

	let created_board = create_board_from_data(data.to_string());

	assert_eq!(expected_result, created_board);
}

#[test]
fn test_create_board_from_data_fail_one_line_has_different_length() {
	let data = "   * \n   *  \n    *\n  *  ";
	assert!(std::panic::catch_unwind(|| {
		create_board_from_data(data.to_string())
	}).is_err());
}

fn test_func(expected: Vec<&str>) {
	let input = create_input(&expected);
	let input_str: Vec<&str> = input.iter().map(|x| x.as_str()).collect();
	let output = annotate(&input_str);
	assert_eq!(expected, output)
}

fn create_input(field: &[&str]) -> Vec<String> {
	field.iter().map(|&line| create_input_line(line)).collect()
}

fn create_input_line(line: &str) -> String {
	line.as_bytes().iter().map(|&char| match char {
		b'*' => '*',
		_ => ' '
	}).collect()
}

#[test]
fn test_create_input() {
	let field = vec![
		"*33*2**23***211",
		"3**3334*3**53*1",
		"*6*21*2134*3*32",
		"**21111 1*2212*",
		"4421221 1221 11",
		"**11**2  1*21  ",
		"*4224*2 123*21 ",
		"2*11*21 1*22*1 "
	];
	let expected = vec![
		"*  * **  ***   ",
		" **    * **  * ",
		"* *  *    * *  ",
		"**       *    *",
		"               ",
		"**  **    *    ",
		"*    *     *   ",
		" *  *    *  *  "
	];
	let actual = create_input(&field);
	assert_eq!(expected, actual)
}

#[test]
fn test() {
	test_func(vec![
		"*222*",
		"2*3*2",
		"23*32",
		"2*3*2",
		"*222*"
	])
}

#[test]
fn test_irregular_field() {
	test_func(vec![
		"*21 ",
		"3*2 ",
		"3*3 ",
		"3*2 ",
		"*21 "
	])
}

#[test]
fn test_all_bombs() {
	test_func(vec![
		"***",
		"***",
		"***"
	])
}

#[test]
fn test_no_bombs() {
	test_func(vec![
		"   ",
		"   ",
		"   "
	])
}

#[test]
fn test_corner_bomb() {
	test_func(vec![
		"*1",
		"11"
	])
}

#[test]
fn test_single_bomb() {
	test_func(vec![
		"*"
	])
}

#[test]
fn test_large_field() {
	test_func(vec![
		"*1  ",
		"221 ",
		"1*1 ",
		"111 "
	])
}

#[test]
fn test_one_bomb_center() {
	test_func(vec![
		"111",
		"1*1",
		"111"
	])
}

#[test]
fn test_one_bomb_bottom_right() {
	test_func(vec![
		"11",
		"1*"
	])
}

#[test]
fn test_multiple_bombs() {
	test_func(vec![
		"*23*",
		"2*4*",
		"24*3",
		"*3*2"
	])
}

#[test]
fn test_large_irregular_field() {
	test_func(vec![
		"*21 ",
		"2*21",
		"23*1",
		"2*21",
		"*21 "
	])
}

#[test]
fn test_very_large_field() {
	test_func(vec![
		"*1      ",
		"221     ",
		"1*1     ",
		"111 111 ",
		"    2*2 ",
		"    3*3 ",
		"    2*2 "
	])
}

#[test]
fn test_random() {
	test_func(vec![
		"*33*2**23***211",
		"3**3334*3**53*1",
		"*6*21*2134*3*32",
		"**21111 1*2212*",
		"4421221 1221 11",
		"**11**2  1*21  ",
		"*4224*2 123*21 ",
		"2*11*21 1*22*1 "
	])
}

#[test]
fn annotate_success_empty_field() {
	let expected: Vec<String> = Vec::new();
	let expected2 = vec![""];
	let actual = annotate(&[]);
	let actual2 = annotate(&[""]);
	assert_eq!(expected, actual);
	assert_eq!(expected2, actual2);
}

#[test]
fn annotate_success_simple_field() {
	let expected = vec![
		"*4*11*1 12*2*21",
		"**21111 1*2212*",
		"4421221 1221 11",
		"**11**1  1*1   "];
	let field = vec![
		"* *  *    * *  ",
		"**       *    *",
		"               ",
		"**  **    *    ",
	];
	let actual = annotate(&field);
	assert_eq!(expected, actual);
}

#[test]
fn annotate_success_no_mines() {
	let input = vec!["   ", "   ", "   "];
	let expected: Vec<String> = vec!["   ".to_string(), "   ".to_string(), "   ".to_string()];
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn annotate_success_only_mines() {
	let input = vec!["***", "***", "***"];
	let expected: Vec<String> = vec!["***".to_string(), "***".to_string(), "***".to_string()];
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn annotate_success_one_row() {
	let input = vec!["* *  *    *"];
	let expected: Vec<String> = vec!["*2*11*1  1*".to_string()];
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn annotate_success_one_column() {
	let input = vec!["*", " ", "*", "*", " ", " ", "*", " "];
	let expected: Vec<String> = ["*", "2", "*", "*", "1", "1", "*", "1"].iter().map(|e| e.to_string()).collect();
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn annotate_success_empty_starting_row() {
	let input = vec!["", "* "];
	let expected: Vec<String> = vec![];
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn annotate_fail_invalid_length() {
	let input = vec!["** ", " *", "* *"];
	let expected: Vec<String> = vec![];
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn annotate_fail_invalid_char() {
	let input = vec!["** ", " Ö*", "* *"];
	let expected: Vec<String> = vec![];
	let result = annotate(&input);
	assert_eq!(expected, result);
}

#[test]
fn test_annotate_no_mines() {
	let empty_board = vec!["    ", "    ", "    "];
	let expected_result = vec!["    ", "    ", "    "];
	assert_eq!(annotate(&empty_board), expected_result);
}

#[test]
fn test_annotate_empty_board() {
	let empty_board = vec![""];
	let expected_result = vec![""];
	assert_eq!(annotate(&empty_board), expected_result);
}

#[test]
fn test_annotate_empty_vector() {
	let empty_board = vec![];
	let expected_result:Vec<&str> = vec![];
	assert_eq!(annotate(&empty_board), expected_result);
}

#[test]
fn test_annotate_all_mines() {
	let board = vec!["*****", "*****", "*****"];
	let expected_result = vec!["*****", "*****", "*****"];
	assert_eq!(annotate(&board), expected_result);
}

#[test]
fn test_annotate_no_adjacent_mines() {
	let board = vec!["* * *", "     ", " * * "];
	let expected_result = vec!["*2*2*", "23332", "1*2*1"];
	assert_eq!(annotate(&board), expected_result);
}

#[test]
fn test_annotate_with_adjacent_mines() {
	let board = vec!["***  ","     ", "  ** ", "*    "];
	let expected_result = vec!["***1 ", "24431", "12**1", "*2221"];
	assert_eq!(annotate(&board), expected_result);
}

#[test]
fn test_get_board_dimensions() {
	let board = vec!["* * *", "     ", " * * ", "     "];
	assert_eq!(get_board_dimensions(&board), (4, 5));
}

#[cfg(test)]
fn remove_marks(field: Vec<Vec<char>>) -> Vec<Vec<char>> {
	let mut blank_field = vec![];
	for row_num in 0..field.len() {
		blank_field.push(Vec::new());
		for pos_num in 0..field[row_num].len() {
			if field[row_num][pos_num] == '*' {
				blank_field[row_num].push('*');
			} else {
				blank_field[row_num].push(' ');
			};
		};
	};
	blank_field
}

#[cfg(test)]
fn exec_test(field: Vec<Vec<char>>) {
	let mut blank_field = remove_marks(field.clone());
	check_mines(&mut blank_field);
	assert_eq!(field, blank_field);
}

#[test]
fn corner_test() {
	exec_test(vec![
		vec!['*', '3', '*', '2', '*'],
		vec!['*', '3', '1', '2', '1'],
		vec!['1', '1', ' ', '1', '1'],
		vec!['1', '1', ' ', '2', '*'],
		vec!['*', '1', ' ', '2', '*']]);
}

#[test]
fn basic_test() {
	exec_test(vec![
		vec![' ', ' ', ' ', ' ', ' '],
		vec!['1', '1', '2', '1', '1'],
		vec!['1', '*', '3', '*', '2'],
		vec!['2', '2', '4', '*', '2'],
		vec!['1', '*', '2', '1', '1']]);
}

#[test]
fn large_empty_test() {
	exec_test(vec![
		vec![' ', ' ', ' ', ' ', ' '],
		vec![' ', ' ', ' ', ' ', ' '],
		vec![' ', ' ', ' ', ' ', ' '],
		vec![' ', ' ', ' ', ' ', ' '],
		vec![' ', ' ', ' ', ' ', ' ']]);
}

fn remove_annotations(board: &[&str]) -> Vec<String> {
	board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
	row.as_bytes()
		.iter()
		.map(|&ch| match ch {
			b'*' => '*',
			_ => ' ',
		})
		.collect()
}

fn run_test(test_case: &[&str]) {
	let cleaned = remove_annotations(test_case);
	let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
	let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
	assert_eq!(expected, annotate(&cleaned_strs).expect("RUN_TEST"));
}

fn run_negative_test(test_case: &[&str]) {
	let test_field = test_case.iter().map(|r| &r[..]).collect::<Vec<_>>();
	assert!(annotate(&test_field).is_err());
}

#[test]
fn no_rows() {
	run_test(&[
	]);
}

#[test]
fn no_columns() {
	run_test(&[
		"",
	]);
}

#[test]
fn no_mines() {
	run_test(&[
		"   ",
		"   ",
		"   ",
	]);
}

#[test]
fn board_with_only_mines() {
	run_test(&[
		"***",
		"***",
		"***",
	]);
}

#[test]
fn mine_surrounded_by_spaces() {
	run_test(&[
		"111",
		"1*1",
		"111",
	]);
}

#[test]
fn space_surrounded_by_mines() {
	run_test(&[
		"***",
		"*8*",
		"***",
	]);
}

#[test]
fn horizontal_line() {
	run_test(&[
		"1*2*1",
	]);
}

#[test]
fn horizontal_line_mines_at_edges() {
	run_test(&[
		"*1 1*",
	]);
}

#[test]
fn vertical_line() {
	run_test(&[
		"1",
		"*",
		"2",
		"*",
		"1",
	]);
}

#[test]
fn vertical_line_mines_at_edges() {
	run_test(&[
		"*",
		"1",
		" ",
		"1",
		"*",
	]);
}

#[test]
fn cross() {
	run_test(&[
		" 2*2 ",
		"25*52",
		"*****",
		"25*52",
		" 2*2 ",
	]);
}

#[test]
fn large_board() {
	run_test(&[
		"1*22*1",
		"12*322",
		" 123*2",
		"112*4*",
		"1*22*2",
		"111111",
	]);
}

#[test]
fn unequal_row_length_simple() {
	run_negative_test(&[
		"",
		" ",
	]);
}

#[test]
fn unequal_row_length_complex() {
	run_negative_test(&[
		" * * ",
		" * * ",
		" * *",
	]);
}

#[test]
fn wrong_character() {
	run_negative_test(&[
		"_* * ",
		" * * ",
		" * * ",
	]);
}

#[test]
fn wrong_character_empty() {
	run_negative_test(&[
		"       ",
		"       ",
		"     _ ",
	]);
}

#[test]
fn wrong_character_full() {
	run_negative_test(&[
		"*********",
		"**A******",
		"*********",
		"*********",
	]);
}

#[test]
fn no_file() {
	let path = PathBuf::from("../no_file.txt");

	let matrix = file_to_matrix(&path);

	assert!(matrix.is_err(), "The path points to an existing file");
}

#[test]
fn wrong_input_char() {
	let path = PathBuf::from("../example_file_wrong_chars.txt");

	let matrix = file_to_matrix(&path);

	assert!(matrix.is_err(), "The file has no violating chars");
}

#[test]
fn wrong_input_length() {
	let path = PathBuf::from("../example_file_wrong_length.txt");

	let matrix = file_to_matrix(&path);

	assert!(matrix.is_err(), "The lines are all equal in length");
}

#[test]
fn right_input() {
	let path = PathBuf::from("../example_file.txt");

	let matrix = file_to_matrix(&path);

	assert!(matrix.is_ok(), "The file does not exist, has violating chars or the line lengths are not equal in length");
}

#[test]
fn solved_right() {
	let path = PathBuf::from("../example_file.txt");
	let expected = vec![
		vec!["1", "*", "3", "*", "1"],
		vec!["1", "3", "*", "3", "1"],
		vec![" ", "2", "*", "2", " "],
		vec![" ", "1", "1", "1", " "],
	];

	let solved_matrix = play_the_game_from_file(&path);
	let matching_count: usize;

	assert!(solved_matrix.is_ok());

	matching_count = count_same(&solved_matrix.unwrap(), &expected);

	assert_eq!(matching_count, 20, "The solved chars doesn't match with the expected ones")
}

#[test]
fn solved_wrong() {
	let path = PathBuf::from("../example_file.txt");
	let expected = vec![
		vec![" ", "*", " ", "*", " "],
		vec!["100", "3", "", "3", "1"],
		vec![" ", "", "*", "200", " "],
		vec![" ", "1", " ", "", " "],
	];
	let solved_matrix = play_the_game_from_file(&path);
	let matching_count: usize;

	assert!(solved_matrix.is_ok());

	matching_count = count_same(&solved_matrix.unwrap(), &expected);

	assert_eq!(matching_count, 11, "The solved chars does match with the expected ones")
}

fn count_same(one: &Vec<Vec<String>>, two: &Vec<Vec<&str>>) -> usize {
	one.iter()
		.zip(two.iter())
		.map(|(&ref one_line, &ref two_line)| {
			one_line
				.iter()
				.zip(two_line.iter())
				.filter(|&(one_char, two_char)| one_char == two_char)
				.count()
		})
		.sum()
}

fn remove_annotations(board: &[&str]) -> Vec<String> {
	board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
	row.as_bytes()
		.iter()
		.map(|&ch| match ch {
			b'*' => '*',
			_ => ' ',
		})
		.collect()
}

fn run_test(test_case: &[&str]) {
	let cleaned = remove_annotations(test_case);
	let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
	let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
	assert_eq!(expected, annotate(&cleaned_strs).unwrap());
}

#[test]
fn diagonal() {
	run_test(&[
		"",
		"1",
		"3*2",
		"**41",
		"3*3*2*",
		"2343322",
		"1**2*11*",
	]);
}

#[test]
fn diagonal2() {
	run_test(&[
		"1**2*11*",
		"2343322",
		"3*3*2*",
		"**41",
		"3*2",
		"1",
		"",
	]);
}

#[test]
fn hello() {
	let input = &[
		"  *   *  *****  *      *      *****  ",
		"  *   *  *      *      *      *   *  ",
		"  *****  *****  *      *      *   *  ",
		"  *   *  *      *      *      *   *  ",
		"  *   *  *****  *****  *****  *****  ",
	];

	let expected = vec![
		" 2*2 2*22*****12*2    2*2    2*****2 ",
		" 3*535*33*766423*3    3*3    3*535*3 ",
		" 3*****33*****13*3    3*3    3*3 3*3 ",
		" 3*535*33*766423*533213*533213*535*3 ",
		" 2*2 2*22*****12*****12*****12*****2 ",
	];

	assert_eq!(expected, annotate(input).unwrap());
}

fn performance_test(pattern: &str, pattern_edges: &str, size: usize) {
	let mut field = Vec::with_capacity(size + 2);
	let edges = pattern_edges.repeat(size);
	let inner = pattern.repeat(size);
	field.push(edges.as_str());
	for _ in 0..size {
		field.push(inner.as_str());
	}
	field.push(edges.as_str());

	run_test(&field);
}

#[test]
fn performance() {
	performance_test("3*3", "2*2", 500);
}

#[test]
fn performance_no_mines() {
	performance_test("   ", "   ", 500);
}

#[test]
fn performance_all_mines() {
	performance_test("***", "***", 500);
}

#[cfg(test)]
fn create_expected_board(board: &[&str]) -> Vec<String> {
	board.iter().map(|r| r.to_string()).collect()
}

#[test]
fn malformed_board() {
	const INPUT: [&str; 3] = [" **  ", "    ", "   *"];

	const EXPECTED: Vec<String> = vec![];

	assert_eq!(EXPECTED, annotate(&INPUT));
}

#[test]
fn weird_chars() {
	const INPUT: [&str; 3] = ["*Ö*", "   ", "   "];

	const EXPECTED: Vec<String> = vec![];

	assert_eq!(EXPECTED, annotate(&INPUT));
}

#[test]
fn simple_field() {
	const INPUT: [&str; 3] = [" * ", "   ", " * "];

	let expected = create_expected_board(&["1*1", "222", "1*1"]);

	assert_eq!(expected, annotate(&INPUT));
}

#[test]
fn mine_circle() {
	const INPUT: [&str; 3] = ["***", "* *", "***"];

	let expected = create_expected_board(&["***", "*8*", "***"]);

	assert_eq!(expected, annotate(&INPUT));
}

#[test]
fn corner_mines() {
	const INPUT: [&str; 3] = ["* *", "   ", "* *"];

	let expected = create_expected_board(&["*2*", "242", "*2*"]);

	assert_eq!(expected, annotate(&INPUT));
}

#[test]
fn just_words() {
	const INPUT: [&str; 3] = ["DEAD", "BEEF", "OBST"];

	const EXPECTED: Vec<String> = vec![];

	assert_eq!(EXPECTED, annotate(&INPUT));
}

#[test]
fn really_really_loooooooooooooooooooooooong_board() {
	const INPUT: [&str; 4] = [
		"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        ",
		"****************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************",
		"                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        ",
		"****************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************",
		];

	let expected = create_expected_board(&[
		"2333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333332",
		"****************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************",
		"4666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666664",
		"****************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************************",
	]);

	assert_eq!(expected, annotate(&INPUT));
}

#[test]
fn test_invalid_character(){
	let wrong_grid: [&str; 5] = ["    z","     ","     ","     ","     "];
	let erwartet : Vec<String>  = vec![];
	assert_eq!(erwartet, annotate(&wrong_grid));
}

#[test]
fn test_few_mines(){
	let mine_grid: [&str; 5] = ["     ","*   *"," *   ","     ","   * "];
	let number_grid = create_expected_board(&["11 11","*211*","2*111","11211","  1*1"]);
	assert_eq!(number_grid, annotate(&mine_grid));

	let mine_grid: [&str; 5] = ["   *   ","       "," *   * ","     * ","       "];
	let number_grid = create_expected_board(&["  1*1  ","1121211","1*1 2*2","111 2*2","    111"]);
	assert_eq!(number_grid, annotate(&mine_grid));

	let mine_grid: [&str; 3] = [" *     *  ","          ","    **    "];
	let number_grid = create_expected_board(&["1*1   1*1 ","111122211 ","   1**1   "]);
	assert_eq!(number_grid, annotate(&mine_grid));
}
#[test]
fn test_many_mines(){
	let mine_grid: [&str; 5] = ["* ** ","*   *","* ***","     ","** * "];
	let number_grid = create_expected_board(&["*3**2","*546*","*3***","34443","**2*1"]);
	assert_eq!(number_grid, annotate(&mine_grid));

	let mine_grid: [&str; 5] = ["** * * ","  *   *","**   * "," **  * "," *   * "];
	let number_grid = create_expected_board(&["**3*2*2","45*233*","**422*3","4**13*3","2*312*2"]);
	assert_eq!(number_grid, annotate(&mine_grid));

	let mine_grid: [&str; 3] = [" *  ** *  ","  *   *   ","*** **  * "];
	let number_grid = create_expected_board(&["1*22**3*1 ","35*445*321","***3**22*1"]);
	assert_eq!(number_grid, annotate(&mine_grid));
}

#[test]
fn test_file_good() {
	let expected = create_expected_board(&["***", "232", "   ", "232", "***"]);
	assert_eq!(expected, annotate_from_file("test_files/test_field.txt"));
}

#[test]
fn test_file_bad_length() {
	const EXPECTED: Vec<String> = vec![];


	/*


****


***


	*/

	assert_eq!(EXPECTED, annotate_from_file("test_files/bad_length.txt"));
}

#[test]
fn test_file_bad_char() {
	const EXPECTED: Vec<String> = vec![];

	/*

***
E

***


*/
	assert_eq!(EXPECTED, annotate_from_file("test_files/bad_char.txt"));
}

#[test]
fn generic_example_1() {
	let buffer = vec![
		vec!['*', '.', '.', '.'],
		vec!['.', '.', '.', '.'],
		vec!['.', '*', '.', '.'],
		vec!['.', '.', '.', '.'],
	];

	let result = minesweeper(buffer).unwrap();

	assert_eq!(result, vec![
		vec!['*', '1', '.', '.'],
		vec!['2', '2', '1', '.'],
		vec!['1', '*', '1', '.'],
		vec!['1', '1', '1', '.'],
	]);
}

#[test]
fn generic_example_2() {
	let buffer = vec![
		vec!['.', '.', '.', '.', '.', '.'],
		vec!['*', '*', '*', '*', '*', '*'],
		vec!['.', '.', '.', '.', '.', '.'],
		vec!['*', '*', '*', '*', '*', '*'],
	];

	let result = minesweeper(buffer).unwrap();

	assert_eq!(result, vec![
		vec!['2', '3', '3', '3', '3', '2'],
		vec!['*', '*', '*', '*', '*', '*'],
		vec!['4', '6', '6', '6', '6', '4'],
		vec!['*', '*', '*', '*', '*', '*'],
	]);
}

#[test]
fn highest_neighbour_count() {
	let buffer = vec![
		vec!['*', '*', '*'],
		vec!['*', '.', '*'],
		vec!['*', '*', '*'],
	];

	let result = minesweeper(buffer).unwrap();

	assert_eq!(result, vec![
		vec!['*', '*', '*'],
		vec!['*', '8', '*'],
		vec!['*', '*', '*'],
	]);
}

#[test]
fn only_mines() {
	let buffer = vec![
		vec!['*', '*', '*'],
		vec!['*', '*', '*'],
		vec!['*', '*', '*'],
	];

	let result = minesweeper(buffer).unwrap();

	assert_eq!(result, vec![
		vec!['*', '*', '*'],
		vec!['*', '*', '*'],
		vec!['*', '*', '*'],
	]);
}

#[test]
fn one_cell_one_mine() {
	let buffer = vec![
		vec!['*'],
	];

	let result = minesweeper(buffer).unwrap();

	assert_eq!(result, vec![
		vec!['*'],
	]);
}

#[test]
fn one_cell_no_mine() {
	let buffer = vec![
		vec!['.'],
	];

	let result = minesweeper(buffer).unwrap();

	assert_eq!(result, vec![
		vec!['.'],
	]);
}

#[test]
fn invalid_input() {
	let buffer = vec![
		vec!['*', '.'],
		vec!['.', '.', '.', '.'],
	];

	let result = minesweeper(buffer);

	assert_eq!(result, Err("The input is not a rectangle".to_string()));
}

#[test]
fn wrong_input() {
	let result = read_from_file("test_wrong_input".to_string());

	assert_eq!(result, Err("Invalid character in the input".to_string()));
}

#[cfg(test)]
fn vector_from_array(board: &[&str]) -> Vec<String> {
	board.iter().map(|r| r.to_string()).collect()
}

#[test]
fn no_mines() {
	let result = annotate(&[
		"   ",
		"   ",
		"   ",
	]);

	let expected_result = vector_from_array(&[
		"   ",
		"   ",
		"   ",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn unequal_board() {
	let result = annotate(&[
		"*  ",
		" ",
		"  *",
	]);

	let expected_result = vector_from_array(&[

	]);
	assert_eq!(expected_result, result);
}

#[test]
fn only_mines() {
	let result = annotate(&[
		"******",
		"******",
		"******",
	]);

	let expected_result = vector_from_array(&[
		"******",
		"******",
		"******",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn big_empty_board() {
	let result = annotate(&[
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
	]);

	let expected_result = vector_from_array(&[
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
		"                                        ",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn eight_mines() {
	let result = annotate(&[
		"***",
		"* *",
		"***",
	]);

	let expected_result = vector_from_array(&[
		"***",
		"*8*",
		"***",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn one_mine() {
	let result = annotate(&[
		"   ",
		" * ",
		"   ",
	]);

	let expected_result = vector_from_array(&[
		"111",
		"1*1",
		"111",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn empty_rows() {
	let result = annotate(&[
		"",
		"",
		"",
		"",
		"",
		"",
		"",
		"",
	]);

	let expected_result = vector_from_array(&[
		"",
		"",
		"",
		"",
		"",
		"",
		"",
		"",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn empty_board() {
	let result = annotate(&[

	]);

	let expected_result = vector_from_array(&[

	]);
	assert_eq!(expected_result, result);
}

#[test]
fn invalid_char() {
	let result = annotate(&[
		"* *",
		"  *",
		"*Ö*",
	]);

	let expected_result = vector_from_array(&[

	]);
	assert_eq!(expected_result, result);
}

#[test]
fn f_bad_char() {

/*

	***
	E

	***

*/

	let result = annotate_with_file("input/bad_char.txt");

	let expected_result = vector_from_array(&[

	]);
	assert_eq!(expected_result, result);
}

#[test]
fn f_bad_length() {
	/*


****


***


	*/



	let result = annotate_with_file("input/bad_length.txt");

	let expected_result = vector_from_array(&[

	]);
	assert_eq!(expected_result, result);
}

#[test]
fn f_test_field() {

/*

***



***


*/


	let result = annotate_with_file("input/test_field.txt");

	let expected_result = vector_from_array(&[
		"***",
		"232",
		"   ",
		"232",
		"***",
	]);
	assert_eq!(expected_result, result);
}

#[test]
fn f_sophia_tom() {

/*

*  *

*  *

*/

	let result = annotate_with_file("input/sophia_tom_input.txt");

	let expected_result = vector_from_array(&[
		"*11*",
		"2222",
		"*11*",
	]);
	assert_eq!(expected_result, result);
}
