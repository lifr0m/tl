use tl::{Deserialize, Serialize};
use tl_example as tl;

// Located on server side.
mod functions {
    use tl_example as tl;

    pub fn get_users(func: tl::api::functions::GetUsers) -> Result<<tl::api::functions::GetUsers as tl::Call>::Return, tl::api::Error> {
        if func.user_ids.contains(&666) {
            Err(tl::api::Error::InvalidUserId {
                user_id: 666,
            })
        } else {
            Ok(Vec::from_iter(
                func.user_ids.iter()
                    .map(|&user_id| tl::api::types::User::User {
                        id: user_id,
                        verified: true,
                        rating: 0.74,
                    })
            ))
        }
    }
}

// Located on client side.
fn call<F: Serialize + tl::Call>(func: &F) -> Result<Result<F::Return, tl::api::Error>, tl::deserialize::Error> {
    let request = func.to_bytes();

    let response = respond(&request)?;

    Result::<F::Return, tl::api::Error>::from_bytes(&response)
}

// Located on server side.
fn respond(request: &[u8]) -> Result<Vec<u8>, tl::deserialize::Error> {
    let func = tl::api::Function::from_bytes(request)?;

    Ok(match func {
        tl::api::Function::GetUsers(func) => functions::get_users(func).to_bytes(),
        _ => unreachable!(),
    })
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), tl::deserialize::Error> {
        assert_eq!(
            call(&tl::api::functions::GetUsers {
                user_ids: vec![1],
            })?,
            Ok(vec![tl::api::types::User::User {
                id: 1,
                verified: true,
                rating: 0.74
            }])
        );

        assert_eq!(
            call(&tl::api::functions::GetUsers {
                user_ids: vec![666],
            })?,
            Err(tl::api::Error::InvalidUserId {
                user_id: 666,
            })
        );

        Ok(())
    }
}
