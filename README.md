# Type Language

Like Telegram's one but simpler.

### Key differences

1. Supports errors.
2. Doesn't support bit flags.

### Example schema

```text
type Message id:int32 text:string? photos:[bytes] sent_at:time = Message
type User id:int64 verified:bool rating:float = User
type UserEmpty id:int64 = User

error InvalidUserId user_id:int64
error TooLongText text:string max_length:int32

func get_users user_ids:[int64] = [User]
func send_message user_id:int64 text:string? photos:[bytes] = Message
```

<details>
<summary>Generated code</summary>

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidUserId {
        user_id: i64,
    },
    TooLongText {
        text: String,
        max_length: i32,
    },
}

impl crate::Serialize for Error {
    fn serialize(&self, buf: &mut Vec<u8>) {
        match self {
            Self::InvalidUserId { user_id: user_id_, } => {
                2283843567_u32.serialize(buf);
                user_id_.serialize(buf);
            }
            Self::TooLongText { text: text_, max_length: max_length_, } => {
                1447747856_u32.serialize(buf);
                text_.serialize(buf);
                max_length_.serialize(buf);
            }
        };
    }
}

impl crate::Deserialize for Error {
    fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
        let id = u32::deserialize(reader)?;

        Ok(match id {
            2283843567_u32 => {
                let user_id_ = i64::deserialize(reader)?;

                Self::InvalidUserId { user_id: user_id_, }
            }
            1447747856_u32 => {
                let text_ = String::deserialize(reader)?;
                let max_length_ = i32::deserialize(reader)?;

                Self::TooLongText { text: text_, max_length: max_length_, }
            }
            _ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    GetUsers(self::functions::GetUsers),
    SendMessage(self::functions::SendMessage),
}

impl crate::Deserialize for Function {
    fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
        let id = u32::deserialize(reader)?;

        Ok(match id {
            1904452899_u32 => Self::GetUsers(self::functions::GetUsers::deserialize(reader)?),
            339054040_u32 => Self::SendMessage(self::functions::SendMessage::deserialize(reader)?),
            _ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),
        })
    }
}

pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Message {
        Message {
            id: i32,
            text: Option::<String>,
            photos: Vec::<Vec::<u8>>,
            sent_at: std::time::SystemTime,
        },
    }

    impl crate::Serialize for Message {
        fn serialize(&self, buf: &mut Vec<u8>) {
            match self {
                Self::Message { id: id_, text: text_, photos: photos_, sent_at: sent_at_, } => {
                    2225622240_u32.serialize(buf);
                    id_.serialize(buf);
                    text_.serialize(buf);
                    photos_.serialize(buf);
                    sent_at_.serialize(buf);
                }
            };
        }
    }

    impl crate::Deserialize for Message {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let id = u32::deserialize(reader)?;

            Ok(match id {
                2225622240_u32 => {
                    let id_ = i32::deserialize(reader)?;
                    let text_ = Option::<String>::deserialize(reader)?;
                    let photos_ = Vec::<Vec::<u8>>::deserialize(reader)?;
                    let sent_at_ = std::time::SystemTime::deserialize(reader)?;

                    Self::Message { id: id_, text: text_, photos: photos_, sent_at: sent_at_, }
                }
                _ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),
            })
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum User {
        User {
            id: i64,
            verified: bool,
            rating: f64,
        },
        UserEmpty {
            id: i64,
        },
    }

    impl crate::Serialize for User {
        fn serialize(&self, buf: &mut Vec<u8>) {
            match self {
                Self::User { id: id_, verified: verified_, rating: rating_, } => {
                    4055296785_u32.serialize(buf);
                    id_.serialize(buf);
                    verified_.serialize(buf);
                    rating_.serialize(buf);
                }
                Self::UserEmpty { id: id_, } => {
                    990500211_u32.serialize(buf);
                    id_.serialize(buf);
                }
            };
        }
    }

    impl crate::Deserialize for User {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let id = u32::deserialize(reader)?;

            Ok(match id {
                4055296785_u32 => {
                    let id_ = i64::deserialize(reader)?;
                    let verified_ = bool::deserialize(reader)?;
                    let rating_ = f64::deserialize(reader)?;

                    Self::User { id: id_, verified: verified_, rating: rating_, }
                }
                990500211_u32 => {
                    let id_ = i64::deserialize(reader)?;

                    Self::UserEmpty { id: id_, }
                }
                _ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),
            })
        }
    }

}

pub mod functions {
    #[derive(Debug, Clone, PartialEq)]
    pub struct GetUsers {
        pub user_ids: Vec::<i64>,
    }

    impl crate::Serialize for GetUsers {
        fn serialize(&self, buf: &mut Vec<u8>) {
            1904452899_u32.serialize(buf);
            self.user_ids.serialize(buf);
        }
    }

    impl crate::Deserialize for GetUsers {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let user_ids_ = Vec::<i64>::deserialize(reader)?;

            Ok(Self { user_ids: user_ids_, })
        }
    }

    impl crate::Call for GetUsers {
        type Return = Vec::<super::types::User>;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct SendMessage {
        pub user_id: i64,
        pub text: Option::<String>,
        pub photos: Vec::<Vec::<u8>>,
    }

    impl crate::Serialize for SendMessage {
        fn serialize(&self, buf: &mut Vec<u8>) {
            339054040_u32.serialize(buf);
            self.user_id.serialize(buf);
            self.text.serialize(buf);
            self.photos.serialize(buf);
        }
    }

    impl crate::Deserialize for SendMessage {
        fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {
            let user_id_ = i64::deserialize(reader)?;
            let text_ = Option::<String>::deserialize(reader)?;
            let photos_ = Vec::<Vec::<u8>>::deserialize(reader)?;

            Ok(Self { user_id: user_id_, text: text_, photos: photos_, })
        }
    }

    impl crate::Call for SendMessage {
        type Return = super::types::Message;
    }

}
```
</details>

### How to use

1. Clone template `tl-example` package.
2. Create schemas in `schemas` folder.
3. Create corresponding modules in `src/schemas` module.
4. Specify schemas in `build.rs`.
5. Remove `src/main.rs` / Edit `Cargo.toml` / Rename package.
6. Specify crate in dependencies of your project.
