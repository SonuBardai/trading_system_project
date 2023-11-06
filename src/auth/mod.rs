use actix_web::HttpRequest;

pub fn check_user_auth(_req: HttpRequest, _user_id: u32) -> bool {
    // No auth for now
    true
}
