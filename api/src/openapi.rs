use crate::{handler, model};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "rust-web-bookmanager",
        contact(name = "kurousa", url = "https://github.com/kurousa"),
    ),
    paths(
        handler::health::handler_health_check_api,
        handler::health::handler_health_check_db,
        handler::auth::login,
        handler::auth::logout,
        handler::book::show_book_list,
        // TODO: implement utoipa::path the following handlers
        // handler::book::show_book,
        // handler::book::register_book,
        // handler::book::update_book,
        // handler::book::delete_book,
        // handler::checkout::checkout_book,
        // handler::checkout::return_book,
        // handler::checkout::checkout_history,
        // handler::checkout::show_checked_out_list,
        // handler::user::get_current_user,
        // handler::user::list_users,
        // handler::user::change_password,
        // handler::user::register_user,
        // handler::user::delete_user,
        // handler::user::change_role,
        // handler::user::get_checkouts,
    ),
    components(schemas(
        model::auth::LoginRequest,
        model::auth::AccessTokenResponse,
        model::book::CreateBookRequest,
        model::book::UpdateBookRequest,
        model::book::BookResponse,
        model::book::PaginatedBookResponse,
        model::book::BookCheckoutResponse,
        model::checkout::CheckoutsResponse,
        model::checkout::CheckoutResponse,
        model::checkout::CheckoutBookResponse,
        model::user::BookOwner,
        model::user::CheckOutUser,
        kernel::model::id::BookId,
        kernel::model::id::UserId,
        kernel::model::id::CheckoutId,
    ))
)]
pub struct ApiDoc;
