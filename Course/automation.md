# Daily Automation Contract

The daily automation must treat `Course/syllabus.json` as the only source of truth.

Required behavior:

1. Read `Course/syllabus.json` before generating anything.
2. Compute `dayNumber = currentDate(Asia/Shanghai) - startDate + 1`.
3. Stop if `dayNumber` is outside the course range.
4. Stop and report an error if the day's `topic`, `objective`, `focus`, `objcBridge`, `practice`, `understanding`, `diagram`, or `reference` is missing or empty.
5. Generate the Notion Day page content from the day's syllabus fields as a reader-facing teaching article, not as a prompt, checklist, or generation plan.
6. Use this article structure by default: `概念剖析 (The Why and What)`, `核心代码演示 (The How)`, `架构师的实战洞察`, `今日挑战`, and `延伸阅读推荐`.
7. Include client-language migration notes in the concept section. Prefer Swift and Objective-C comparisons when useful, especially ARC, `weak`, `inout`, `Data`/`NSData`, `ArraySlice`, `async/await`, and protocol-oriented design.
8. Code examples should be idiomatic Rust and, when feasible, runnable as a standalone `main.rs` in CodeSandbox or with `rustc`.
9. Diagram content must be written as teaching copy: include the figure's learning purpose, caption, and where it helps comprehension. Do not include prompt-like generation instructions, self-check notes, or internal wording such as "the diagram should draw".
10. Generate `Days/DayNN/README.md` and `Days/DayNN/DayNN_<codeSlug>.rs`.
11. Never overwrite existing files.
12. Commit with `Day NN: add {topic} learning code`.
13. Push to `origin`.
14. Sync the short commit SHA and commit URL to the Notion Day page under `GitHub Commit`.
15. If push fails, keep the Notion tutorial content and record the failure reason in `GitHub Commit`.

Required Notion Day page sections:

```markdown
# 概念剖析 (The Why and What)
# 核心代码演示 (The How)
# 架构师的实战洞察
# 今日挑战
# 延伸阅读推荐
# GitHub Commit
```
