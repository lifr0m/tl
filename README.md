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
    fn serialize(&self, dst: &mut Vec<u8>) {
        match self {
            Self::InvalidUserId { user_id: user_id_, } => {
                2283843567_u32.serialize(dst);
                user_id_.serialize(dst);
            }
            Self::TooLongText { text: text_, max_length: max_length_, } => {
                1447747856_u32.serialize(dst);
                text_.serialize(dst);
                max_length_.serialize(dst);
            }
        };
    }
}

impl crate::Deserialize for Error {
    fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {
        let id = u32::deserialize(src)?;

        Ok(match id {
            2283843567_u32 => {
                let user_id_ = i64::deserialize(src)?;

                Self::InvalidUserId { user_id: user_id_, }
            }
            1447747856_u32 => {
                let text_ = String::deserialize(src)?;
                let max_length_ = i32::deserialize(src)?;

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
    fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {
        let id = u32::deserialize(src)?;

        Ok(match id {
            1904452899_u32 => Self::GetUsers(self::functions::GetUsers::deserialize(src)?),
            339054040_u32 => Self::SendMessage(self::functions::SendMessage::deserialize(src)?),
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
        fn serialize(&self, dst: &mut Vec<u8>) {
            match self {
                Self::Message { id: id_, text: text_, photos: photos_, sent_at: sent_at_, } => {
                    2225622240_u32.serialize(dst);
                    id_.serialize(dst);
                    text_.serialize(dst);
                    photos_.serialize(dst);
                    sent_at_.serialize(dst);
                }
            };
        }
    }

    impl crate::Deserialize for Message {
        fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {
            let id = u32::deserialize(src)?;

            Ok(match id {
                2225622240_u32 => {
                    let id_ = i32::deserialize(src)?;
                    let text_ = Option::<String>::deserialize(src)?;
                    let photos_ = Vec::<Vec::<u8>>::deserialize(src)?;
                    let sent_at_ = std::time::SystemTime::deserialize(src)?;

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
        fn serialize(&self, dst: &mut Vec<u8>) {
            match self {
                Self::User { id: id_, verified: verified_, rating: rating_, } => {
                    4055296785_u32.serialize(dst);
                    id_.serialize(dst);
                    verified_.serialize(dst);
                    rating_.serialize(dst);
                }
                Self::UserEmpty { id: id_, } => {
                    990500211_u32.serialize(dst);
                    id_.serialize(dst);
                }
            };
        }
    }

    impl crate::Deserialize for User {
        fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {
            let id = u32::deserialize(src)?;

            Ok(match id {
                4055296785_u32 => {
                    let id_ = i64::deserialize(src)?;
                    let verified_ = bool::deserialize(src)?;
                    let rating_ = f64::deserialize(src)?;

                    Self::User { id: id_, verified: verified_, rating: rating_, }
                }
                990500211_u32 => {
                    let id_ = i64::deserialize(src)?;

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
        fn serialize(&self, dst: &mut Vec<u8>) {
            1904452899_u32.serialize(dst);
            self.user_ids.serialize(dst);
        }
    }

    impl crate::Deserialize for GetUsers {
        fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {
            let user_ids_ = Vec::<i64>::deserialize(src)?;

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
        fn serialize(&self, dst: &mut Vec<u8>) {
            339054040_u32.serialize(dst);
            self.user_id.serialize(dst);
            self.text.serialize(dst);
            self.photos.serialize(dst);
        }
    }

    impl crate::Deserialize for SendMessage {
        fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {
            let user_id_ = i64::deserialize(src)?;
            let text_ = Option::<String>::deserialize(src)?;
            let photos_ = Vec::<Vec::<u8>>::deserialize(src)?;

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
