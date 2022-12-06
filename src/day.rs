use std::io::{Read, Write};

use std::time::Instant;
use std::marker::{Send, Sync};

static mut COOKIE: Option<String> = None;

pub trait Day: Send + Sync {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
    fn get_test_data(&self) -> String;
    fn compute(&self) -> String {
        Self::fetch_input_from_website(self);
        let day_path = format!("./inputs/day{}.txt", self.get_day_number());
        let test_data = self.get_test_data();
        let mut result = String::new();
        if !test_data.is_empty() {
            result.push_str(&format!("Day {}: part 1 test: {}\n",
                                     self.get_day_number(),
                                     self.part1(test_data.as_str())
            ));
            result.push_str(&format!("Day {}:  part 2 test: {}\n",
                                     self.get_day_number(),
                                     self.part2(test_data.as_str())
            ));
        }
        let input = std::fs::read_to_string(day_path).unwrap();
        // invoke part1 and measure time
        let start = Instant::now();
        let part1_result = self.part1(&input);
        let part1_duration = start.elapsed();
        // invoke part2 and measure time
        let start = Instant::now();
        let part2_result = self.part2(&input);
        let part2_duration = start.elapsed();
        result.push_str(&format!("Day {}: part 1: {} ({} ms)\n", self.get_day_number(), part1_result, part1_duration.as_millis()));
        result.push_str(&format!("Day {}: part 2: {} ({} ms)\n", self.get_day_number(), part2_result, part2_duration.as_millis()));

        result
    }

    fn fetch_input_from_website(&self) {
        let day_number = self.get_day_number();
        let file_path_input = format!("./inputs/day{}.txt", day_number);
        let url = format!("https://adventofcode.com/2022/day/{}/input", day_number);

        Self::download_aoc_file(self, &file_path_input, day_number, &url);

        let file_path_html = format!("./questions_html/day{}.html", day_number);
        let url = format!("https://adventofcode.com/2022/day/{}", day_number);

        Self::download_aoc_file(self, &file_path_html, day_number, &url);

        let file_path_md = format!("./questions/day{}.md", day_number);
        if (std::path::Path::new(&file_path_md)).exists() {
            return;
        }
        // read question from file and get only <article> tag
        let mut file = std::fs::File::open(file_path_html).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let start = contents.find("<main").unwrap();
        let end = contents.find("</main>").unwrap();
        let question = &contents[start..end];

        // convert string to const c_char
        let md = html2md::parse_html(question);
        let mut file = std::fs::File::create(file_path_md).unwrap();
        file.write_all(md.as_bytes()).unwrap();
    }

    fn download_aoc_file(&self, file_path: &String, day_number: i32, url: &String) {
        if std::path::Path::new(&file_path).exists() {
            return;
        }

        // if we have cookie in static COOKIE, use it
        let cookie = unsafe {
            match COOKIE {
                Some(ref cookie) => cookie,
                None => {
                    // read cookie from terminal
                    let mut cookie = String::new();
                    println!("Please enter your session:");
                    std::io::stdin().read_line(&mut cookie).unwrap();
                    // store cookie in static variable
                    // trim line breaks
                    cookie = cookie.trim().to_string();
                    // print what you read
                    println!("You entered: `{}`", cookie);
                    COOKIE = Some(cookie);
                    match COOKIE {
                        Some(ref cookie) => cookie,
                        None => panic!("Cookie not found"),
                    }
                }
            }
        };

        // to use  reqwest::blocking::Client::new() we need to add reqwest = { version = "0.11", features = ["blocking"] } to Cargo.toml
        let client = reqwest::blocking::Client::new();

        let request_builder = client.get(url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0")
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Referer", format!("https://adventofcode.com/2022/day/{}", day_number))
            .header("DNT", "1")
            .header("Connection", "keep-alive")
            .header("Cookie", format!("session={}", cookie))
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "same-origin")
            .header("Sec-Fetch-User", "?1")
            .header("Sec-GPC", "1")
            .header("Pragma", "no-cache")
            .header("Cache-Control", "no-cache");


        let request = request_builder.build()
            .unwrap_or_else(|e| {
                panic!("Error building request: {}", e);
            });

        // print all headers
        println!("Headers:");
        for (key, value) in request.headers() {
            println!("{}: {}", key, value.to_str().unwrap());
        }

        //send request
        let response = client.execute(request)
            .unwrap_or_else(|e| {
                panic!("Error sending request: {}", e);
            });
        // print response headers
        println!("Response headers:");
        for (key, value) in response.headers() {
            println!("{}: {}", key, value.to_str().unwrap());
        }

        let is_gzip = response.headers().get("Content-Encoding").unwrap().to_str().unwrap() == "gzip";

        // if it's gzip, unzip it
        let body = response.bytes()
            .unwrap_or_else(|e| {
                panic!("Error reading response: {}", e);
            });
        let body = if is_gzip {
            // to use flate2::read::GzDecoder we need to add flate2 = "1.0.20" to Cargo.toml
            let mut decoder = flate2::read::GzDecoder::new(&body[..]);
            let mut decoded_body = Vec::new();
            decoder.read_to_end(&mut decoded_body)
                .unwrap_or_else(|e| {
                    panic!("Error decoding response: {}", e);
                });
            decoded_body
        } else {
            body.to_vec()
        };
        // remove last line break
        let body = String::from_utf8(body).unwrap();
        let body = body.trim().to_string();

        // write to file
        std::fs::write(file_path, body).unwrap();
    }
    fn get_day_number(&self) -> i32;
}