#[doc = include_str!("../README.md")]

pub use console as internal;

// TODO: WIP more powerful version of the current macro, last time i tested it doesn't work with rust-analyzer autocomplete, will finish later.

// /// Main macro for printing colored text, the examples should explain everything
// /// 
// /// # Examples
// /// ```
// /// use print_colored::*;
// /// 
// /// let num = 2;
// /// 
// /// cprint!("The" => green " number " => "is {}", num);
// /// ```
// #[macro_export]
// macro_rules! cprint {
// 	($($($prop:ident($($prop_arg:expr),* $(,)?))* $fmt:literal $(,$arg:expr)* $(,)?)=>*) => {{
// 		use std::io::Write;
// 		let mut term: $crate::internal::Term = $crate::internal::Term::stdout();
// 		$(
// 			write!(term, "{}", style(format!($fmt, $($arg)*))
// 				$(.$prop($($prop_arg)*))*
// 			).unwrap();
// 		)*
// 	}};
// }

/// Main macro for printing colored text
/// 
/// # Examples
/// ```
/// use print_colored::*;
/// 
/// let num = 2;
/// cprint!(black on_white strikethrough "The number is {}\n", num);
/// ```
#[macro_export]
macro_rules! cprint {
	($($prop:ident $(($($prop_arg:expr),* $(,)?))? )* $fmt:literal $(,$arg:expr)* $(,)?) => {{
		use std::io::Write;
		let mut term: $crate::internal::Term = $crate::internal::Term::stdout();
		term.write($crate::internal::style(format!($fmt $(,$arg)*))
			$(.$prop($($($prop_arg)*)?))*
		.to_string().as_bytes()).unwrap();
	}};
}

/// Main macro for printing colored text
/// 
/// # Examples
/// ```
/// use print_colored::*;
/// 
/// let num = 2;
/// cprintln!(black on_white strikethrough "The number is {}", num);
/// ```
#[macro_export]
macro_rules! cprintln {
	($($prop:ident $(($($prop_arg:expr),* $(,)?))? )* $fmt:literal $(,$arg:expr)* $(,)?) => {{
		use std::io::Write;
		let mut term: $crate::internal::Term = $crate::internal::Term::stdout();
		term.write_line(&$crate::internal::style(format!($fmt $(,$arg)*))
			$(.$prop($($($prop_arg)*)?))*
		.to_string()).unwrap();
	}};
}