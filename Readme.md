# 模式
- 模式是 Rust 中特殊的语法，它用来匹配类型中的结构，无论类型是简单还是复杂。
- 结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。
- 模式由如下一些内容组合而成：
    - 字面值
    - 解构的数组、枚举、结构体或者元组
    - 变量
    - 通配符
    - 占位符

## match 分支
    - ```Rust 
        match VALUE {
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
        }
    ```
    - match 表达式必须是 穷尽（exhaustive）
    - 特定的模式 _ 可以匹配所有情况,不过它从不绑定任何变量

## if let 条件表达式
- 用于编写等同于只关心一个情况的 match 语句简写的。if let 可以对应一个可选的带有代码的 else 在 if let 中的模式不匹配时运行。
- if let 表达式的缺点在于其穷尽性没有为编译器所检查，而 match 表达式则检查了

## while let 条件循环
```Rust
let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
```

## for 循环
```Rust
let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
```

## let 语句
```Rust
let PATTERN = EXPRESSION;
let x = 5;
let (x, y, z) = (1, 2, 3); // let 解构元组
let (x, y) = (1, 2, 3);  // 编译时错误

// 如果希望忽略元组中一个或多个值，也可以使用 _ 或 ..

```

## 函数参数
```Rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

# irrefutable & refutable
- [Refutability（可反驳性）: 模式是否会匹配失效](https://github.com/endruz/kulolo/blob/main/%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80/Rust/1-Rust%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80/18.%E6%A8%A1%E5%BC%8F%E4%B8%8E%E6%A8%A1%E5%BC%8F%E5%8C%B9%E9%85%8D/18.2-Refutability%EF%BC%88%E5%8F%AF%E5%8F%8D%E9%A9%B3%E6%80%A7%EF%BC%89:%20%E6%A8%A1%E5%BC%8F%E6%98%AF%E5%90%A6%E4%BC%9A%E5%8C%B9%E9%85%8D%E5%A4%B1%E6%95%88.md)
- 模式有两种形式：refutable（可反驳的） 和 irrefutable（不可反驳的）。
    - 在一些地方，模式必须是 irrefutable 的，意味着他们必须匹配所提供的任何值。在另一些情况，他们则可以是 refutable 的
    - irrefutable: 能匹配任何传递的可能值的模式被称为是不可反驳的 eg let x = 5;
    - refutable : 对某些可能的值进行匹配会失败的模式被称为是可反驳的 eg if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配

    ```Rust
     let Some(x) = some_option_value;
     // `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant

     修改为：
     if let Some(x) = some_option_value {
        println!("{}", x);
     }

     if let x = 5 {
        println!("{}", x);
    }; // this pattern will always match, so the `if let` is useless
    ```
