# Rust Basic Notes

## Toolchain

### Installation

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Cargo

#### Cargo Basic Commands

```bash
cargo new hello_world
cargo run
cargo build
cargo run --release
cargo build --release
cargo check
cargo generate-lockfile
```

```bash
cargo fmt --check
cargo clippy
cargo test
```

```bash
cargo install cargo-edit
cargo install cargo-release
cargo install cargo-tarpaulin
cargo install cargo-watch
cargo install cargo-workspaces
```

Cargo release configuration:

```toml
[workspace.metadata.release]
# cargo install cargo-release
# cargo release -x
sign-commit = true
sign-tag = true
release = false
push = false
publish = false
shared-version = true
pre-release-commit-message = "chore(release): {{version}}"
post-release-commit-message = "chore(release): {{version}}"
tag-message = "{{tag_name}}"
```

#### Cargo Cache

`~/.cargo/`:

- `config.toml`: global configuration.
- `credentials.toml`: `cargo login` related file.
- `.crates.toml`/`.crates2.json`: installed package information.
- `bin/`: installed binaries.
- `git/`: installed rust git repositories.
  - `git/db/`: installed git repositories.
  - `git/checkouts/`: branches of git repositories.
- `registry/`: `crates.io` metadata and packages.
  - `registry/index/`: metadata git repository.
  - `registry/cache/`: dependencies cache (`.crate` gzip files).
  - `registry/src/`: package source files.

#### Cargo Configuration

`Cargo.toml`:

- `cargo-features`: 只能用于 `nightly`版本的 `feature`.
- `[package]`: 定义项目( `package` )的元信息.
  - `name`: 名称.
  - `version`: 版本.
  - `authors`: 开发作者.
  - `edition`: Rust edition..
  - `rust-version`: 支持的最小化 Rust 版本.
  - `description`: 描述.
  - `documentation`: 文档 URL.
  - `readme`: README 文件的路径.
  - `homepage`: 主页 URL.
  - `repository`: 源代码仓库的 URL.
  - `license`: 开源协议 License..
  - `license-file`: License 文件的路径..
  - `keywords`: 项目的关键词.
  - `categories`: 项目分类.
  - `workspace`: 工作空间 workspace 的路径.
  - `build`: 构建脚本的路径.
  - `links`: 本地链接库的名称.
  - `exclude`: 发布时排除的文件.
  - `include`: 发布时包含的文件.
  - `publish`: 用于阻止项目的发布.
  - `metadata`: 额外的配置信息，用于提供给外部工具.
  - `default-run`: [`cargo run`] 所使用的默认可执行文件( binary ).
  - `autobins`: 禁止可执行文件的自动发现.
  - `autoexamples`: 禁止示例文件的自动发现.
  - `autotests`: 禁止测试文件的自动发现.
  - `autobenches`: 禁止 bench 文件的自动发现.
  - `resolver`: 设置依赖解析器( dependency resolver).
- Cargo target configuration:
  - `[lib]`: Library target.
  - `[[bin]]`: Binary target.
  - `[[example]]`: Example target.
  - `[[test]]`: Test target.
  - `[[bench]]`: Benchmark target.
- Dependency tables:
  - `[dependencies]`: 项目依赖包.
  - `[dev-dependencies]`:
    用于 examples、tests 和 benchmarks 的依赖包.
  - `[build-dependencies]`: 用于构建脚本的依赖包.
  - `[target]`: 平台特定的依赖包.
- `[badges]`: 维护状态.
- `[features]`: `features` 可以用于条件编译.
- `[patch]`: 推荐使用的依赖覆盖方式.
- `[profile]`: 编译器设置和优化.
- `[workspace]`: 工作空间的定义.

### GitHub Action

- Use [tool](https://github.com/mozilla/sccache) to speed up compilation.

```yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - uses: actions-rs/cargo@v1
        with:
          command: test
          args: --all-features --workspace

  rustfmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
          components: rustfmt
      - name: Check formatting
        uses: actions-rs/cargo@v1
        with:
          command: fmt
          args: --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
          components: clippy
      - name: Clippy check
        uses: actions-rs/cargo@v1
        with:
          command: clippy
          args: --all-targets --all-features --workspace -- -D warnings

  docs:
    name: Docs
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - name: Check documentation
        env:
          RUSTDOCFLAGS: -D warnings
        uses: actions-rs/cargo@v1
        with:
          command: doc
          args: --no-deps --document-private-items --all-features --workspace

  publish-dry-run:
    name: Publish dry run
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - uses: actions-rs/cargo@v1
        with:
          command: publish
          args: --dry-run

  coverage:
    name: Code coverage
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v2
      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - name: Run cargo-tarpaulin
        uses: actions-rs/tarpaulin@v0.1
        with:
          args: --all-features --workspace --ignore-tests --out Lcov
      - name: Upload to Coveralls
        if: ${{ github.event_name == 'push' }}
        uses: coverallsapp/github-action@v1
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          path-to-lcov: ./lcov.info
```

## Ownership

### Copy Trait

Copyable type (implement `Copy` trait):

- Integer type.
- Bool type.
- Float type.
- Char type.
- Copyable Tuple type, e.g `(i32, i32)`.
- Reference type (**borrowing** ownership).

Most these types store on stack
(including reference type with vtable).

```rust
fn main() {
    // Primitive type.
    let a = 5;
    let b = a;

    // Reference type.
    let x: &str = "hello, world";
    let y = x;

    // Deep clone on `non-Copy` type.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // Correct.
    println!("a = {}, b = {}", a, b);
    println!("x = {}, y = {}", x, y);
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Error[E0382]: use of moved value: `s1`.
    // Move occurs because `s1` has type `std::string::String`,
    // which does not implement the `Copy` trait.
    println!("{}, world!", s1);
}
```

### Reference Type

Borrowing ownership with reference type:

- At same time, only one mutable reference or multiple immutable reference.
- Reference should be valid (rustc will report dangling reference error).

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // Leave function without drop `s`,
    // due to `s` not owner string.
}
```

Mutable reference:

- Only one mutable reference for a value in a scope).
- Can't mutable borrow an already immutable borrowed value.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

```rust
fn main() {
   let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    // Error.
    println!("{}, {} and {}", r1, r2, r3);
    // End of r1 and r2 borrowing.

    // Correct.
    let r4 = &mut s;
    println!("{}", r4);
}
```

## String Type

`&str` string slice reference type:

- Borrowing type.
- UTF-8 encode (1 ~ 4 bytes).
- String literal is `&str` type.

```rust
let s = String::from("hello world");
let len = s.len();

