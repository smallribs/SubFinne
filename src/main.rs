mod arg;
mod file;
mod resolver;
mod task;

use arg::parse_args;
use file::{save_to_file, read_to_file};
use resolver::resolve_all_record_types;
use task::execute_with_limit;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
async fn process_fn(rx: mpsc::Receiver<String>) {
    let task = rx.recv().unwrap();
    let result = resolve_all_record_types(&task).await;
    if let Ok(true) = result {
        save_to_file("output.txt", &task).unwrap();
    }
}

fn set_task(domain: &str, rule: &str, dict_path: &str) -> Vec<String> {
    let reader = read_to_file(dict_path).unwrap();
    let tasks: Vec<String> = reader
        .filter_map(|line| line.ok())
        .map(|line| format!("{}{}{}", line, &rule, &domain))
        .collect();

    tasks
}

fn init_domain(domain: &str, dict_path: &str) -> Vec<String> {
    let reader = read_to_file(dict_path).unwrap();
    let domains: Vec<String> = reader
        .filter_map(|line| line.ok())
        .map(|line| format!("{}.{}", line, &domain))
        .collect();

    domains
}

fn init_word_list(dict_path: &str) -> Vec<String> {
    let reader = read_to_file(dict_path).unwrap();
    let word_list: Vec<String> = reader
        .filter_map(|line| line.ok())
        .map(|line| line.to_string())
        .collect();

    word_list
}

fn generate_domains(tx: mpsc::Sender<String>, prefix: String, depth: usize, max_depth: usize, word_list: &Vec<String>, domain: &str) {
    if depth == max_depth {
        println!("{}.{}", prefix, domain);
        let _ = tx.send(format!("{}{}", prefix, domain));
        return;
    }
    for item in word_list.iter() {
        let new_prefix = if prefix.is_empty() {
            item.to_string()
        } else {
            format!("{}-{}", prefix, item)
        };
        generate_domains(tx.clone(), new_prefix, depth + 1, max_depth, &word_list, domain);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let num_workers = 30;
    let producer_tx = tx.clone();
    let word_list = init_word_list("./test.txt");
    let domain = "example.com";
    let max_depth = 3;
    thread::spawn(move || {
        generate_domains(producer_tx, "".to_string(), 0, max_depth, &word_list, domain);
    });
    let mut workers = vec![];
    let rx = Arc::new(Mutex::new(rx));
    for i in 0..num_workers {
        let rx_clone = Arc::clone(&rx);
        workers.push(thread::spawn(move || process_fn(rx_clone)));
    }

    // 等待所有消费者完成（因为 mpsc::Receiver 没有 drop，所有 Sender 关闭后会自动结束）
    for worker in workers {
        worker.join().unwrap();
    }
    return;
    let args = parse_args();

    let domian = args.domian.unwrap_or_else(|| {
        eprintln!("未提供域名");
        std::process::exit(1);
    });

    let dict_path = args.dict.unwrap_or_else(|| {
        eprintln!("未提供字典文件");
        std::process::exit(1);
    });

    let rule = args.rule.unwrap_or_else(|| {
        eprintln!("未提供规则");
        std::process::exit(1);
    });

    let pool_size = args.pool_size.unwrap_or(10);
    let domains = init_domain(&domian, &dict_path);
    for domain in domains {
        let tasks = set_task(&domain, &rule, &dict_path);
        execute_with_limit(pool_size, tasks, process_fn).await;
    }

}
