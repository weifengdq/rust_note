fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // 整数
    let a = 1;
    println!("{}", a);
    // 字符
    let b = 'a';
    println!("{}", b);
    // 字符串
    let s = "hello";
    println!("{}", s);
    // 布尔值
    let b = true;
    println!("{}", b);
    // 浮点数
    let f = 3.1415926;
    println!("{}", f);
    // 数组
    let arr = [10, 11, 13, 24, 50];
    println!("{:?}", arr);
    // 元组
    let tup = (1, "hello", true);
    println!("{:?}", tup);
    // 函数
    let f = add;
    println!("11 + 22 = {}", f(11, 22));
    // =================
    let c = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("{}", c);
    // lambda表达式
    let lambda = |x: i32, y: i32| x + y;
    println!("{}", lambda(3, 4));
    // 打印匿名函数
    let anon = |x: i32, y: i32| x + y;
    println!("{:?}", anon(5, 6));
    // 匿名函数作为参数传递
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(9));
    // 匿名函数作为返回值
    let f = || {
        println!("hi");
    };
    f();
    // 打印十六进制
    let hex = |x: i32| -> String {
        format!("{:02x}", x)
    };
    println!("{}", hex(255));
    // 遍历数组, 打印十六进制
    for i in arr.iter() {
        println!("{}", hex(*i));
    }
    // 排序
    let mut v = vec![10, 30, 11, 20, 4, 330, 21, 110, 5, 10, 1];
    v.sort();
    println!("{:?}", v);
}