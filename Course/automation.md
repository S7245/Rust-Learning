# Daily Automation Contract

The daily automation must treat `Course/syllabus.json` as the only source of truth.

Required behavior:

1. Read `Course/syllabus.json` before generating anything.
2. Compute `dayNumber = currentDate(Asia/Shanghai) - startDate + 1`.
3. Stop if `dayNumber` is outside the course range.
4. Stop and report an error if the day's `topic`, `focus`, `practice`, `understanding`, or `reference` is missing or empty.
5. Generate the Notion Day page content from the day's syllabus fields.
6. Generate `Days/DayNN/README.md` and `Days/DayNN/DayNN_<codeSlug>.rs`.
7. Never overwrite existing files.
8. Commit with `Day NN: add {topic} learning code`.
9. Push to `origin`.
10. Sync the short commit SHA and commit URL to the Notion Day page under `GitHub Commit`.
11. If push fails, keep the Notion tutorial content and record the failure reason in `GitHub Commit`.

Required Notion Day page sections:

```markdown
# 主题
# 学习重点
# 项目实践
# 核心理解
# 参考源
# 今日目标
# 知识展开
# 代码示例
# 项目任务拆解
# 常见误区
# 自测问题
# 验收标准
# 明日衔接
# GitHub Commit
```

