use std::{collections::HashMap, fs};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[tokio::main]
async fn main() {
    let book_url = "https://www.gutenberg.org/files/2591/2591-0.txt";
    let book_response = reqwest::get(book_url).await.unwrap();
    let book_contents = book_response.text().await.unwrap();

    let book_chars = book_contents.chars().collect::<Vec<char>>();
    let char_count = book_chars
        .into_par_iter()
        .filter(|c| c.is_alphabetic())
        .fold(
            || HashMap::new(),
            |mut counts, c| {
                *counts.entry(c).or_insert(0) += 1;
                counts
            },
        )
        .reduce(
            || HashMap::new(),
            |mut fin_counts, thread_counts| {
                for (char, num) in thread_counts {
                    *fin_counts.entry(char).or_insert(0) += num;
                }
                fin_counts
            },
        );

    println!("{:?}", char_count);

    let file_path = "./char_count.json";
    let json_file = fs::File::create(file_path).unwrap();

    serde_json::to_writer(json_file, &char_count).unwrap();
}
