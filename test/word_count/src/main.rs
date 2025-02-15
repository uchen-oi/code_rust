use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // 打开并读取文件
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    // 创建一个哈希表来统计单词出现次数
    let mut word_count = HashMap::new();

    // 逐行读取文件
    for line in reader.lines() {
        let line = line?;
        // 按空格分割行并遍历每个单词
        for word in line.split_whitespace() {
            let count = word_count.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }

    // 将统计结果打印到控制台
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

    // 将结果保存到新的文件
    let mut output = File::create("word_count.txt")?;
    for (word, count) in &word_count {
        writeln!(output, "{}: {}", word, count)?;
    }

    Ok(())
}