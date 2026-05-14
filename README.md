# Rust 50 天高级学习计划

本仓库用于承载一套系统化、工程化、偏高级的 Rust 学习计划。

连续项目：`FerrisFlow`，一个从 CLI 逐步演进为具备持久化、异步调度、Web API、可观测性、性能优化和发布流程的本地任务编排系统。

## 仓库约定

- `Course/syllabus.json` 是唯一事实源。
- `Days/DayNN/` 由每日自动化任务按课程日期生成。
- 每天的 Notion 正文、代码文件、commit 信息都必须从 `Course/syllabus.json` 读取当天字段后生成。
- 不允许覆盖已有 Day 文件；如果当天目录或文件已存在，自动化任务应停止并报告，除非只是补齐 Notion 的 `GitHub Commit` 区块。

## 自动化

- 开始日期：`2026-05-14`
- 时区：`Asia/Shanghai`
- 每日运行时间：`11:00`
- Notion 主页面：https://www.notion.so/Rust-Plan-35fbfc73ea1480cd876ce9932f452269?source=copy_link
- GitHub 远端：https://github.com/S7245/Rust-Learning.git

每日提交信息格式：

```text
Day NN: add {topic} learning code
```

