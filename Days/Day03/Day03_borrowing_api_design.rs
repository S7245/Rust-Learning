use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TaskId(u64);

#[derive(Clone, Debug, PartialEq, Eq)]
enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Task {
    id: TaskId,
    title: String,
    status: TaskStatus,
}

#[derive(Debug)]
enum DomainError {
    TaskNotFound(TaskId),
}

type DomainResult<T> = Result<T, DomainError>;

struct TaskStore {
    next_id: u64,
    tasks: HashMap<TaskId, Task>,
}

impl TaskStore {
    fn new() -> Self {
        Self {
            next_id: 1,
            tasks: HashMap::new(),
        }
    }

    fn create_task(&mut self, title: &str) -> TaskId {
        let id = TaskId(self.next_id);
        self.next_id += 1;

        let task = Task {
            id,
            title: title.to_string(),
            status: TaskStatus::Todo,
        };

        self.tasks.insert(id, task);
        id
    }

    fn get_task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn update_title(&mut self, id: TaskId, new_title: &str) -> DomainResult<()> {
        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(DomainError::TaskNotFound(id))?;

        task.title.clear();
        task.title.push_str(new_title);
        Ok(())
    }

    fn mark_done(&mut self, id: TaskId) -> DomainResult<()> {
        let task = self
            .tasks
            .get_mut(&id)
            .ok_or(DomainError::TaskNotFound(id))?;

        task.status = TaskStatus::Done;
        Ok(())
    }
}

// 三种 API 形状对比：按值（move）/ 共享借用 / 可变借用
fn takes_owned(task: Task) {
    println!("[takes_owned] moved in: {:?}", task);
}

fn takes_ref(task: &Task) {
    println!("[takes_ref] borrowed read-only: {:?}", task);
}

fn takes_mut(task: &mut Task) {
    task.status = TaskStatus::InProgress;
    println!("[takes_mut] mutated in-place: {:?}", task);
}

fn main() {
    let mut store = TaskStore::new();

    let id1 = store.create_task("learn borrowing APIs");
    let id2 = store.create_task("apply &T / &mut T to FerrisFlow");

    println!("== after create ==");
    println!("id1 => {:?}", store.get_task(id1));
    println!("id2 => {:?}", store.get_task(id2));

    store.update_title(id1, "learn borrowing APIs (deep dive)")
        .expect("update should succeed");
    store.mark_done(id2).expect("mark done should succeed");

    println!("\n== after updates ==");
    let t1_ref = store.get_task(id1).expect("task exists");
    takes_ref(t1_ref);

    // 下面这段是典型“读着读着想写”的借用冲突：
    // 你拿着对 store 的共享借用（t1_ref），又要对同一个 store 做可变借用。
    //
    // store.update_title(id1, "this will not compile").unwrap();

    // 修复方式之一：缩短只读借用的生命周期（让它在调用 update 前结束）。
    {
        let preview = store.get_task(id1).unwrap();
        println!("preview before update: {:?}", preview);
    }
    store.update_title(id1, "borrow scope fixed")
        .expect("update should succeed");

    println!("\n== move / borrow / mut borrow demo ==");
    let owned = store.get_task(id1).unwrap().clone();
    takes_owned(owned);

    let mut local = store.get_task(id1).unwrap().clone();
    takes_mut(&mut local);

    println!("\n== final store snapshot ==");
    println!("id1 => {:?}", store.get_task(id1));
    println!("id2 => {:?}", store.get_task(id2));
}
