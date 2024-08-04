fn main() {}

#[cfg(test)]
mod tests {
    use sea_orm::entity::prelude::*;

    #[tokio::test]
    async fn test() -> Result<(), DbErr> {
        // create_oauth2_client_secret
        Ok(())
    }
}
