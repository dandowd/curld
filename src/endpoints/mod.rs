pub fn construct_curl_endpoint(
    endpoint: String,
    method: String,
    data: Option<String>,
    base_url: Option<String>,
    headers: Option<Vec<String>>,
) -> String {
    let url = match base_url {
        Some(base_url) => format!(
            "{base_url}/{endpoint}",
            base_url = base_url,
            endpoint = endpoint
        ),
        None => endpoint,
    };

    let header_str = match headers {
        Some(headers) => {
            let mut header_str = String::new();
            for header in headers {
                header_str.push_str(&format!("--header {}", header))
            }
            header_str
        },
        None => "".to_string()
    };

    let data_str = match data {
        Some(data) => data,
        None => "".to_string(),
    };

    format!(
        "--request {method}{headers}{data} {url}",
        method = method,
        headers = header_str,
        data = data_str,
        url = url
    )
}
