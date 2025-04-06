use tl::{Deserialize, Serialize};
use tl_types as tl;

// Located on server side.
mod functions {
    use tl_types as tl;

    pub fn get_users(func: tl::functions::GetUsers) -> <tl::functions::GetUsers as tl::Call>::Result {
        if func.user_ids.contains(&666) {
            Err(tl::Error::InvalidUserId {
                user_id: 666,
            })
        } else {
            Ok(Vec::from_iter(
                func.user_ids.iter()
                    .map(|&user_id| tl::types::User::User {
                        id: user_id,
                        verified: true,
                        rating: 0.74,
                    })
            ))
        }
    }
}

// Located on client side.
fn call<F: Serialize + tl::Call>(func: &F) -> Result<Result<F::Return, tl::Error>, tl::deserialize::Error> {
    let request = func.to_bytes();

    let response = respond(&request)?;

    Result::<F::Return, tl::Error>::from_bytes(&response)
}

// Located on server side.
fn respond(request: &[u8]) -> Result<Vec<u8>, tl::deserialize::Error> {
    let func = tl::Function::from_bytes(request)?;

    Ok(match func {
        tl::Function::GetUsers(func) => functions::get_users(func).to_bytes(),
        _ => unreachable!(),
    })
}

#[test]
fn example() -> Result<(), tl::deserialize::Error> {
    assert_eq!(
        call(&tl::functions::GetUsers {
            user_ids: vec![1],
        })?,
        Ok(vec![tl::types::User::User {
            id: 1,
            verified: true,
            rating: 0.74
        }])
    );

    assert_eq!(
        call(&tl::functions::GetUsers {
            user_ids: vec![666],
        })?,
        Err(tl::Error::InvalidUserId {
            user_id: 666,
        })
    );

    Ok(())
}
