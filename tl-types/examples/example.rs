use tl::{Deserialize, Serialize};
use tl_types as tl;

mod functions {
    use tl_types as tl;

    pub async fn get_users(func: tl::functions::GetUsers) -> <tl::functions::GetUsers as tl::Call>::Result {
        if func.user_ids.contains(&666) {
            Err(tl::Error::InvalidUserId(tl::errors::InvalidUserId {
                user_id: 666,
            }))
        } else {
            Ok(Vec::from_iter(
                func.user_ids.iter()
                    .map(|&user_id| tl::enums::User::User(tl::types::User {
                        id: user_id,
                        verified: true,
                        rating: 0.74,
                    }))
            ))
        }
    }
}

async fn call<F: Serialize + tl::Call>(func: &F) -> Result<Result<F::Return, tl::Error>, tl::deserialize::Error> {
    let request = func.serialize_to();

    let response = respond(&request).await?;

    Result::<F::Return, tl::Error>::deserialize_from(&response)
}

async fn respond(request: &[u8]) -> Result<Vec<u8>, tl::deserialize::Error> {
    let func = tl::Function::deserialize_from(request)?;

    Ok(match func {
        tl::Function::GetUsers(func) => functions::get_users(func).await.serialize_to(),
        _ => unreachable!(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let func = tl::functions::GetUsers {
        user_ids: vec![1],
    };
    assert_eq!(call(&func).await?, Ok(vec![tl::enums::User::User(tl::types::User {
        id: 1,
        verified: true,
        rating: 0.74
    })]));

    let func = tl::functions::GetUsers {
        user_ids: vec![666],
    };
    assert_eq!(call(&func).await?, Err(tl::Error::InvalidUserId(tl::errors::InvalidUserId {
        user_id: 666,
    })));

    Ok(())
}