let hello = &s[0..5];
let world = &s[6..11];
let slice1 = &s[0..2];
let slice2 = &s[..2];
let slice3 = &s[4..len];
let slice4 = &s[4..];
let slice5 = &s[0..len];
let slice6 = &s[..];
```

`String` type:

- Ownership type.
- UTF-8 encode (1 ~ 4 bytes).

```rust
fn main() {
    let mut s = String::new();
    s.push_str("hello,world");
    s.push('!');
    assert_eq!(s,"hello,world!");

    let mut s = "hello,world".to_string();
    s.push('!');
    assert_eq!(s,"hello,world!");

    let mut s = String::from("你好, 世界");
    s.push('!');
    assert_eq!(s,"你好, 世界!");

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");

    for c in "中国人".chars() {
        println!("{}", c);
    }
}
```

## Struct Type

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("username123"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

### Tuple Struct

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

`newtype`: Wrap type into tuple struct:

- Make code more readable.
- Implement 3rd traits for 3rd types.
- Hide internal details of types.

```rust
struct Meters(u32);
```

### Unit-like Struct

```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
impl SomeTrait for AlwaysEqual {}
```

## Enum Type

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x: 1, y: 1};
    let m3 = Message::ChangeColor(255, 255, 0);
}
```

```rust
enum Option<T> {
    Some(T),
    None,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## Array Type

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = [3; 5];
let slice: &[i32] = &a[1..3];
assert_eq!(slice, &[2, 3]);

fn main() {
  let one             = [1, 2, 3];
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  for a in &arrays {
    print!("{:?}: ", a);

    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;

    for i in 0..a.len() {
      sum += a[i];
    }

    println!("\t({:?} = {})", a, sum);
  }
}
```

## Type Alias

```rust
type Meters = i32;

let x: u32 = 5;
let y: Meters = 5;

println!("x + y = {}", x + y);
```

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));
fn takes_long_type(f: Thunk) {}
fn returns_long_type() -> Thunk {}
```

## Type Conversion

### From Trait

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```

```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```

### Explicit Type Conversion

```rust
fn main() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{}, {}, {}", a, b, c)

    let x: i16 = 1500;
    let x_: u8 = match x.try_into() {
        Ok(x1) => x1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
}
```

### Implicit Type Conversion

`target.method()`:

1. Call by value: `T::method(target)`.
2. Call by reference: `T::method(&target)` or `T::method(&mut target)`.
3. Call by deref: when `T: Deref<Target = U>`, then `(&T).method() => (&U).method()`.
4. Length-non-determined collection to length-determined slice.
5. Panic.

```rust
let array: Rc<Box<[T; 3]>> = ...;
let first_entry = array[0];
// 1. `Index` trait grammar sugar: array[0] => array.index(0).
// 2. Call by: value: `Rc<Box<[T; 3]>>` not impl `Index` trait.
// 3. Call by reference: `&Rc<Box<[T; 3]>>` not impl `Index` trait.
// 4. Call by reference: `&mut Rc<Box<[T; 3]>>` not impl `Index` trait.
// 5. Call by deref -> Call by value: `Box<[T; 3]>` not impl `Index` trait.
// 6. Call by deref -> Call by reference: `&Box<[T; 3]>` not impl `Index` trait.
// 7. Call by deref -> Call by reference: `&mut Box<[T; 3]>` not impl `Index` trait.
// 8. Call by deref -> Call by deref: `[T; 3]` not impl `Index` trait.
// 9. `[T; 3]` => `[T]` impl `Index` trait.
```

## Dynamically Sized Type

DST:

- DST 无法单独被使用, 必须要通过 `&`/`Box`/`Rc` 来间接使用.
- `str`, `[T]`, `dyn Trait`.

```rust
// Error!
let s1: str = "Hello there!";
let s2: str = "How's it going?";

// Ok.
let s3: &str = "on?"
let s4: Box<str> = "Hello there!".into();
```

```rust
// Error!
fn my_function(n: usize) {
    let array = [123; n];
}
```

```rust
fn foobar_1(thing: &dyn MyThing) {}     // OK.
fn foobar_2(thing: Box<dyn MyThing>) {} // OK.
fn foobar_3(thing: Rc<dyn MyThing>) {}  // OK.
fn foobar_4(thing: MyThing) {}          // ERROR!
```

### Sized Trait

Implicit sized trait:

