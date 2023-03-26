use crate::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    // 数据上锁，走完handler锁被释放
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response =
        format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
