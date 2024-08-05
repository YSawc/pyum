pub mod prepare;
use model_entity::oauth2_client_secret::query::Query;

#[tokio::test]
#[cfg(feature = "mock")]
async fn main() {
    let db = &prepare::prepare_mock_db();

    {
        let oauth2_client_recret = Query::find_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(oauth2_client_recret.client_id, 1);
    }
}
