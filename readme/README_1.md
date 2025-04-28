好嘞！  
给你来一份 🔥 **最全的 `.iter` 系统总结**！  
直接开冲：

---

# Rust `.iter()`、`.iter_mut()`、`.into_iter()` 系统总结

| 方法           | 意义                       | 迭代得到的类型 | 是否消耗原数据？ | 示例                             |
| :------------- | :------------------------- | :------------- | :--------------- | :------------------------------- |
| `.iter()`      | 不可变借用地遍历集合       | `&T`           | 否               | `for item in vec.iter() {}`      |
| `.iter_mut()`  | 可变借用地遍历集合         | `&mut T`       | 否               | `for item in vec.iter_mut() {}`  |
| `.into_iter()` | 消费集合，拿走所有权地遍历 | `T`            | ✅ 是的（消耗）  | `for item in vec.into_iter() {}` |

---

## 1. `.iter()` — 普通借用遍历

- 元素是 `&T`
- 不改变集合
- 集合还可以继续用

```rust
let v = vec![1, 2, 3];
for item in v.iter() {
    println!("{item}");
}
// v还能用
println!("v.len() = {}", v.len());
```

---

## 2. `.iter_mut()` — 可变借用遍历

- 元素是 `&mut T`
- 可以**修改**集合内容
- 需要 `mut` 集合

```rust
let mut v = vec![1, 2, 3];
for item in v.iter_mut() {
  // .iter_mut() 返回的是每个元素的可变引用（类型是 &mut T）
  // item 是 &mut i32
  // 如果你要直接操作值（比如乘2），就需要先解引用 *item，拿到i32本身
  // *item *= 2 是什么意思？   =>>  *item = *item * 2
    *item *= 2;
}
println!("{:?}", v); // [2, 4, 6]
```

注意：**`v`必须是`mut`的**，因为 `iter_mut` 会对元素产生可变借用！

---

## 3. `.into_iter()` — 消耗集合遍历

- 元素是 `T`
- 集合**被消耗**，之后不能再用

```rust
let v = vec![1, 2, 3];
for item in v.into_iter() {
    println!("{item}");
}
// println!("{:?}", v); // ❌ v已经失效，不能用了
```

---

## 🔥 额外补充：for 循环其实自动调用了什么？

- 如果是 `Vec`，`for item in v`，其实自动用的是 `.into_iter()`
- 如果是 `&Vec`，`for item in &v`，自动用的是 `.iter()`
- 如果是 `&mut Vec`，`for item in &mut v`，自动用的是 `.iter_mut()`

所以下面这些都是合法的：

```rust
let v = vec![1, 2, 3];

for item in &v { println!("{item}"); } // 调 .iter()
for item in &mut v.clone() { println!("{item}"); } // 调 .iter_mut()
for item in v { println!("{item}"); } // 调 .into_iter()
```

---

# 总结口诀 🧠

> **想改用 `.iter_mut()`，想借用用 `.iter()`，想搬走用 `.into_iter()`！**

或者：

| 目标     | 用法           |
| :------- | :------------- |
| 只读遍历 | `.iter()`      |
| 修改元素 | `.iter_mut()`  |
| 拿走元素 | `.into_iter()` |

---

## 例子：回到你的 `todo_list`

```rust
todo_list.iter()       // 只需要借用，不修改，不消耗
todo_list.iter_mut()   // 需要修改元素，比如改todo内容
todo_list.into_iter()  // 消耗todo_list，比如转成别的集合
```

---

要不要我再顺便给你补一个：

- 为什么链式调用 (`map`、`filter`、`chain`) 都需要 `iter`？
- `collect` 自动推导和手动指定类型的场景？

想要的话告诉我：“继续补充链式总结！” 🚀  
我可以接着整理得非常清晰！要继续吗？🔍
