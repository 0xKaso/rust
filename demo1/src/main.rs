fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let md = html2md::parse_html(&body);

    std::fs::write(output, md.as_bytes()).unwrap();

    
    println!("✅ 网站抓取完成!");
}
