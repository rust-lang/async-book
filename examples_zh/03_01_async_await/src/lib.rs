#![allow(unused)]
#![cfg(test)]
#![feature(async_await)]

mod async_fn_and_block_examples {
use std::future::Future;
// ANCHOR: async_fn_and_block_examples

// `foo()` 返回一个实现了 `Future<Output = u8>` 的类型.
// `foo().await` 将返回类型为 `u8` 的值.
async fn foo() -> u8 { 5 }

fn bar() -> impl Future<Output = u8> {
    // 这个 `async` 区域返回一个实现了 `Future<Output = u8>` 的类型.
    async {
        let x: u8 = foo().await;
        x + 5
    }
}
// ANCHOR_END: async_fn_and_block_examples
}

mod async_lifetimes_examples {
use std::future::Future;
// ANCHOR: lifetimes_expanded
// 这是一个 `async` 函数:
async fn foo(x: &u8) -> u8 { *x }

// 相当于这个普通函数:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
// ANCHOR_END: lifetimes_expanded

async fn borrow_x(x: &u8) -> u8 { *x }

#[cfg(feature = "never_compiled")]
// ANCHOR: static_future_with_borrow
fn bad() -> impl Future<Output = u8> {
    let x = 5;
    borrow_x(&x) // ERROR: `x` does not live long enough
}

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}
// ANCHOR_END: static_future_with_borrow
}

mod async_move_examples {
use std::future::Future;
// ANCHOR: async_move_examples
/// `async` 区域:
///
/// 多个 `async` 区域可以访问相同的本地变量，
/// 只要它们在变量的作用域内执行.
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{}", my_string);
    };

    let future_two = async {
        // ...
        println!("{}", my_string);
    };

    // 运行两个 `future`，输出两次 "foo":
    let ((), ()) = futures::join!(future_one, future_two);
}

/// `async move` 区域:
///
/// 只有一个 `async move` 区域可以访问同一个被捕获的变量, 
/// 因为被捕获的变量已经移动到 `async move` 生成的 `future` 中:
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{}", my_string);
    }
}
// ANCHOR_END: async_move_examples
}