```rust
fn generic<T>(t: T) {}
// Auto-transform to by Rust compiler
fn generic<T: Sized>(t: T) {}
```

Dynamic sized generics:

```rust
fn generic<T: ?Sized>(t: &T) {}
```

## Flow Control

### If Statement

`if` expression:

```rust
let number = if condition {
    5
} else {
    6
};
```

`if let` expression:

```rust
let o = Some(3);
let v = if let Some(x) = o {
    x
} else {
    0
};
```

### Loop Statement

#### For Loop Statement

```rust
for i in 1..=5 {}
for _ in 0..10 {}
for item in collection {}
for item in &collection {}
for item in &mut collection {}
for (i, v) in collection.iter().enumerate() {}
```

#### While Loop Statement

```rust
fn main() {
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);
        n = n + 1;
    }
}
```

#### Loop Expression

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

## Pattern Matching

```rust
match target {
    pattern1 => expression1,
    pattern2 => {
        statement1;
        statement2;
        expression2
    },
    _ => expression3
}

if let pattern = target {
    statement;
    expression
}

while let pattern = target {
    statement;
}
```

### Enum Pattern Matching

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
```

### Tuple Pattern Matching

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

### Struct Pattern Matching

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point { x: 0, y: 7, z: 0 };
    let Point { x: a, y: b, z: c } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(0, c);

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
```

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

### Match Guard

Combine pattern matching and `if` expression:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### Match Assignment

Combine pattern matching and `@` expression:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
```

## Method

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### Self

- `self`: 所有权转移.
- `&self`: 不可变借用.
- `&mut self`: 可变借用.

```rust
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
    pub fn height(&self) -> u32 {
        return self.height;
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    println!("{}", rect.width());
    println!("{}", rect.height());
}
```

## Generics

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<U>(self, other: Point<U>) {}
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn add<T: std::ops::Add<T, Output = T>>(a:T, b:T) -> T {
    a + b
}

fn largest<T: PartialOrd>(list: &[T]) -> T {}
```

- TurboFish:
  - `generics_struct::<T>::method()`.
  - `struct.generics_method::<T>()`.
- Use associated types in traits.

## Traits

```rust
pub struct Post {
    pub username: String,
    pub content: String
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let post = Post{username: "username".to_string(),content: "content".to_string()};
    println!("1 new post: {}", post.summarize());
}
```

### Orphan Rule

Rust can’t implement external traits on external types:
can’t implement the `Display` trait on `Vec<T>` in `some_package` crate,
because `Display` and `Vec<T>` are **both** defined out of `some_package`.
This restriction is part of a property of programs called coherence,
ensures that other people’s code can’t break your code and vice versa.

### Trait Bound

```rust
fn notify(item: &impl Summary) {}
fn notify(item: &(impl Summary + Display)) {}
fn notify<T: Summary>(item: &T) {}
fn notify<T: Summary + Display>(item: &T) {}
fn notify<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

```rust
trait SomeTrait: BoundTrait {}
```

```rust
// 可以对任何实现了 Display 特征的类型调用 ToString 特征中方法.
impl<T: Display> ToString for T {}
```

### Trait Derive

```rust
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
#[derive(Default)]
```

```rust
trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
```

### Trait Object

- Define trait object:
  - `Box<dyn some_trait>`.
  - `&dyn some_trait`.
- A trait can have trait object only when
  it is `object safe`:
  - all methods can't return `Self`.
  - all methods can't be generics.
- Trait object has `'static` lifetime.
- Trait object stand for dynamic distributing (runtime),
  generics stand for static distributing (compile time).

```rust
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;

    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}
```

### Associated Types

Associated types make code become readable and concise:

```rust
trait Container<A,B> {
    fn contains(&self,a: A,b: B) -> bool;
}

fn difference<A,B,C>(container: &C) -> i32
  where
    C : Container<A,B> {}
```

```rust
trait Container{
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

fn difference<C: Container>(container: &C) {}
```

For all **generic trait**,
use associated types better than `<T>`.

### Common Traits

- `std::fmt::Display` (better than `std::string::ToString`).
- `std::fmt::Debug`.
- `std::ops::Add`/`Mul`/`Div`/`BitAnd`/`BitOr`/`Not`/`Neg`: operators overload.
- `std::ops::Fn`/`FnMut`/`FnOnce`.
- `std::ops::Deref`.
- `std::ops::Drop`.
- `std::clone::Clone`.
- `std::iter::Iterator`.

`std::prelude`:

- `std::marker::{Copy, Send, Sized, Sync, Unpin}`.
- `std::ops::{Drop, Fn, FnMut, FnOnce}`.
- `std::mem::drop`.
- `std::boxed::Box`.
- `std::borrow::ToOwned`.
- `std::clone::Clone`.
- `std::cmp::{PartialEq, PartialOrd, Eq, Ord}`.
- `std::convert::{AsRef, AsMut, Into, From}`.
- `std::default::Default`.
- `std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}`.
- `std::option::Option::{self, Some, None}`.
- `std::result::Result::{self, Ok, Err}`.
- `std::string::{String, ToString}`.
- `std::vec::Vec`.
- `std::convert::{TryFrom, TryInto}`.
- `std::iter::FromIterator`.

```rust
use std::io::prelude::*;
```

## Collection

### Vector

Create and Insert:

```rust
let mut v = Vec::new();
v.push(1);
```

Access and Get:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("3rd: {}", third);

