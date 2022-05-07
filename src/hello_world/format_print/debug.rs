//所有想要使用 std::fmt 格式化特征的类型都需要实现可打印。 仅为 std 库中的类型提供自动实现。 所有其他必须以某种方式手动实现。
//fmt::Debug trait 让这变得非常简单。 所有类型都可以派生（自动创建） fmt::Debug 实现。 对于必须手动实现的 fmt::Display 而言，情况并非如此。

// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


//所有标准库类型也可以使用 {:?} 自动打印
pub fn print_debug(){
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    //"Christian" "Slater" is the "actor's" name.  打印结果
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
   //Christian Slater is the actor's name.  打印结果
    println!("{1} {0} is the {2} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    //Now Structure(3) will print!   打印结果
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how    `derive` 的问题是无法控制如何
    // the results look. What if I want this to just show a `7`?  结果看起来。 如果我想让它只显示一个“7”怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));//Now Deep(Structure(7)) will print!  打印结果
    //所以 fmt::Debug 绝对可以打印，但牺牲了一些优雅。 Rust 还通过 {:#?} 提供“漂亮的打印”。

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print   漂亮的打印效果
    // Person {
    //     name: "Peter",
    //     age: 27,
    // }
    println!("{:#?}", peter);
    //可以手动实现 fmt::Display 来控制显示
}