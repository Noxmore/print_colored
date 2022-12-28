//! A small crate used to print out colored text.
//! 
//! ## Formatting
//! 
//! If you put the color formatting character in your input, followed by a formatting code, the text after that point will be formatted with that color.
//! 
//! The default formatting color character is `'§'` (if you have a num-pad type right-alt-down > 0167 > right-alt-up) but you can use an alternative character with `cprintln_with!`.
//! 
//! A formatting code is either an 'r' for resetting the color, or 2 hex digits, regarding the foreground and background colors respectively. The case of the formatting code does not matter.
//! 
//! ### Digits
//! 0 = Black, 1 = Dark Blue, 2 = Dark Green, 3 = Dark Cyan, 4 = Dark Red, 5 = Dark Magenta, 6 = Gold, 
//! 7 = Gray, 8 = Dark Gray, 9 = Blue, a = Green, b = Cyan, c = Red, d = Magenta, e = Yellow, f = White
//! 
//! ### Attributes
//! A formatting code can also be prefixed with attribute characters, the list of which are defined below.
//! 
//! L = Bold, M = Crossed Out, U = Underlined, I = Italic
//! 
//! ## Examples
//! 
//! ```
//! use print_colored::*;
//! 
//! fn main()
//! {
//! 	cprintln!("§ub0This is blue against black, underlined! §rAnd this is the terminal's default color!");
//! 
//! 	let foo = 32;
//! 	cprintln!("§07This is Black against Gray! §rThis macro also supports arguments like these: {foo}");
//! }
//! ```

use std::{io::stdout, collections::HashMap, fmt::{self}};

use crossterm::{style::*, execute};

pub const DEFAULT_COLOR_CHAR: char = '§';

/// Prints something out with the specified formatting.
/// 
/// ## Formatting
/// 
/// If you put the color formatting character in your input, followed by a formatting code, the text after that point will be formatted with that color.
/// 
/// The default formatting color character is `'§'` (if you have a num-pad type right-alt-down > 0167 > right-alt-up) but you can use an alternative character with `cprintln_with!`.
/// 
/// A formatting code is either an 'r' for resetting the color, or 2 hex digits, regarding the foreground and background colors respectively. The case of the formatting code does not matter.
/// 
/// ### Digits
/// 0 = Black, 1 = Dark Blue, 2 = Dark Green, 3 = Dark Cyan, 4 = Dark Red, 5 = Dark Magenta, 6 = Gold, 
/// 7 = Gray, 8 = Dark Gray, 9 = Blue, a = Green, b = Cyan, c = Red, d = Magenta, e = Yellow, f = White
/// 
/// ### Attributes
/// A formatting code can also be prefixed with attribute characters, the list of which are defined below.
/// 
/// L = Bold, M = Crossed Out, U = Underlined, I = Italic
/// 
/// ## Examples
/// 
/// ```
/// use print_colored::*;
/// 
/// fn main()
/// {
/// 	cprint!("§ub0This is blue against black, underlined! §rAnd this is the terminal's default color!\n");
/// 
/// 	let foo = 32;
/// 	cprint!("§07This is Black against Gray! §rThis macro also supports arguments like these: {foo}\n");
/// }
/// ```
#[macro_export]
macro_rules! cprint
{
	($($arg:tt)*) => {{
		$crate::_print_colored($crate::DEFAULT_COLOR_CHAR, std::format_args!($($arg)*), $crate::get_color_map(), $crate::get_attribute_map(), false);
	}};
}

/// Like `cprint!` but with a newline character at the end.
#[macro_export]
macro_rules! cprintln
{
	($($arg:tt)*) => {{
		$crate::_print_colored($crate::DEFAULT_COLOR_CHAR, std::format_args!($($arg)*), $crate::get_color_map(), $crate::get_attribute_map(), true);
	}};
}

