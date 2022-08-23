This is a prototype for how to add the declaration engine to the existing Sway compiler.

The motivating force behind adding the declaration engine is to create a mechanism that allows us to change
the inner compiler AST's and stop inlining declarations and function bodies. For example, this would directly
enable trait constraits and would open the door for recursive functions.

Take this Sway code as an example (the [trait_constraint_test](tests/harness.rs) test):

```rust
trait HandleU64 {
  fn handle_u64_fn(n: u64) -> u64;
};

struct Data {
  field_one(u8),
  field_two(u32),
};

struct Point {
  x_cord(u64),
  y_cord(u64),
};

impl HandleU64 for Data {
  fn handle_u64_fn(n: u64) -> u64 {
    return 99u64;
  }
};

impl HandleU64 for Point {
  fn handle_u64_fn(n: u64) -> u64 {
    return 222u64;
  }
};

fn call_it<T>(value: T) -> u64 where T: HandleU64 {
  return value.handle_u64_fn(75u64);
};

fn main() -> () {
  let foo: UNK = Data {
    field_one: 2u8,
    field_two: 3u32,
  };
  let bar: UNK = Point {
    x_cord: 99u64,
    y_cord: 24u64,
  };
  let apple: UNK = call_it(foo);
  let orange: UNK = call_it(bar);
};
```

If function bodies are inlined, this code is unable to be compiled, as it is not tractible to determine
the body of `value.handle_u64_fn(75u64)` inside of `call_it`.

But if instead we change the internal compiler AST to not inline function bodies and add a declaration
engine, we can perform type checking, and produce this output:

```rust
trait HandleU64 {
  fn handle_u64_fn(n: u64) -> u64;
};

struct Data {
  field_one(u8),
  field_two(u32),
};

struct Point {
  x_cord(u64),
  y_cord(u64),
};

impl HandleU64 for Data {
  fn handle_u64_fn(n: u64) -> u64 {
    return 99u64;
  }
};

impl HandleU64 for Point {
  fn handle_u64_fn(n: u64) -> u64 {
    return 222u64;
  }
};

fn call_it<Data>(value: Data) -> u64 {
  return value.handle_u64_fn(75u64);
};

fn call_it<Point>(value: Point) -> u64 {
  return value.handle_u64_fn(75u64);
};

fn main() -> () {
  let foo: Data = Data {
    field_two: 3u32,
    field_one: 2u8,
  };
  let bar: Point = Point {
    y_cord: 24u64,
    x_cord: 99u64,
  };
  let apple: u64 = call_it(foo);
  let orange: u64 = call_it(bar);
};
```
