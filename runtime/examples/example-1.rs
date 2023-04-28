use serde_json::Value;

#[tokio::main]
async fn main() {
    // let json = read_to_string("runtime/examples/test.postman_collection.json").unwrap();
    let json_str = r#"
    {
       "name": "Transaction Info",
       "request": {
        "method": "POST",
        "header": [
			{
				"key": "x-api-key",
				"value": "M0em5cixFdsw0nA3W6DO8NGlDTXWYsT7rCyvy3sd"
			}
        ],
        "body": {
            "mode": "raw",
            "raw": "{\n    \"entityId\":\"NSMEWTCUST01\"\n}\n",
            "options": {
                "raw": {
                    "language": "json"
                }
            }
        },
        "url": {
            "raw": "https://staging.api.mewt.in/card/payment/transaction",
            "host": [
                "{{base_url}}"
            ],
            "path": [
                "card",
                "payment",
                "transaction"
            ]
        }
      }
    }
    "#;

    let mut json_value: Value = serde_json::from_str(json_str).unwrap();
    let body_str = match json_value["request"]["body"]["raw"].take() {
        Value::String(val) => val,
        _ => panic!("Error")
    };
    let body: Value = serde_json::from_str(body_str.as_str()).unwrap();
    let headers = match json_value["request"]["header"].take() {
        Value::Array(val) => val,
        _ => panic!("Error"),
    };
    let url = match json_value["request"]["url"]["raw"].take() {
        Value::String(val) => val,
        _ => panic!("Error"),
    };

    println!("{:?}", body_str);

    runtime::make_request("POST", url, headers, Some(body)).await;
}
