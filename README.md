# Type Language

Like Telegram's one but simpler.

### Key differences

1. Doesn't support bit flags yet.
2. Supports errors.

### Example schema

```text
type Message id:int32 text:string? photos:[bytes] sent_at:time = Message
type User id:int64 verified:bool rating:float = User
type UserEmpty id:int64 = User

error InvalidUserId id:int64
error TooLongText text:string max_length:int64

func get_users user_ids:[int64] = [User]
func send_message user_id:int64 text:string? photos:[bytes] = Message
```

<details>
<summary>Generated code</summary>

```rust
pub mod types {
    #[derive(Debug)]
    pub struct Message {
        pub id: i32,
        pub text: Option::<String>,
        pub photos: Vec::<Vec::<u8>>,
        pub sent_at: std::time::SystemTime,
    }

    impl crate::Identify for Message {
        const ID: u32 = 2225622240;
    }

    impl crate::Serialize for Message {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.id.serialize(buf);
            self.text.serialize(buf);
            self.photos.serialize(buf);
            self.sent_at.serialize(buf);
        }
    }

    impl crate::Deserialize for Message {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let id = i32::deserialize(reader)?;
            let text = Option::<String>::deserialize(reader)?;
            let photos = Vec::<Vec::<u8>>::deserialize(reader)?;
            let sent_at = std::time::SystemTime::deserialize(reader)?;

            Ok(Self { id, text, photos, sent_at, })
        }
    }

    #[derive(Debug)]
    pub struct User {
        pub id: i64,
        pub verified: bool,
        pub rating: f64,
    }

    impl crate::Identify for User {
        const ID: u32 = 4055296785;
    }

    impl crate::Serialize for User {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.id.serialize(buf);
            self.verified.serialize(buf);
            self.rating.serialize(buf);
        }
    }

    impl crate::Deserialize for User {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let id = i64::deserialize(reader)?;
            let verified = bool::deserialize(reader)?;
            let rating = f64::deserialize(reader)?;

            Ok(Self { id, verified, rating, })
        }
    }

    #[derive(Debug)]
    pub struct UserEmpty {
        pub id: i64,
    }

    impl crate::Identify for UserEmpty {
        const ID: u32 = 990500211;
    }

    impl crate::Serialize for UserEmpty {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.id.serialize(buf);
        }
    }

    impl crate::Deserialize for UserEmpty {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let id = i64::deserialize(reader)?;

            Ok(Self { id, })
        }
    }

}

pub mod enums {
    #[derive(Debug)]
    pub enum Message {
        Message(crate::types::Message),
    }

    impl crate::Serialize for Message {
        fn serialize(&self, buf: &mut Vec<u8>) {
            use crate::Identify;

            match self {
                Self::Message(x) => {
                    crate::types::Message::ID.serialize(buf);
                    x.serialize(buf);
                }
            };
        }
    }

    impl crate::Deserialize for Message {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            use crate::Identify;

            let id = u32::deserialize(reader)?;

            Ok(match id {
                crate::types::Message::ID => Self::Message(crate::types::Message::deserialize(reader)?),
                _ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),
            })
        }
    }

    #[derive(Debug)]
    pub enum User {
        User(crate::types::User),
        UserEmpty(crate::types::UserEmpty),
    }

    impl crate::Serialize for User {
        fn serialize(&self, buf: &mut Vec<u8>) {
            use crate::Identify;

            match self {
                Self::User(x) => {
                    crate::types::User::ID.serialize(buf);
                    x.serialize(buf);
                }
                Self::UserEmpty(x) => {
                    crate::types::UserEmpty::ID.serialize(buf);
                    x.serialize(buf);
                }
            };
        }
    }

    impl crate::Deserialize for User {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            use crate::Identify;

            let id = u32::deserialize(reader)?;

            Ok(match id {
                crate::types::User::ID => Self::User(crate::types::User::deserialize(reader)?),
                crate::types::UserEmpty::ID => Self::UserEmpty(crate::types::UserEmpty::deserialize(reader)?),
                _ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),
            })
        }
    }

}

pub mod errors {
    #[derive(Debug)]
    pub struct InvalidUserId {
        pub id: i64,
    }

    impl crate::Identify for InvalidUserId {
        const ID: u32 = 195021614;
    }

    impl crate::Serialize for InvalidUserId {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.id.serialize(buf);
        }
    }

    impl crate::Deserialize for InvalidUserId {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let id = i64::deserialize(reader)?;

            Ok(Self { id, })
        }
    }

    #[derive(Debug)]
    pub struct TooLongText {
        pub text: String,
        pub max_length: i64,
    }

    impl crate::Identify for TooLongText {
        const ID: u32 = 2294709341;
    }

    impl crate::Serialize for TooLongText {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.text.serialize(buf);
            self.max_length.serialize(buf);
        }
    }

    impl crate::Deserialize for TooLongText {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let text = String::deserialize(reader)?;
            let max_length = i64::deserialize(reader)?;

            Ok(Self { text, max_length, })
        }
    }

}

pub mod functions {
    #[derive(Debug)]
    pub struct GetUsers {
        pub user_ids: Vec::<i64>,
    }

    impl crate::Identify for GetUsers {
        const ID: u32 = 1904452899;
    }

    impl crate::Serialize for GetUsers {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.user_ids.serialize(buf);
        }
    }

    impl crate::Deserialize for GetUsers {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let user_ids = Vec::<i64>::deserialize(reader)?;

            Ok(Self { user_ids, })
        }
    }

    impl crate::Function for GetUsers {
        type Return = Vec::<crate::enums::User>;
    }

    #[derive(Debug)]
    pub struct SendMessage {
        pub user_id: i64,
        pub text: Option::<String>,
        pub photos: Vec::<Vec::<u8>>,
    }

    impl crate::Identify for SendMessage {
        const ID: u32 = 339054040;
    }

    impl crate::Serialize for SendMessage {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.user_id.serialize(buf);
            self.text.serialize(buf);
            self.photos.serialize(buf);
        }
    }

    impl crate::Deserialize for SendMessage {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let user_id = i64::deserialize(reader)?;
            let text = Option::<String>::deserialize(reader)?;
            let photos = Vec::<Vec::<u8>>::deserialize(reader)?;

            Ok(Self { user_id, text, photos, })
        }
    }

    impl crate::Function for SendMessage {
        type Return = crate::enums::Message;
    }

}
```
</details>

### How to use

1. Clone project.
2. Create `schema.tl` file at the root of `tl-types` crate.
3. Specify `tl-types` crate in dependencies of your project.

Example usage:

```rust
use std::io::Cursor;
use tl::{Deserialize, DeserializeError, Function, Serialize};

async fn call<F: Serialize + Function>(func: F) -> Result<F::Return, DeserializeError> {
    let mut request = Vec::new();
    func.serialize(&mut request);
    // send request here

    let response = Vec::new(); // receive response here
    let mut cur = Cursor::new(response);
    F::Return::deserialize(&mut cur)
}

#[tokio::main]
async fn main() {
    let users = call(tl::functions::GetUsers {
        user_ids: vec![1, 2],
    }).await.unwrap();

    println!("{users:#?}");
}
```
