pub fn construct_curl_cmd(
    endpoint: &String,
    method: &String,
    data: &String,
    base_url: &String,
    headers: &Vec<String>,
) -> String {
    let url = format!(
        " {base_url}{endpoint}",
        base_url = base_url,
        endpoint = endpoint
    );

    let header_str = {
        let mut header_str = String::new();
        for header in headers {
            header_str.push_str(&format!(" --header '{}'", header))
        }
        header_str
    };

    let data_str = if !data.is_empty() {
        format!(" --data '{}'", data)
    } else {
        "".to_string()
    };

    format!(
        "--request -X {method}{headers}{data}{url}",
        method = method,
        headers = header_str,
        data = data_str,
        url = url
    )
}

pub fn construct_curl_args(
    endpoint: &String,
    method: &String,
    data: &String,
    base_url: &String,
    headers: &Vec<String>,
) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    args.push("-X".to_string());
    args.push(method.to_string());

    let header_str = {
        let mut header_str = String::new();
        for header in headers {
            header_str.push_str(&format!(" --header '{}'", header))
        }
        header_str
    };

    if header_str.is_empty() {
        args.push(header_str);
    }

    if !data.is_empty() {
        args.push("-d".to_string());
        args.push(data.to_string());
    };

    // Url should always be last
    let url = format!(
        "{base_url}{endpoint}",
        base_url = base_url,
        endpoint = endpoint
    );
    args.push(url);

    args
}

