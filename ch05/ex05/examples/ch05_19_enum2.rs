type UserName = String;

#[allow(dead_code)]
#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

// use宣言でTaskが持つバリアントをインポートするとバリアント名が直接書けるようになる
use crate::Task::*;

fn main() {
    // Task型の値を3つ作り、ベクタに格納する
    let tasks = vec![
        // もし上のuse宣言がなかったらTask::AssignedToと書かないといけない
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Done,
    ];

    // 個々のタスクの状況をレポートする。
    for (i, task) in tasks.iter().enumerate() {
        // match式によるパターンマッチでバリアントを識別し、フィールド値を取り出す
        match task {
            AssignedTo(assignee) => {
                println!("タスク{}は{}さんにアサインされています", i, assignee)
            }
            Working { assignee, remaining_hours } => {
                println!("タスク{}は{}さんが作業中です。残り{}時間の見込み",
                        i, assignee, remaining_hours)
            }
            _ => println!("タスク{}はその他のステータス（{:?}）です", i, task)
        }
    }
}
