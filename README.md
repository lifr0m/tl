# Type Language

Like Telegram's one but simpler.

### Key differences

1. Doesn't support enums (types), only types (constructors).
2. Doesn't support bit flags.

### Example schema

```text
# Supports comments
type Message id:int32 text:string? photos:[bytes] sent_at:time
type User id:int64 verified:bool rating:float

# Some functions
func get_users [User] user_ids:[int64]
func send_message Message user_id:int64 text:string? photos:[bytes]
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
        const ID: crate::Id = crate::Id([191, 174, 130, 13]);
    }

    impl crate::serialize::Serialize for Message {
        fn serialize(&self, buf: &mut Vec<u8>) {
            use crate::Identify;

            Self::ID.serialize(buf);
            self.id.serialize(buf);
            self.text.serialize(buf);
            self.photos.serialize(buf);
            self.sent_at.serialize(buf);
        }
    }

    impl crate::deserialize::Deserialize for Message {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let id = i32::deserialize(cur)?;
            let text = Option::<String>::deserialize(cur)?;
            let photos = Vec::<Vec::<u8>>::deserialize(cur)?;
            let sent_at = std::time::SystemTime::deserialize(cur)?;
            Ok(Self { id, text, photos, sent_at, })
        }
    }

    impl crate::Definition for Message {}

    pub struct User {
        pub id: i64,
        pub verified: bool,
        pub rating: f64,
    }

    impl crate::Identify for User {
        const ID: crate::Id = crate::Id([22, 37, 132, 46]);
    }

    impl crate::serialize::Serialize for User {
        fn serialize(&self, buf: &mut Vec<u8>) {
            use crate::Identify;

            Self::ID.serialize(buf);
            self.id.serialize(buf);
            self.verified.serialize(buf);
            self.rating.serialize(buf);
        }
    }

    impl crate::deserialize::Deserialize for User {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let id = i64::deserialize(cur)?;
            let verified = bool::deserialize(cur)?;
            let rating = f64::deserialize(cur)?;
            Ok(Self { id, verified, rating, })
        }
    }

    impl crate::Definition for User {}

}

pub mod functions {
    pub struct GetUsers {
        pub user_ids: Vec::<i64>,
    }

    impl crate::Identify for GetUsers {
        const ID: crate::Id = crate::Id([159, 52, 221, 250]);
    }

    impl crate::serialize::Serialize for GetUsers {
        fn serialize(&self, buf: &mut Vec<u8>) {
            use crate::Identify;

            Self::ID.serialize(buf);
            self.user_ids.serialize(buf);
        }
    }

    impl crate::deserialize::Deserialize for GetUsers {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let user_ids = Vec::<i64>::deserialize(cur)?;
            Ok(Self { user_ids, })
        }
    }

    impl crate::Definition for GetUsers {}

    impl crate::Function for GetUsers {
        type Return = Vec::<super::types::User>;
    }

    pub struct SendMessage {
        pub user_id: i64,
        pub text: Option::<String>,
        pub photos: Vec::<Vec::<u8>>,
    }

    impl crate::Identify for SendMessage {
        const ID: crate::Id = crate::Id([48, 212, 40, 86]);
    }

    impl crate::serialize::Serialize for SendMessage {
        fn serialize(&self, buf: &mut Vec<u8>) {
            use crate::Identify;

            Self::ID.serialize(buf);
            self.user_id.serialize(buf);
            self.text.serialize(buf);
            self.photos.serialize(buf);
        }
    }

    impl crate::deserialize::Deserialize for SendMessage {
        fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {
            let user_id = i64::deserialize(cur)?;
            let text = Option::<String>::deserialize(cur)?;
            let photos = Vec::<Vec::<u8>>::deserialize(cur)?;
            Ok(Self { user_id, text, photos, })
        }
    }

    impl crate::Definition for SendMessage {}

    impl crate::Function for SendMessage {
        type Return = super::types::Message;
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
tl_types::functions::GetUsers
tl_types::functions::SendMessage
```
