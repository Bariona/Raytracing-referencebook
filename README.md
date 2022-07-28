# RayTracing-referencebook

## Outline:

- Reference Book:[《RayTracing In One Weekend》](https://raytracing.github.io/)

- Goal: accomplish with Rust 

## Current Progress: 

1. **bonus part**:

- [x] track 1: Reduce Contention
- [x] track 2: Static Dispatch (增加泛型)
- [x] track 3: Code Generation (add function-like macro)
        
    具体可以查看macro.md(学习笔记)
- [x] track 4: PDF dispatch
- [x] track 6: support PDF for Translate
- [ ] track 7: Benchmark (在学...)
- [x] track 8: support for OBJ 

胡桃贴图
![](output/hutao.jpg)

![](output/baseball.jpg)

2. Reference Book:

- Book3 done

学习重点: Monte Carlo Integration

$$
    I = \int_a^b f(x) dx
$$
建立一个连续型随机变量$\chi$, 满足$\chi$在$[a,b]$分布, 则

$$
\begin{aligned}
E(\chi) &= \int_a^b \frac{f(x)}{p(x)} p(x) dx\\
&=I
\end{aligned}
$$

同时我们随机采样$N$次, 则有$\overline{X}=\frac 1N \sum X_i$为$E(X)$的无偏估计.


在RT中, 要做的是:
$$
Color = \int A \cdot s(direction) \cdot color(direction)
$$

$s(direction)$: 为材质表面在$dir$方向的概率密度函数
$color(direction)$: 为该方向射来的光的RGB参数


![](output/book3.jpg)


- Book2 done

![](output/book2.jpg)

- Book1 done

这里的random_scene() 出现了两个小球相交的情况

解决办法:
1. 多随机几次
2. 每次不断随机一个小球直至与前面的球不相交

![](output/book1.jpg)
