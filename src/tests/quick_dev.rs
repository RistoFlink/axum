#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Risto").await?.print().await?;

    //hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "api/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );

    req_login.await?.print().await?;

    Ok(())
}
