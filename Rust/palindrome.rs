#![allow(dead_code)]
#![allow(unused)]

/* Palindrome is a number, word, phrase or other sequence of
 * character that remains the same when its digits are reversed */
fn is_palindrome_num(n: i32) -> bool {
	let mut dup_n: i32 = n;  // duplicate of original number
	let mut rev_n: i32 = 0;  // reversed of original number
	
	// negative number is not palindrome
	while dup_n > 0 {
		let digit: i32 = dup_n % 10;  // get the smallest digit
		rev_n = (rev_n * 10) + digit; // put it into the 
		dup_n = (dup_n / 10);         // and remove it from
	}
	
	return n == rev_n;
}

fn main() {
	assert_eq!(is_palindrome_num(1221), true);
	assert_eq!(is_palindrome_num(123), false);
}

// vim:ft=rust:noet:ai:cin:nosi

