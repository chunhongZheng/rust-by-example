pub(crate) mod debug;  //外部可以引用到
mod display;  //外部引用不到

use crate::hello_world::format_print::debug::print_debug;
use crate::hello_world::format_print::display::display_fn;  //引进子包中的打印函数 print_debug 正确语法

///format!: write formatted text to String
///
///print!: same as format! but the text is printed to the console (io::stdout).
///
///println!: same as print! but a newline is appended.
///
///eprint!: same as format! but the text is printed to the standard error (io::stderr).
///
///eprintln!: same as eprint!but a newline is appended.
pub fn format_print_fn(){
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);
    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    //参数 0，1，2 分别为Alice Bob,caspar
    println!("{0}, this is {1}. {1}, this is {0}. this is {2} coding", "Alice", "Bob","caspar");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    // Special formatting can be specified after a `:`.    指定特定格式
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);   // {:b}指二进制显示
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    // 指定宽度右对齐文本
    //width=2，number=1，显示为 1，表示文本宽度为2，右对齐
    println!("{number:>width$}", number=1, width=2);   //输出为 1

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number=1, width=6); //输出为000001

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");  //报错
    // FIXME ^ Add the missing argument: "James"
    println!("My name is {0}, {1} {0}", "Bond","James"); //正确

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3)); //编译报错
    // FIXME ^ Comment out this line.

    print_debug();
    display_fn();
}