match v.get(2) {
    Some(third) => println!("3rd: {}", third),
    None => println!("None."),
}
```

Visit:

```rust
let v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
}
```

Visit and Mutate:

```rust
let mut v = vec![1, 2, 3];

for i in &mut v {
    *i += 10
}
```

Store different types:

```rust
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}
```

### HashMap

```rust
use std::collections::HashMap;
```

```rust
// Create.
let mut scores = HashMap::new();

// Insert.
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
scores.entry("Red").or_insert(5);

// Get.
let team_name = String::from("Blue");
let score: Option<&i32> = scores.get(&team_name);

// Visit
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Transform.
let from_list: HashMap<_,_> = some_list.into_iter().collect();
```

### HashSet

- insert.
- contains.
- union.
- difference.
- intersection.
- symmetric_difference.

## Error Handling

### Result Type

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

### Result Type Compositor

- `or`: logic or.
- `and`: logic and.
- `or_else`: logic or function.
- `and_then`: logic and function.
- `filter`: `Option` filter function.
- `map`: `Ok`/`Some` map function.
- `map_or`: `Ok`/`Some` map function with defaults value.
- `map_or_else`: `Ok`/`Some` map function with defaults function.
- `map_err`: `Err` map function.
- `ok_or`: `Option` -> `Result` with error message.
- `ok_or_else`: `Option` -> `Result` with error message function.

### Error Handling Macro

`?` for `Result` type:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

`?` for `Option` type:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

### Error Trait

#### Standard Error Trait

```rust
use std::fmt::{Debug, Display};

pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(Error + 'static)> { ... }
}
```

```rust
use std::fs::read_to_string;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String, Box<dyn Error>> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}
```

#### Custom Error Type

```rust
use std::error;
use std::fmt;

#[derive(Debug)]
struct AppError;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!")
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        "Invalid App"
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError)
}

fn main(){
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err({ file: src/main.rs, line: 17 })
}
```

#### Convert From Standard Error

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    let mut file = File::open("hello_world.txt")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let _number: usize;
    _number = content.parse()?;

    Ok(())
}


// --------------- 上述代码运行后的可能输出 ---------------
// 01. 若 hello_world.txt 文件不存在
// Error: AppError { kind: "io", message: "No such file or directory" }
// 02. 若用户没有相关的权限访问 hello_world.txt
// Error: AppError { kind: "io", message: "Permission denied" }
// 03. 若 hello_world.txt 包含有非数字的内容，例如 Hello, world!
// Error: AppError { kind: "parse", message: "invalid digit found in string" }
```

## Format Print

### Format Print Macros

- `print!`.
- `println!`.
- `eprint!`.
- `eprintln!`.
- `format!`.

```rust
println!("Hello");                 // => "Hello"
println!("Hello, {}!", "world");   // => "Hello, world!"
println!("The number is {}", 1);   // => "The number is 1"
println!("{:?}", (3, 4));          // => "(3, 4)"
println!("{value}", value=4);      // => "4"
println!("{} {}", 1, 2);           // => "1 2"
println!("{:04}", 42);             // => "0042" with leading zeros
```

```rust
fn main() {
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
}
```

### Format Print Placeholder

- `{}`.
- `{:?}`.
- `{:#?}`.
- Index placeholder.
- Alias placeholder.

```rust
fn main() {
    println!("{1}{}{0}{}", 1, 2); // => 2112
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
}
```

- Pad placeholder.

```rust
fn main() {
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格, 补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容, 同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);

    // 使用参数5为参数x指定宽度, 同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);

    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5, 使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
}
```

- Alignment placeholder.

```rust
fn main() {
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");
}
```

- Precision placeholder.

```rust
fn main() {
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416, 相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hello I'm some one";
    // 保留字符串前三个字符 => hel
    println!("{:.3}", s);
    // {:.*} 接收两个参数, 第一个是精度, 第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");
}
```

- Radix placeholder: `boxXeE`.
  - `fmt::Binary` trait.
  - `fmt::Octal` trait.
  - `fmt::LowerHex` trait.
  - `fmt::UpperHex` trait.
  - `fmt::LowerExp` trait.
  - `fmt::UpperExp` trait.

```rust
fn main() {
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制, 宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);

    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9
}
```

### Debug Trait

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person{name: "name".to_string(), age: 18};
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}
```

### Display Trait

```rust
use std::fmt;

struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "My name is {}, {} year old.",
            self.name, self.age
        )
    }
}

fn main() {
    let p = Person {
        name: "name".to_string(),
        age: 18,
    };

    println!("{}", p);
}
```

## Lifetime

显式地使用生命周期, 可以让编译器正确地认识到多个**引用**之间的关系.

```rust
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用
```

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Function Lifetime

函数或者方法中,
参数的生命周期被称为`输入生命周期`,
返回值的生命周期被称为`输出生命周期`:

- 每一个引用参数都会获得独自的生命周期:
  `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`.
- 若只有一个输入生命周期, 则该生命周期会被赋给所有输出生命周期.
- 若存在多个输入生命周期, 且其中一个是`&self`/`&mut self`,
  则`&self`生命周期被赋给所有输出生命周期 (除非显式地声明输出生命周期).

### Static Lifetime

生命周期`'static`表示持续整个程序,
例如字符串字面量和特征对象.

### Lifetime Constraint

- `'a: 'b`: `'a` 生命周期更长.
- `T: 'a`: `T` 生命周期更长.

## Closure

### Function Parameter Closure

改变捕获变量的所有权 (FnOnce):

```rust
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
```

可变借用捕获 (FnMut):

```rust
fn main() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}
```

