# Type Language

Like Telegram's one but simpler.

### Key differences

1. Doesn't support enums (types), only types (constructors).
2. Doesn't support bit flags.
3. Supports errors.

### Example schema

```text
type Message id:int32 text:string? photos:[bytes] sent_at:time
type User id:int64 verified:bool rating:float

err InvalidUserId id:int64
err TooLongText text:string max_length:int64

func get_users user_ids:[int64] = [User]
func send_message user_id:int64 text:string? photos:[bytes] = Message
```

<details>
<summary>Generated code</summary>

```rust
pub mod types {
    pub struct Message {
        pub id: i32,
        pub text: Option::<String>,
        pub photos: Vec::<Vec::<u8>>,
        pub sent_at: std::time::SystemTime,
    }

    impl crate::Identify for Message {
        const ID: u32 = 226668223;
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
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let id = i32::deserialize(cur)?;
            let text = Option::<String>::deserialize(cur)?;
            let photos = Vec::<Vec::<u8>>::deserialize(cur)?;
            let sent_at = std::time::SystemTime::deserialize(cur)?;

            Ok(Self { id, text, photos, sent_at, })
        }
    }

    pub struct User {
        pub id: i64,
        pub verified: bool,
        pub rating: f64,
    }

    impl crate::Identify for User {
        const ID: u32 = 780412182;
    }

    impl crate::Serialize for User {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.id.serialize(buf);
            self.verified.serialize(buf);
            self.rating.serialize(buf);
        }
    }

    impl crate::Deserialize for User {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let id = i64::deserialize(cur)?;
            let verified = bool::deserialize(cur)?;
            let rating = f64::deserialize(cur)?;

            Ok(Self { id, verified, rating, })
        }
    }

}

pub mod errors {
    pub struct InvalidUserId {
        pub id: i64,
    }

    impl crate::Identify for InvalidUserId {
        const ID: u32 = 1636459753;
    }

    impl crate::Serialize for InvalidUserId {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.id.serialize(buf);
        }
    }

    impl crate::Deserialize for InvalidUserId {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let id = i64::deserialize(cur)?;

            Ok(Self { id, })
        }
    }

    pub struct TooLongText {
        pub text: String,
        pub max_length: i64,
    }

    impl crate::Identify for TooLongText {
        const ID: u32 = 1299639700;
    }

    impl crate::Serialize for TooLongText {
        fn serialize(&self, buf: &mut Vec<u8>) {
            self.text.serialize(buf);
            self.max_length.serialize(buf);
        }
    }

    impl crate::Deserialize for TooLongText {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let text = String::deserialize(cur)?;
            let max_length = i64::deserialize(cur)?;

            Ok(Self { text, max_length, })
        }
    }

}

pub mod functions {
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
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let user_ids = Vec::<i64>::deserialize(cur)?;

            Ok(Self { user_ids, })
        }
    }

    impl crate::Function for GetUsers {
        type Return = Vec::<crate::types::User>;
    }

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
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let user_id = i64::deserialize(cur)?;
            let text = Option::<String>::deserialize(cur)?;
            let photos = Vec::<Vec::<u8>>::deserialize(cur)?;

            Ok(Self { user_id, text, photos, })
        }
    }

    impl crate::Function for SendMessage {
        type Return = crate::types::Message;
    }

}
```
</details>

### How to use

1. Clone project.
2. Create `schema.tl` file at the root of `tl-types` crate.
3. Specify `tl-types` crate in dependencies of your project.

Usage in code is simple:

```text
tl_types::types::User
tl_types::types::Message
tl_types::errors::InvalidUserId
tl_types::errors::TooLongText
tl_types::functions::GetUsers
tl_types::functions::SendMessage
```
