use super::pages::*;
use super::{get_route_arg, Response, Status};
use crate::model::dao::*;
use crate::model::pool::PostgrePool;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::rc::Rc;

pub fn handle_welcome(method: &str) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }
    Response::ok(welcome::page())
}

pub fn handle_transfer_table(method: &str, pool: Rc<PostgrePool>) -> Response {
    if method != "GET" {
        return Response::method_not_allowed();
    }
    let transfer_dao = transfer::TransferDAOImpl::new(pool);

    let detailed = match transfer_dao.get_detailed_transfers() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    return Response::ok(transfer_pages::table_page(detailed));
}

pub fn handle_transfer_add(
    method: &str,
    body: Option<HashMap<String, String>>,
    pool: Rc<PostgrePool>,
) -> Response {
    let tech_dao = tech::TechDaoImpl::new(pool.clone());
    let unit_dao = unit::UnitDAOImpl::new(pool.clone());
    let room_dao = room::RoomDAOImpl::new(pool.clone());
    let tech = match tech_dao.get_all() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    let units = match unit_dao.get_all_acountable() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    let rooms = match room_dao.get_all() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };

    if method == "GET" {
        return Response::ok(transfer_pages::add_form_get(
            tech,
            units,
            rooms,
            Message::none(),
        ));
    } else if method == "POST" {
        let mut body = body.unwrap();
        let unit_selected = body.get("unit_selected").is_some();

        let date: NaiveDate = match body["date"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Not valid date value", MK::Error);
                return Response::ok(transfer_pages::add_form_post(
                    tech, units, rooms, None, message,
                ));
            }
        };
        let tech_id: i32 = match body["tech"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Invalid tech value", MK::Error);
                return Response::ok(transfer_pages::add_form_post(
                    tech, units, rooms, None, message,
                ));
            }
        };
        let unit_id: i32 = match body["unit"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Invalid unit value", MK::Error);
                return Response::ok(transfer_pages::add_form_post(
                    tech, units, rooms, None, message,
                ));
            }
        };

        let room_id: i32 = match body["room"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Invalid room value", MK::Error);
                return Response::ok(transfer_pages::add_form_post(
                    tech, units, rooms, None, message,
                ));
            }
        };

        if !unit_selected {
            let employee_dao = employee::EmployeeDAOImpl::new(pool.clone());
            let accountable_employees = match employee_dao.get_all_accountable(unit_id) {
                Ok(x) => x,
                Err(e) => return Response::internal_server_error(&e.to_string()),
            };

            let session = Session {
                date,
                unit_id,
                tech_id,
                room_id,
                accountable_employees,
            };
            let message = Message::new("Select an accountable employee", MK::Notify);
            let response = Response::ok(transfer_pages::add_form_post(
                tech,
                units,
                rooms,
                Some(session),
                message,
            ));
            return response;
        } else {
            let employee_id: i32 = match body["employee"].parse() {
                Ok(x) => x,
                Err(_) => {
                    let message = Message::new("Invalid employee value", MK::Error);
                    return Response::ok(transfer_pages::add_form_post(
                        tech, units, rooms, None, message,
                    ));
                }
            };
            let mut transfer = transfer::Transfer::new_date(date, tech_id, room_id, employee_id);
            let transfer_dao = transfer::TransferDAOImpl::new(pool.clone());
            let message = match transfer_dao.insert(&mut transfer) {
                Ok(()) => Message::new("Added new transfer", MK::Notify),
                Err(e) => Message::new(&format!("Erorr on adding new transfer: {}", e), MK::Error),
            };

            let response = Response::ok(transfer_pages::add_form_get(tech, units, rooms, message));
            return response;
        }
    } else {
        return Response::method_not_allowed();
    }
}

pub fn handle_transfer_edit(
    method: &str,
    route: &str,
    body: Option<HashMap<String, String>>,
    pool: Rc<PostgrePool>,
) -> Response {
    let arg = match get_route_arg::<i32>(route, 2) {
        Some(x) => x,
        None => {
            return Response::internal_server_error("Couldn't get arguemnts from url");
        }
    };

    let tech_dao = tech::TechDaoImpl::new(pool.clone());
    let unit_dao = unit::UnitDAOImpl::new(pool.clone());
    let room_dao = room::RoomDAOImpl::new(pool.clone());
    let transfer_dao = transfer::TransferDAOImpl::new(pool.clone());
    let tech = match tech_dao.get_all() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    let units = match unit_dao.get_all_acountable() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    let rooms = match room_dao.get_all() {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    let transfer = match transfer_dao.get_id_with_unit(arg) {
        Ok(data) => data,
        Err(e) => return Response::internal_server_error(&e.to_string()),
    };
    if method == "GET" {
        return Response::ok(transfer_pages::edit_form_get(
            tech,
            units,
            rooms,
            transfer,
            Message::none(),
        ));
    } else if method == "POST" {
        let mut body = body.unwrap();
        let unit_selected = body.get("unit_selected").is_some();

        let date: NaiveDate = match body["date"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Not valid date value", MK::Error);
                return Response::ok(transfer_pages::edit_form_post(
                    tech, units, rooms, transfer, None, message,
                ));
            }
        };
        let tech_id: i32 = match body["tech"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Invalid tech value", MK::Error);
                return Response::ok(transfer_pages::edit_form_post(
                    tech, units, rooms, transfer, None, message,
                ));
            }
        };
        let unit_id: i32 = match body["unit"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Invalid unit value", MK::Error);
                return Response::ok(transfer_pages::edit_form_post(
                    tech, units, rooms, transfer, None, message,
                ));
            }
        };

        let room_id: i32 = match body["room"].parse() {
            Ok(x) => x,
            Err(_) => {
                let message = Message::new("Invalid room value", MK::Error);
                return Response::ok(transfer_pages::edit_form_post(
                    tech, units, rooms, transfer, None, message,
                ));
            }
        };

        if !unit_selected {
            let employee_dao = employee::EmployeeDAOImpl::new(pool.clone());
            let accountable_employees = match employee_dao.get_all_accountable(unit_id) {
                Ok(x) => x,
                Err(e) => return Response::internal_server_error(&e.to_string()),
            };

            let session = Session {
                date,
                unit_id,
                tech_id,
                room_id,
                accountable_employees,
            };
            let message = Message::new("Select an accountable employee", MK::Notify);
            let response = Response::ok(transfer_pages::edit_form_post(
                tech,
                units,
                rooms,
                transfer,
                Some(session),
                message,
            ));
            return response;
        } else {
            let employee_id: i32 = match body["employee"].parse() {
                Ok(x) => x,
                Err(_) => {
                    let message = Message::new("Invalid employee value", MK::Error);
                    return Response::ok(transfer_pages::edit_form_post(
                        tech, units, rooms, transfer, None, message,
                    ));
                }
            };
            let transfer_dao = transfer::TransferDAOImpl::new(pool.clone());
            let mut transfer = transfer::Transfer::new_date(date, tech_id, room_id, employee_id);
            transfer.set_id(arg);
            let message = match transfer_dao.update(&mut transfer) {
                Ok(()) => Message::new("Updated transfer", MK::Notify),
                Err(e) => {
                    Message::new(&format!("Erorr on updating the transfer: {}", e), MK::Error)
                }
            };
            let transfer = match transfer_dao.get_id_with_unit(arg) {
                Ok(data) => data,
                Err(e) => return Response::internal_server_error(&e.to_string()),
            };

            let response = Response::ok(transfer_pages::edit_form_get(
                tech, units, rooms, transfer, message,
            ));
            return response;
        }
    } else {
        return Response::method_not_allowed();
    }
}