不可变借用捕获 (Fn):

```rust
fn main() {
    let s = "hello, ".to_string();

    let update_string =  |str| println!("{},{}",s,str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: Fn(String) -> ()>(f: F)  {
    f("world".to_string())
}
```

- 所有闭包都自动实现了 `FnOnce` 特征, 因此任何一个闭包都至少可以被调用一次.
- 没有移出所捕获变量的所有权的闭包自动实现了 `FnMut` 特征.
- 不需要对捕获变量进行改变的闭包自动实现了 `Fn` 特征.

### Function Return Closure

```rust
fn factory() -> impl Fn(i32) -> i32 {
    let num = 5;
    |x| x + num
}
```

```rust
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
```

## Iterator

```rust
let arr = [1, 2, 3];

for v in arr.into_iter() {
    println!("{}", v);
}
```

```rust
fn main() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);
}
```

### Iterator Trait

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}
```

- `iter`: 不可变借用.
- `iter_mut`: 可变借用.
- `into_iter`: 改变所有权.

```rust
fn iter(&self) -> Iter             // Iter implements Iterator<Item = &U>
fn iter_mut(&mut self) -> IterMut  // IterMut implements Iterator<Item = &mut U>
fn into_iter(self) -> IntoIter     // IntoIter implements Iterator<Item = U>
```

```rust
fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错.
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错.
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用.
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素, 并修改为0.
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出 [0, 2, 3].
    println!("{:?}", values);
}
```

Implement iterator:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
```

### Adapter Methods

- 消费性适配器: 获取迭代器的所有权, 并消耗迭代器中所有元素.
  - `collect::<T>()`.
  - fold.
  - partition.
  - `sum::<T>()`.
- 迭代性适配器: 惰性方法 (Lazy Iterator)
  - enumerate.
  - filter.
  - filter_map.
  - map.
  - take_while.
  - zip.
- Ordinary iterator methods:
  - Iterator::any.
  - Iterator::find.

More adapter methods see `Iterator` trait
[documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html).

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // v1_iter 是借用了 v1, 因此 v1 可以照常使用.
    println!("{:?}",v1);

    // 以下代码会报错, 因为 `sum` 拿到了迭代器 `v1_iter` 的所有权.
    // println!("{:?}",v1_iter);
}
```

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
```

```rust
use std::collections::HashMap;

fn main() {
    let names = ["name1", "name2"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}",folks);
}
```

## Smart Pointer

### Box Type

`Box<T>` 将一个值分配到堆上, 然后在栈上保留一个智能指针指向堆上数据:

- 实现转移所有权时的零拷贝.
- 将不定长类型转换为定长类型.

```rust
fn main() {
    // 在栈上创建一个长度为 1000 的数组.
    let arr = [0;1000];
    // 将 arr 所有权转移 arr1, 由于 `arr` 分配在栈上, 因此直接重新深拷贝了一份数据.
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组, 因此不会报错.
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为 1000 的数组, 然后使用一个智能指针指向它.
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1, 由于数据在堆上, 因此仅仅拷贝了智能指针的结构体, 底层数据并没有被拷贝.
    // 所有权顺利转移给 arr1, arr 不再拥有所有权.
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权, 因此下面代码将报错.
    // println!("{:?}", arr.len());
}
```

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### Deref Trait

- `&smart_pointer`
  => `smart_pointer.defer()`.
- `*smart_pointer`
  => `*(smart_pointer.defer())`.
- `smart_pointer.method()`
  => `(&smart_pointer).method()`
  => `(smart_pointer.defer()).method()`.
- When `T: Deref<Target=U>`, then `&T => &U`.
- When `T: DerefMut<Target=U>`, then `&mut T => &mut U`.
- When `T: Deref<Target=U>`, then `&mut T => &U`.

```rust
use core::ops::{self};
use crate::str::{self, from_boxed_utf8_unchecked};
use crate::vec::Vec;

struct String {
    vec: Vec<u8>,
}

impl ops::Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = MyBox::new(5);
    assert_eq!(5, *x);
    // => *(x.deref())
    // => *(&x.0)
    // => x.0

    let s = MyBox::new(String::from("hello world"));
    display(&s);
    // => &MyBox
    // => MyBox.deref()
    // => &String
    // => String.deref()
    // => &str

    let hello_world = MyBox::new(String::from("hello, world"));
    let s1: &str = &hello_world;
    // => &MyBox<String>
    // => MyBox<String>.deref()
    // => &String
    // => String.deref()
    // => &str
    let s2: String = hello_world.to_string();
    // => MyBox<String>.to_string()
    // => (&MyBox<String>).to_string()
    // => (MyBox<String>.defer()).to_string()
    // => (&String).to_string()
    let ptr: *const u8 = hello_world.as_ptr();
    // => MyBox<String>.as_ptr()
    // => (&MyBox<String>).as_ptr()
    // => (MyBox<String>.defer()).as_ptr()
    // => (&String).as_ptr()
    // => (String.defer()).as_ptr()
    // => (&str).as_ptr()
}

fn display(s: &str) {
    println!("{}", s);
}
```

### Drop Trait

Drop order:

- 变量级别, 按照逆序的方式, 先创建的变量后 drop.
- 结构体内部, 按照顺序的方式, 结构体中的字段按照定义中的顺序依次 drop.

### Reference Counting Type

