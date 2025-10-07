use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Person1").await?.print().await?; // No cookie yet
    // hc.do_get("/src/main.rs").await?.print().await?;

    // Cookie is set here
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "admin"
        }),
    );
    req_login.await?.print().await?; // comment out to see errors

    // hc.do_get("/hello2/Person2").await?.print().await?; // Cookie exists

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "My first ticket"
        }),
    );
    req_create_ticket.await?.print().await?;

    // hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
