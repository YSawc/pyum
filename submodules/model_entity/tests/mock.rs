pub mod prepare;

#[tokio::test]
async fn main() {
    let db = &prepare::prepare_mock_db();
}