通过引用计数的方式, 允许一个数据资源在同一时刻拥有多个所有者.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a); // 复制了智能指针并增加了引用计数, 并没有克隆底层数据.
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
}
```

- `Rc`/`Arc` 是不可变引用, 无法修改它指向的值.
- `Rc<T>` 是一个智能指针, 实现了 `Deref` 特征, 可以直接使用 `T`.
- 一旦最后一个拥有者消失, 则资源会自动被回收.
- `Arc`: Atomic reference counting.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("Multiple threads walker"));

    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{}", s)
        });
    }
}
```

### Cell and RefCell Type

`Cell` for copyable type.

```rust
use std::cell::Cell;

fn main() {
    let c = Cell::new("abc");
    let one = c.get();
    c.set("xyz");
    let two = c.get();
    println!("{}, {}", one, two); // abc, xyz
}
```

```rust
use std::cell::Cell;

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;

    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
```

`RefCell` for borrowing reference:

- 实现内部可变性: 不可变值的可变借用.
  `imut_self.refcell_member.borrow_mut().changeMember()`.
- `Rc<RefCell<T>>`: 实现多个可变数据所有者.
- 实现编译期**可变借用**与**不可变借用**共存,
  但会引起运行时 `panic`.

```rust
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{}, {}", s1, s2);
}
```

通过包裹一层 `RefCell`,
将不可变借用 `&self` 的成员成为一个可变值,
然后实现修改:

```rust
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn main() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());
}
```

### Circle Reference

`Weak` 通过 `use std::rc::Weak` 引入, 具有以下特点:

- 可访问, 但没有所有权, 不增加引用计数, 不影响 drop.
- 可由 `Rc<T>` 调用 `downgrade` 方法转换成 `Weak<T>`.
- `Weak<T>` 可使用 `upgrade` 方法转换成 `Option<Rc<T>>`,
  如果资源已经被释放, 则 `Option` 的值是 `None`.
- 常用于解决循环引用的问题.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

## Phantom Type

虚类型/幽灵类型参数是一种在**运行时不出现**,
仅进行**静态编译检查**的类型参数.

```rust
use std::marker::PhantomData;

struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>,
}

struct Vec<T> {
    data: *const T,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}
```

```rust
use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

fn main() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    println!("_tuple1 == _tuple2 yields: {}",
              _tuple1 == _tuple2);

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    println!("_struct1 == _struct2 yields: {}",
              _struct1 == _struct2);
}
```

```rust
use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // 编译期错误: 类型不匹配.
    let compile_error = one_foot + one_meter;
}
```

## Concurrent Programming

### Concurrency Programming Model

| Name         | Pros                 | Cons                                   |
| ------------ | -------------------- | -------------------------------------- |
| OS Thread    | simple, native model | consistent and context switch overhead |
| Event Driven | perf model           | non-liner logic, callback hell         |
| Coroutines   | perf model           | non-system abstraction                 |
| Actor        | distributed model    | complex flow control and retry logic   |
| Async/Await  | perf, native model   | complex internal logic                 |

`OS Threads` for CPU intensive task (parallel computing),
`Async/Await` for I/O intensive task (blocking I/O).

### Threads

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

### Barrier

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move|| {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Condition Variables and Mutex

```rust
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}
```

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Read and write mutex:

- 同时允许多个读, 但最多只能有一个写.
- 读和写不能同时存在.
- 可以使用 `read`/`try_read`/`write`/`try_write`.

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读.
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // Drop.

    // 同一时间只允许一个写.
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } // Drop.
}
```

### Threads Communication

Message channel:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    // Block.
    println!("receive {}", rx.recv().unwrap());
}
```

Sync channel with message buffer:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Sync channel with 3 length buffer.
    let (tx, rx)= mpsc::sync_channel(3);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}
```

Send message via `for` loop:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

Multiple producers:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

Multiple type message:

```rust
use std::sync::mpsc::{self, Receiver, Sender};

enum Fruit {
    Apple(u8),
    Orange(String)
}

fn main() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}
```

### Tokio Semaphore

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            /**
             * Task here ...
             */
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
```

### Atomic Primitives

```rust
use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn main() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}",Instant::now().sub(s));
}
```

`Ordering` 内存顺序:

- Relaxed: 乱序.
- Release: 设置内存屏障, 保证它之前的操作永远在它之前.
- Acquire: 设置内存屏障, 保证它之后的操作永远在它之后.
- AcqRel: Acquire + Release.
- SeqCst: 顺序一致性.

```rust
use std::thread::{self, JoinHandle};
use std::sync::atomic::{Ordering, AtomicBool};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;                                 // A
        }
        READY.store(true, Ordering::Release);           // B: 内存屏障 ↑
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Acquire) {}         // C: 内存屏障 ↓

        assert_eq!(100, unsafe { DATA });               // D
    })
}


fn main() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
```

Spinlock:

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其它线程释放锁.
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
```

### Send and Sync Trait

Send and Sync:

- Marker trait.
- 实现 `Send` 的类型可以在线程间安全的传递其所有权,
  实现 `Sync` 的类型可以在线程间安全的共享 (通过引用).
  若 `&T: Send`, 则 `T: Sync`.
- 绝大部分类型都实现了 `Send`/`Sync`,
  例外: 原生指针, `Cell`/`RefCell`, `Rc`.

### Thread Pool

```rust
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

## Asynchronous Programming

### Async and Await

- 在 `.await` 执行期间, 任务可能会在线程间转移.
- `.await` 只能用于 async fn 函数中.

```rust
use futures::executor::block_on;
use futures::join;

async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
```

