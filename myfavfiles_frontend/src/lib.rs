use axum::body::Body;
use axum_extra::routing::SpaRouter;
use myfavfiles_common as common;

pub fn create_frontend_router() -> SpaRouter<Body> {
    SpaRouter::new("", "frontend")
}