/// Like `cprint!` but with the color formatting character specified.
/// 
/// ## Examples
/// 
/// ```
/// use print_colored::*;
/// 
/// fn main()
/// {
/// 	cprint_with!('&', "&c0This &ris red!");
/// }
/// ```
#[macro_export]
macro_rules! cprint_with
{
	($chr:expr, $($arg:tt)*) => {{
		$crate::_print_colored($chr, std::format_args!($($arg)*), $crate::get_color_map(), $crate::get_attribute_map(), false);
	}};
}

/// Like `cprint_with!` but with a newline character at the end.
#[macro_export]
macro_rules! cprintln_with
{
	($chr:expr, $($arg:tt)*) => {{
		$crate::_print_colored($chr, std::format_args!($($arg)*), $crate::get_color_map(), $crate::get_attribute_map(), true);
	}};
}

/// Internal function to print, it is recommended to use the `cprint!` or `cprintln!` macro instead of calling this.
pub fn _print_colored(color_char: char, args: fmt::Arguments<'_>, color_map: HashMap<char, Color>, attribute_map: HashMap<char, Attribute>, add_newline: bool)
{
	//let mut input = String::new();
	//input.write_fmt(args).ok();
	let mut input = fmt::format(args);
	if add_newline { input += "\n"; }
	
	//let color_map = get_color_map();
	//let attribute_map = get_attribute_map();

	let mut output = String::new();
	let mut chars = input.chars();
	let mut current_fg = Color::Grey;
	let mut current_bg = Color::Black;
	let mut attributes = Vec::new();
	let mut reset_color = false;

	while let Some(chr) = chars.next()
	{
		if chr == color_char
		{
			print(output, current_fg, current_bg, &attributes, &mut reset_color);
			output = String::new();

			loop
			{
				let fg = chars.next();
				// Reset color?
				if let Some(chr) = &fg
				{
					let chr = chr.to_ascii_lowercase();
	
					if chr == 'r'
					{
						reset_color = true;
						attributes.clear();
						break;
					}
					else if let Some(attribute) = attribute_map.get(&chr)
					{
						attributes.push(attribute.clone());
						continue;
					}
				}
	
				let bg = chars.next();
				
				if let Some(fg) = fg { if let Some(color) = color_map.get(&fg.to_ascii_lowercase()) { current_fg = color.clone() }}
				if let Some(bg) = bg { if let Some(color) = color_map.get(&bg.to_ascii_lowercase()) { current_bg = color.clone() }}

				break;
			}
		}

		else { output += &chr.to_string(); }
	}

	print(output, current_fg, current_bg, &attributes, &mut reset_color);
}


fn print(output: String, fg: Color, bg: Color, attributes: &Vec<Attribute>, reset: &mut bool)
{
	//println!("{reset}");
	if *reset
	{
		execute!
		(
			stdout(),
			ResetColor,
			Print(output),
		).unwrap();
	}
	else
	{
		execute!(stdout(), SetAttributes(Attributes::from(&attributes[..]))).unwrap();
		execute!
		(
			stdout(),
			//: WHY IS THIS BROKEN?????
			SetColors(Colors::new(fg, bg)),
			Print(output),
			ResetColor,
		).unwrap();
	}

	*reset = false;
}

/// Internal function that provides the char to color map.
pub fn get_color_map() -> HashMap<char, Color>
{
	HashMap::from
	([
		('0', Color::Black),
		('1', Color::DarkBlue),
		('2', Color::DarkGreen),
		('3', Color::DarkCyan),
		('4', Color::DarkRed),
		('5', Color::DarkMagenta),
		('6', Color::DarkYellow),
		('7', Color::Grey),
		('8', Color::DarkGrey),
		('9', Color::Blue),
		('a', Color::Green),
		('b', Color::Cyan),
		('c', Color::Red),
		('d', Color::Magenta),
		('e', Color::Yellow),
		('f', Color::White),
	])
}

/// Internal function that provides the char to attribute map.
pub fn get_attribute_map() -> HashMap<char, Attribute>
{
	HashMap::from
	([
		('l', Attribute::Bold),
		('m', Attribute::CrossedOut),
		('u', Attribute::Underlined),
		('i', Attribute::Italic),
	])
}