```rust
use futures::future;
use futures::select;

pub fn main() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => panic!(), // 该分支永远不会运行.
        };
    }

    assert_eq!(total, 10);
}
```

### Future Trait

- `Future` 代表一组计算, 惰性求值. 当 `.await` 调用时才真正开始执行.
- `Future` 启动后会因资源等原因阻塞, 转入 `pending` 状态.
- `Future` 阻塞后, 当资源准备好可以重新启动时, 会通过 `Waker.wake` 通知执行器, 等待被下一次 `poll`.

```rust
trait Future {
    type Output;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
```

```rust

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
```

当 `Future` 会返回 `Poll::Pending` 时,
一定要确保 `wake` 能被正常调用,
否则会导致任务永远被挂起,
再也不会被执行器 `poll`.

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
```

### Asynchronous Runtime

```rust
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    // 引入之前实现的定时器模块
    timer_future::TimerFuture,
};

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("任务队列已满");
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);

                if future.as_mut().poll(context).is_pending() {
                    // Future 未执行完，, 将它放回任务中, 等待下次被 poll.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);

    // 运行执行器直到任务队列为空.
    // 任务运行后, 会先打印 `howdy!`, 暂停 2 秒, 接着打印 `done!`.
    executor.run();
}
```

```rust
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use futures::task::{self, ArcWake};
use crossbeam::channel;

fn main() {
    let mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        spawn(async {
            delay(Duration::from_millis(100)).await;
            println!("world");
        });

        spawn(async {
            println!("hello");
        });

        delay(Duration::from_millis(200)).await;
        std::process::exit(0);
    });

    mini_tokio.run();
}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();

        MiniTokio { scheduled, sender }
    }

    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        CURRENT.with(|cell| {
            *cell.borrow_mut() = Some(self.sender.clone());
        });

        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    CURRENT.with(|cell| {
        let borrow = cell.borrow();
        let sender = borrow.as_ref().unwrap();
        Task::spawn(future, sender);
    });
}

async fn delay(dur: Duration) {
    struct Delay {
        when: Instant,
        waker: Option<Arc<Mutex<Waker>>>,
    }

    impl Future for Delay {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();

                if !waker.will_wake(cx.waker()) {
                    *waker = cx.waker().clone();
                }
            } else {
                let when = self.when;
                let waker = Arc::new(Mutex::new(cx.waker().clone()));
                self.waker = Some(waker.clone());

                thread::spawn(move || {
                    let now = Instant::now();

                    if now < when {
                        thread::sleep(when - now);
                    }

                    let waker = waker.lock().unwrap();
                    waker.wake_by_ref();
                });
            }

            if Instant::now() >= self.when {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    let future = Delay {
        when: Instant::now() + dur,
        waker: None,
    };

    future.await;
}

thread_local! {
    static CURRENT: RefCell<Option<channel::Sender<Arc<Task>>>> =
        RefCell::new(None);
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }

    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let _ = arc_self.executor.send(arc_self.clone());
    }
}
```

```rust
#[tokio::main]
async fn main() {
    println!("Hello world");
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        })
}
```

## Tests

```rust
fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    #[should_panic]
    #[should_panic(expected = "Panic message.")]
    fn greeting_contains_name() {
        let target = "name";
        let result = greeting("Name");
        assert!(
            result.contains(target),
            "Expect: `{}`, Result: `{}`",
            target,
            result
        );
    }
}
```

## IO

### Path

```rust
use std::path::Path;

fn main() {
    let path = Path::new(".");
    let new_path = path.join("a").join("b");

    // 将路径转换成一个字符串切片
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    // `display` 方法返回一个可显示的结构体
    let display = path.display();
}
```

### Files

- `File::open`.
- `File::create`.
- `file.read_to_string`.
- `file.write_all`.
- `bufReader.lines`.
- `OpenOptions`.
- `fs::read_dir`.
- `fs::create_dir`.
- `fs::create_dir_all`.
- `fs::remove_file`.
- `fs::remove_dir`.
- `fs::symlink`.

```rust
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
```

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

```rust
static LOREM_IPSUM: &'static str = "Words";

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
```

```rust
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`mkdir a`");
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    println!("`echo hello > a/b.txt`");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    println!("`rm a/c/e.txt`");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
```

## System

### Process

```rust
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }
}
```

```rust
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PROGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PROGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why.description()),
        Ok(_) => println!("sent program to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
```

### Command Line

```rust

use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```

## Macros

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

## Unsafe Code

`unsafe {}`:

- 对原始指针进行解引用.
- 调用`不安全`的函数 (包括 C 函数, 编译器内建指令, 原始分配器).
- 实现`不安全`的特性.
- 访问`union`字段.
- 改变静态数据.

## Comments

```rust
// Line Comments
/* Block Comments */
/// Document Line Comments
/** Document Block Comments */
//! Crate Line Comments
/*! Crate Block Comments */

/// [`Option`]
/// [`Type`](struct@Type)
/// [`Type`](fn@Type)

#[doc(alias = "alias" )]
```

## Attributes

- Crate scope:`#![crate_attribute]`.
- Module/Function scope: `#[item_attribute]`.

```rust
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
```

### Crate Attributes

```rust
#![crate_type = "lib"]
#![crate_name = "rand"]
```

### Linter Attributes

```rust
#[allow(dead_code)]
#[allow(unused)]
```

### Compile Attributes

```rust
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();

    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

## Standard Library

- `as_`: `borrowed` -> `borrowed`.
- `into_`: `owned` -> `owned` (移除所有权).
- `to_`:
  `borrowed` -> `borrowed`,
  `borrowed` -> `owned` on non-copy types,
  `owned` -> `owned` on copy types.
- `try_`: 尝试一次, 失败则返回或报错.
- `_mut`: 可变借用.

## Web Development

### Node.js Bindings

Tasks suite for native `Node.js` add-ons:

- Computing intensive tasks with simple I/O:
  e.g `@node-rs/crc32` (CPU SIMD instruction), `@node-rs/bcrypt`, `@node-rs/jieba`.
- System call tasks:
  SIMD instruction, GPU instruction.

[Napi](https://github.com/napi-rs/napi-rs):
Framework for building compiled `Node.js` add-ons in `Rust` via Node API.

```rust
#[macro_use]
extern crate napi;

/// import the preludes
use napi::bindgen_prelude::*;

/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
fn fibonacci(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// use `Fn`, `FnMut` or `FnOnce` traits to defined JavaScript callbacks
/// the return type of callbacks can only be `Result`.
#[napi]
fn get_cwd<T: Fn(String) -> Result<()>>(callback: T) {
    callback(env::current_dir().unwrap().to_string_lossy().to_string()).unwrap();
}

/// or, define the callback signature in where clause
#[napi]
fn test_callback<T>(callback: T)
where T: Fn(String) -> Result<()>
{}

/// async fn, require `async` feature enabled.
/// [dependencies]
/// napi = {version="2", features=["async"]}
#[napi]
async fn read_file_async(path: String) -> Result<Buffer> {
    tokio::fs::read(path)
        .map(|r| match r {
            Ok(content) => Ok(content.into()),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("failed to read file, {}", e),
            )),
        })
        .await
}
```

[Neon](https://github.com/neon-bindings/neon):
`Rust` bindings for safe and fast native `Node.js` modules.

```rust
use neon::context::{Context, ModuleContext, FunctionContext};
use neon::types::JsNumber;
use neon::result::JsResult;
use neon::result::NeonResult;

