mod http;
#[allow(unused_variables, dead_code, unused_imports)]
mod model;
use http::connection::open_connection;
use model::dao::tech::{self, Tech};
use model::pool::create_pool;
use std::rc::Rc;

// #[allow(dead_code)]
// fn test_pool() {
//     let pool = create_pool();
//
//     let tech_dao = tech::TechDaoImpl::new(pool);
//     let tech = &mut Tech::new(3, "New", "Wow", "2024-11-03", 128);
//     tech_dao.insert(tech);
// }

fn main() {
    let pool = Rc::new(create_pool());
    open_connection(pool);
}
