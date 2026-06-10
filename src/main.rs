mod arg;
mod file;
mod resolver;
mod task;

use arg::parse_args;
use file::{save_to_file, read_to_file};
use resolver::resolve_all_record_types;
use task::execute_with_limit;
use std::io::{self, Write};

async fn process_task(task: String) {
    let result = resolve_all_record_types(&task).await;
    match result {
        Ok(true) => {
            let _ = save_to_file("output.txt", &task);
            println!("\x1b[32m[+]\x1b[0m {}", task);
        }
        Ok(false) => {}
        Err(_) => {}
    }
    let _ = io::stdout().flush();
}

fn set_task(domain: &str, rule: &str, dict_path: &str) -> Vec<String> {
    let reader = read_to_file(dict_path).unwrap();
    let tasks: Vec<String> = reader
        .filter_map(|line| line.ok())
        .map(|line| format!("{}{}{}", line, &rule, &domain))
        .collect();

    tasks
}

#[tokio::main]
async fn main() {
    let args = parse_args();

    let domian = args.domian.unwrap_or_else(|| {
        eprintln!("未提供域名");
        std::process::exit(1);
    });

    let dict_path = args.dict.unwrap_or_else(|| {
        eprintln!("未提供字典文件");
        std::process::exit(1);
    });

    let rule = args.rule.unwrap_or(".".to_string());

    let pool_size = args.pool_size.unwrap_or(10);
    let tasks = set_task(&domian, &rule, &dict_path);
    eprintln!("[*] 开始爆破 {} 条任务，最大并发: {}", tasks.len(), pool_size);
    execute_with_limit(pool_size, tasks, process_task).await;
    eprintln!("[*] 爆破完成");
    eprintln!("[*] 结果已保存到 output.txt");
}