fn fibonacci(n: i32) -> i32 {
    return match n {
        n if n < 1 => 0,
        n if n <= 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
  }
}

fn fibonacci_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0).unwrap();
    let res = fibonacci(handle.value(&mut cx) as i32);
    Ok(cx.number(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fibonacci_rs", fibonacci_api)?;
    Ok(())
}
```

```ts
const { fibonacci_rs } = require('./index.node');

const value = process.argv[2] || null;
const number = parseInt(value);

if (isNaN(number)) {
  console.log('Provided value is not a number');
  return;
}

const result = fibonacci_rs(number);
console.log(result);
```

## Library

- [Num: Numeric Types and Traits](https://github.com/rust-num/num)
- [Rand: Random Number Generator](https://github.com/rust-random/rand)
- [Regex: Regular Expression Engine](https://github.com/rust-lang/regex)
- [Chrono: DateTime Library](https://github.com/chronotope/chrono)
- [AsyncStd: Asynchronous Version Standard Library](https://github.com/async-rs/async-std)
- [Crossbeam: Concurrent Programming Tools](https://github.com/crossbeam-rs/crossbeam)
- [Tokio: Asynchronous Runtime](https://github.com/tokio-rs/tokio)
- [Rayon: Data Parallelism Library](https://github.com/rayon-rs/rayon)
- [Log: Logging Library](https://github.com/rust-lang/log)
- [Tracing: Tracing Library](https://github.com/tokio-rs/tracing)
- [Serde: Serialization Framework](https://github.com/serde-rs/serde)
- [Axum: Tokio Web Framework](https://github.com/tokio-rs/axum)
- [Rocket: Web Framework](https://github.com/SergioBenitez/Rocket)
- [Actix: Web Framework](https://github.com/actix/actix-web)
- [Warp: Web Framework](https://github.com/seanmonstar/warp)
- [Request: HTTP Client](https://github.com/seanmonstar/reqwest)
- [Quiche: QUIC and HTTP/3 Library](https://github.com/cloudflare/quiche)
- [Tonic: gPRC Framework](https://github.com/hyperium/tonic)
- [QuickWit: Distributed Search Engine](https://github.com/quickwit-oss/quickwit)
- [MeiliSearch: Realtime Search Engine](https://github.com/meilisearch/MeiliSearch)
- [Clap: CLI Framework](https://github.com/clap-rs/clap)
- [Console: ProgressBar](https://github.com/console-rs/indicatif)
- [Syn: Source Code Parser](https://github.com/dtolnay/syn)
- [Napi: Node.js Bindings Library](https://github.com/napi-rs/napi-rs):
- [Neon: Node.js Bindings Library](https://github.com/neon-bindings/neon):
- [Git: Git Bindings Library](https://github.com/rust-lang/git2-rs)
- [PrettyAssertions](https://github.com/colin-kiegel/rust-pretty-assertions)
- [Criterion: Benchmarking Library](https://github.com/bheisler/criterion.rs)
- [Clog: Conventional Changelog](https://github.com/clog-tool/clog-cli)

## Reference

- [Rust Book](https://github.com/rust-lang/book)
- [Rust Nomicon](https://github.com/rust-lang/nomicon)
- [Rust Asynchronism](https://github.com/rust-lang/async-book)
- [Rust Data Structures](https://github.com/rust-unofficial/too-many-lists)
- [Rust Example](https://github.com/rust-lang/rust-by-example)
- [Rust Lings](https://github.com/rust-lang/rustlings)
- [Rust Course](https://github.com/sunface/rust-course)
