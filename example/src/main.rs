fn main() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let val = 4;
    let reference = &val;

    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        //    |
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        val => println!("Got a value via destructuring: {:?}", val),
    }
}
