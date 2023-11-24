#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;
    hc.do_get("/hello/DanhLe").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "admin",
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Test",
        }),
    );
    req_create_ticket.await?.print().await?;

    // hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
