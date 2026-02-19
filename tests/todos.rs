// use serde_json::json;
// use tower::ServiceExt; // oneshot

// use tudu::{build_app, build_state};
// mod common;
// use common::{bearer_req, json_req, read_json, test_pool};

// #[tokio::test]
// async fn create_todo_ok() {
//     let pool = test_pool().await;
//     let state = build_state(pool).await;
//     let app = build_app(state);

//     // register
//     let res = app
//         .clone()
//         .oneshot(json_req(
//             "POST",
//             "/auth/register",
//             json!({
//                 "name": "mladen",
//                 "email": "a@a.com",
//                 "password_hash": "pass12345"
//             }),
//         ))
//         .await
//         .unwrap();
//     assert_eq!(res.status(), 201);

//     // login
//     let res = app
//         .clone()
//         .oneshot(json_req(
//             "POST",
//             "/auth/login",
//             json!({
//                 "email": "a@a.com",
//                 "password": "pass12345"
//             }),
//         ))
//         .await
//         .unwrap();
//     assert_eq!(res.status(), 200);
//     let v = read_json(res).await;
//     let token = v["token"].as_str().unwrap().to_string();

//     // create category
//     let req = bearer_req(
//         json_req(
//             "POST",
//             "/api/categories",
//             json!({
//                 "name": "Work"
//             }),
//         ),
//         &token,
//     );
//     let res = app.clone().oneshot(req).await.unwrap();
//     assert_eq!(res.status(), 201);
//     let cat = read_json(res).await;
//     let category_id = cat["id"].as_i64().unwrap();

//     // create todo
//     let req = bearer_req(
//         json_req(
//             "POST",
//             "/api/todos",
//             json!({
//                 "title": "My todo",
//             }),
//         ),
//         &token,
//     );

//     let res = app.oneshot(req).await.unwrap();
//     assert_eq!(res.status(), 201);

//     let todo = read_json(res).await;
//     assert_eq!(todo["title"], "My todo");
//     assert_eq!(todo["categoryId"], category_id);
// }
