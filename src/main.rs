use serde::{Deserialize, Serialize};
use serde_json;

type Users = Vec<User>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    id: i8,
    username: String,
    age: i8,
    email: Vec<String>,
    is_married: bool,
}

fn main() {
    let json = r#"
        {
            "id": 1,
            "username": "Alfin",
            "age": 20,
            "email": [
                "foo@foo.com",
                "bob@bob.com"
            ],
            "isMarried": false
        }
        "#;

    let user: User = serde_json::from_str(json).unwrap();
    println!("{:#?}", user);

    let json = r#"
    [  
        {
            "id": 1,
            "username": "Alfin",
            "age": 20,
            "email": [
                "foo@foo.com",
                "bob@bob.com"
            ],
            "isMarried": false
        },
        {
            "id": 2,
            "username": "Aditya",
            "age": 20,
            "email": [
                "foo@foo.com",
                "bob@bob.com"
            ],
            "isMarried": true
        }
    ]
        "#;

    let users: Users = serde_json::from_str(json).unwrap();
    println!("{:#?}", users);
}
