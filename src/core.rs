use potato::{
    hooks::{messages::FacebookMessage, FacebookRequest},
    models,
    response_models::{quick_replies::QuickReplie, send, Response},
};
use rocket::serde::json::Json;

#[get("/")]
pub fn hooks_verify(request: FacebookRequest) -> String {
    request.0
}

#[post("/", format = "json", data = "<facebook_message>")]
pub async fn hooks_core(facebook_message: Json<FacebookMessage>) -> &'static str {
    let message = facebook_message.get_message();
    let facebook_user_id = facebook_message.get_sender();

    let user = models::User::default();
    user.create(&facebook_user_id);

    let response: Response = match user
        .get_action(&facebook_user_id)
        .expect("maybe there is no user in base")
        .as_str()
    {
        "/" => {
            let red = QuickReplie::new("red", "red.png");
            let blue = QuickReplie::new("blue", "blue.png");
            user.set_action(&facebook_user_id, "/pick_color");
            Response::QuickReply("pick color", vec![red, blue])
        }
        "/pick_color" => {
            user.reset_action(&facebook_user_id);
            let answer = format!("the color is {message}");
            Response::TextMessage(answer)
        }
        _ => {
            user.reset_action(&facebook_user_id);
            Response::TextMessage("action is reset".to_string())
        }
    };

    send(facebook_user_id, response).await;

    "ok"
}
