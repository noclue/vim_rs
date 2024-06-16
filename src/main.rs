use std::fs::File;
use std::io::Read;
use openapiv3::OpenAPI;
use serde_json;

mod output;


fn main() {
    let Login = output::LoginRequestType {
        user_name: "administrator@vsphere.local".to_string(),
        password: "Admin!23".to_string(),
        locale: None,
    };
    let data = serde_json::to_string(&Login).unwrap();
    println!("{}", data);
//    let data = from_path("data/vi_json_openapi_specification_v8_0_2_0.yaml");
    // let mut file = File::open("data/vi_json_openapi_specification_v8_0_2_0.json").expect("unable to open file");

    // let mut data = String::new();
    // file.read_to_string(&mut data).expect("unable to read file");    
    // //let data = include_str!("data/vi_json_openapi_specification_v8_0_2_0.yaml");
    // let openapi: OpenAPI = serde_json::from_str(&data).expect("Could not deserialize input"); // Change OpenAPI to Value
    // println!("{:?}", openapi);

}
