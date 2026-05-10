import re

with open("adapter/src/repository/auth.rs", "r") as f:
    content = f.read()

replacement = """        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?;

        let password = password.to_string();

        match user_item {
            Some(item) => {
                let hash = item.password_hash;
                let valid = tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash))
                    .await
                    .map_err(|e| AppError::InternalError(e.into()))??;
                if !valid {
                    return Err(AppError::UnauthenticatedError);
                }
                Ok(item.user_id)
            }
            None => {
                let hash = bcrypt::hash("", bcrypt::DEFAULT_COST).unwrap();
                let _ = tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash)).await;
                Err(AppError::UnauthenticatedError)
            }
        }"""

old_code = """        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::DatabaseOperationError)?
        .ok_or(AppError::UnauthenticatedError)?;

        let password = password.to_string();
        let hash = user_item.password_hash;
        let valid = tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash))
            .await
            .map_err(|e| AppError::InternalError(e.into()))??;

        if !valid {
            return Err(AppError::UnauthenticatedError);
        }

        Ok(user_item.user_id)"""

if old_code in content:
    content = content.replace(old_code, replacement)
    with open("adapter/src/repository/auth.rs", "w") as f:
        f.write(content)
    print("Patch applied")
else:
    print("Could not apply patch")
