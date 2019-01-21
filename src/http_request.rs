#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub text: String,
    pub headers: Vec<String>,
    pub method: Method,
    pub uri: String,
    pub protocol: String,
    pub version: String,
}

impl HttpRequest {
    pub fn new(request_text: String) -> HttpRequest {
        let trimmed_request: Vec<&str> = request_text.split("\r\n\r\n").collect();
        let parsed_request = trimmed_request[0];
        let (protocol, version) = HttpRequest::get_protocol_and_version(&parsed_request);
        HttpRequest {
            text: String::from(parsed_request.to_owned()),
            headers: HttpRequest::get_headers(&parsed_request),
            method: HttpRequest::get_method(&parsed_request),
            uri: String::from(HttpRequest::get_uri(&parsed_request)),
            protocol: String::from(protocol),
            version: String::from(version),
        }
    }
    fn get_method(text: &str) -> Method {
        let splited: Vec<&str> = text.split(" ").collect();
        let verb = splited[0];
        match verb {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => panic!("Method not recognized")
        }
    }
    fn get_uri(text: &str) -> &str {
        let splited: Vec<&str> = text.split(" ").collect();
        splited[1]
    }
    fn get_protocol_and_version(text: &str) -> (&str, &str) {
        let splited: Vec<&str> = text.split(" ").collect();
        let protocol_version = splited[2];
        let splited_protocol_version: Vec<&str> = protocol_version.split("/").collect();
        let (version, _) = splited_protocol_version[1].split_at(3);
        (splited_protocol_version[0], version)
    }
    // TODO: Implement this!
    fn get_headers(text: &str) -> Vec<String> {
        vec![String::from("Accept: text/http")]
    }
} 
