不要用官方提供的benchmark(~~好像一定要rust的非stable版本才可以跑~~)

可以用社区里的criterion库

- [官方文档](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)

- [b乎文章](https://zhuanlan.zhihu.com/p/402478044) 这篇文章涉及了一点宏编程的知识

其中在.toml里要加上

```rust
name = "my_benchmark"
harness = false
```

harness=false是为了不调用官方的benchmark

这样在运行cargo bench时就会自动定位到project目录下的`benches/my_benchmark.rs`文件

运行完后可以再./target/criterion/xxx/index.html里查看具体运行时间和各种chart.