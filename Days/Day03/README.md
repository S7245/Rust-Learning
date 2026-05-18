# Day 03｜借用、引用与 API 形状设计

> 本日聚焦：把 `&T / &mut T` 当作“API 设计工具”，并把它落到 FerrisFlow 的任务（Task）创建/更新/查询接口中。

## 你要解决的真实问题
当你写一个“任务存储（store）”时，会自然遇到这些选择：

- 我是把任务 `Task` **按值传进去**（发生 move）？
- 我是传 `&Task` **只读借用**（避免 move/clone）？
- 我是传 `&mut Task` **独占可变借用**（就地更新，强约束但更安全）？

Rust 的借用检查器不是来“卡你”，而是强迫你把边界写清楚：谁拥有数据、谁能修改、修改到哪里截止。

## 三种常见 API 形状（以及它们在调用侧的代价）

### 1) 按值传递（move）
- 形状：`fn takes_owned(t: Task)`
- 代价：调用者把所有权交出去；如果还想继续用原值，要 `clone()` 或改成借用。
- 适用：明确的所有权转移（例如把对象交给队列、线程、异步任务）。

### 2) 共享借用（只读）
- 形状：`fn takes_ref(t: &Task)` 或 `fn get(&self, ...) -> Option<&Task>`
- 代价：返回引用时，生命周期会把“引用活多久”绑定到 `self` 上；调用者需要在引用存活期间遵守限制。
- 适用：读多写少、避免拷贝。

### 3) 可变借用（独占写入）
- 形状：`fn takes_mut(t: &mut Task)` 或 `fn update(&mut self, ...) -> ...`
- 代价：同一时间只能有一个可变借用；这会限制同时访问，但换来强一致性和更好推理。
- 适用：就地更新、需要保证状态修改的原子性。

## 最常见的借用冲突：读着读着就想写
一个经典错误模式是：

- 你先 `get_task(&self)` 拿到了 `&Task`（对 store 的共享借用）
- 然后又想 `update_title(&mut self)`（对同一个 store 的可变借用）

解决思路通常不是“到处 clone”，而是：

- **缩小借用范围**：把只读借用放在更小的作用域里
- **先算后写**：先把要写入的数据准备好，再一次性 `&mut` 写入

## 今日实践：FerrisFlow 的最小任务存储与 API
对应代码文件：`Day03_borrowing_api_design.rs`

你将实现：
- `TaskId`：新类型包裹 `u64`，避免裸数字乱传
- `Task`：最小字段 `id/title/status`
- `TaskStore`：`HashMap<TaskId, Task>` + 自增 `next_id`
- API：
  - `create_task(&mut self, title: &str) -> TaskId`
  - `get_task(&self, id: TaskId) -> Option<&Task>`
  - `update_title(&mut self, id: TaskId, new_title: &str) -> Result<(), DomainError>`

### 运行方式
该示例只依赖标准库，可直接用 `rustc` 运行：

```bash
rustc Days/Day03/Day03_borrowing_api_design.rs -o /tmp/day03 && /tmp/day03
```

## 参考源
- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- https://doc.rust-lang.org/reference/types/pointer.html
- https://rust-lang.github.io/api-guidelines/flexibility.html
