use reqwest::Client;

mod _helpers;
use _helpers::spawn_app;
use backend::features::users::model::{UserDetailsDTO, UserListItemDTO};

#[tokio::test]
async fn user_all_returns_five_seeded_entities() {
    let (addr, shutdown) = spawn_app().await;
    let client = Client::new();

    let res = client
        .get(format!("{addr}/user/all"))
        .send()
        .await
        .expect("request failed");
    assert_eq!(res.status(), 200);

    let users: Vec<UserListItemDTO> = res.json().await.expect("invalid JSON");
    assert_eq!(users.len(), 5, "Expected to receive 5 users");

    let _ = shutdown.send(());
}

#[tokio::test]
async fn user_by_id() {
    let (addr, shutdown) = spawn_app().await;
    let client = Client::new();
    let user_id = "00000000-0000-0000-0000-000000000001";

    let res = client
        .get(format!("{addr}/user/{user_id}"))
        .send()
        .await
        .expect("request failed");
    assert_eq!(res.status(), 200);

    let users: UserDetailsDTO = res.json().await.expect("invalid JSON");
    assert_eq!(users.first_name, "Danil");

    let _ = shutdown.send(());
}
