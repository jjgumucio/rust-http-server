#[derive(Debug)]
pub struct Status {
    code: u8,
    text: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    status: Status,
    headers: Vec<String>,
    text: String,
    body: String,
}

impl HttpResponse {
    pub fn new(status: Status) -> HttpResponse {
        HttpResponse {
            status: status,
            headers: vec![String::from("Content-Type: text/html")],
            text: String::from("Format the whole response into one messy string"),
            body: String::from("SOME BODY"),
        }
    }
} 
