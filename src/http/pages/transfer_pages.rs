use super::base_page::base_template;
use super::{Message, Session};
use crate::model::dao::room::Room;
use crate::model::dao::tech::Tech;
use crate::model::dao::transfer::{TransferDetails, TransferUnit};
use crate::model::dao::unit::Unit;
use maud::{html, Markup};

fn get_or_def<T, F>(option: &Option<T>, default: &str, accessor: F) -> String
where
    F: Fn(&T) -> String,
{
    option
        .as_ref()
        .map_or(default.to_string(), |value| accessor(value).to_string())
}

pub fn table_page(transfer_details: Vec<TransferDetails>) -> Markup {
    let content = html! {
        h1 { "Transfer list" }

        a href="/add-transfer" {
            button type="button" { "Add Transfer" }
        }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "Transfer date" }
                    th { "Tech" }
                    th { "Room" }
                    th { "Employee" }
                    th { "Unit" }
                    th { "Actions" }
                }
            }
            tbody {
                @for transfer in transfer_details {
                    tr {
                        td { (transfer.id) }
                        td { (transfer.transfer_date) }
                        td { (get_or_def(&transfer.tech, "No tech data", |tech| tech.name.as_str().to_string())) }
                        td { (get_or_def(&transfer.room, "No room data", |room| format!("number: {}, squares: {}", room.room_number, room.squares))) }
                        td { (get_or_def(&transfer.employee, "No employee data", |employee| format!("{} {} {}", employee.first_name.clone(), employee.middle_name.clone(), employee.last_name.clone()))) }
                        td { (get_or_def(&transfer.unit, "No unit data", |unit| unit.full_name.clone())) }
                        td {
                            form action={(format!("/edit-transfer/{}", transfer.id))} method="GET" {
                                button type="submit" { "Edit" }
                            }
                        }
                    }
                }
            }
        }
    };

    base_template("Employee table", content)
}

fn input_form_edit(
    tech: Vec<Tech>,
    units: Vec<Unit>,
    rooms: Vec<Room>,
    transfer: TransferUnit,
    session: Option<Session>,
    message: Markup,
) -> Markup {
    let is_unit = session.is_none();

    let content = html! {
        h1 { "Edit Transfer" }

        form action=(format!("/edit-transfer/{}", transfer.inner.id())) method="POST" {

            @if is_unit  {
                label for="date" { "Transfer Date: " }
                input type="date" id="date" name="date" value=(transfer.inner.transfer_date) required;

                br; br;

                label for="tech" { "Tech: " }
                select id="tech" name="tech" required {
                    @for t in tech {
                        option value=(t.id()) selected[t.id() == transfer.inner.tech_id]{
                            (t.name)
                        }
                    }
                }
                br; br;
                label for="unit" { "Unit: " }
                select id="unit" name="unit" required {
                    @for unit in units {
                        option value=(unit.id()) selected[unit.id() == transfer.unit_id] {
                            (unit.full_name)
                        }
                    }
                }
                br; br;

                label for="room" { "Room: " }
                select id="room" name="room" required {
                    @for room in rooms {
                        option value=(room.id()) selected[room.id() == transfer.inner.room_id]{
                            (format!("Room: {}, {} sq. meters", room.room_number, room.squares))
                        }
                    }
                }
                br; br;

            } @else {
                @let session = session.unwrap();

                label for="_date" { "Transfer Date: " }
                input type="date" id="_date" name="_date" value=(session.date) disabled;
                input type="hidden" id="date" name="date" value=(session.date);
                input type="hidden" id="unit_selected" name="unit_selected" value=(true);

                br; br;
                label for="_tech" { "Tech: " }
                input type="text" id="_tech" name="_tech" value=(tech.iter().find(|t| t.id() == session.tech_id).map(|t| t.name.clone()).unwrap_or("".to_string())) readonly;
                input type="hidden" name="tech" value=(session.tech_id);

                br; br;

                label for="_unit" { "Unit: " }
                input type="text" id="_unit" name="_unit" value=(units.iter().find(|u| u.id() == session.unit_id).map(|u| u.full_name.clone()).unwrap_or("".to_string())) readonly;
                input type="hidden" name="unit" value=(session.unit_id);

                br; br;

                label for="_room" { "Room: " }
                input type="text" id="_room" name="_room" value=(rooms.iter().find(|r| r.id() == session.room_id).map(|r| format!("Room: {}, {} sq. meters", r.room_number, r.squares)).unwrap_or("".to_string())) readonly;
                input type="hidden" name="room" value=(session.room_id);

                br; br;

                label for="employee" { "Employee: " }
                select id="employee" name="employee" required {
                    @for employee in session.accountable_employees {
                        option value=(employee.id()) {
                            (employee.first_name) " " (employee.middle_name) " " (employee.last_name)
                        }
                    }
                }
                br; br;
            }

            button type="submit" { "Edit Transfer" }
        }

        (message)

        br; br;

        a href="/transfer" { "Back to Transfer List" }

    };
    base_template("Edit transfer", content)
}

pub fn edit_form_get(
    tech: Vec<Tech>,
    units: Vec<Unit>,
    rooms: Vec<Room>,
    transfer: TransferUnit,
    message: Message,
) -> Markup {
    input_form_edit(tech, units, rooms, transfer, None, message.to_html())
}

pub fn edit_form_post(
    tech: Vec<Tech>,
    units: Vec<Unit>,
    rooms: Vec<Room>,
    transfer: TransferUnit,
    session: Option<Session>,
    message: Message,
) -> Markup {
    input_form_edit(tech, units, rooms, transfer, session, message.to_html())
}

fn input_form_add(
    tech: Vec<Tech>,
    units: Vec<Unit>,
    rooms: Vec<Room>,
    session: Option<Session>,
    message: Markup,
) -> Markup {
    let is_unit = session.is_none();

    let content = html! {
        h1 { "Add new Transfer" }

        form action="/add-transfer" method="POST" {

            @if is_unit  {
                label for="date" { "Transfer Date: " }
                input type="date" id="date" name="date" required;

                br; br;

                label for="tech" { "Tech: " }
                select id="tech" name="tech" required {
                    @for t in tech {
                        option value=(t.id()) {
                            (t.name)
                        }
                    }
                }
                br; br;
                label for="unit" { "Unit: " }
                select id="unit" name="unit" required {
                    @for unit in units {
                        option value=(unit.id()) {
                            (unit.full_name)
                        }
                    }
                }
                br; br;

                label for="room" { "Room: " }
                select id="room" name="room" required {
                    @for room in rooms {
                        option value=(room.id()) {
                            (format!("Room: {}, {} sq. meters", room.room_number, room.squares))
                        }
                    }
                }
                br; br;

            } @else {
                @let session = session.unwrap();

                label for="_date" { "Transfer Date: " }
                input type="date" id="_date" name="_date" value=(session.date) disabled;
                input type="hidden" id="date" name="date" value=(session.date);
                input type="hidden" id="unit_selected" name="unit_selected" value=(true);

                br; br;
                label for="_tech" { "Tech: " }
                input type="text" id="_tech" name="_tech" value=(tech.iter().find(|t| t.id() == session.tech_id).map(|t| t.name.clone()).unwrap_or("".to_string())) disabled;
                input type="hidden" name="tech" value=(session.tech_id);

                br; br;

                label for="_unit" { "Unit: " }
                input type="text" id="_unit" name="_unit" value=(units.iter().find(|u| u.id() == session.unit_id).map(|u| u.full_name.clone()).unwrap_or("".to_string())) disabled;
                input type="hidden" name="unit" value=(session.unit_id);

                br; br;

                label for="_room" { "Room: " }
                input type="text" id="_room" name="_room" value=(rooms.iter().find(|r| r.id() == session.room_id).map(|r| format!("Room: {}, {} sq. meters", r.room_number, r.squares)).unwrap_or("".to_string())) disabled;
                input type="hidden" name="room" value=(session.room_id);

                br; br;

                label for="employee" { "Employee: " }
                select id="employee" name="employee" required {
                    @for employee in session.accountable_employees {
                        option value=(employee.id()) {
                            (employee.first_name) " " (employee.middle_name) " " (employee.last_name)
                        }
                    }
                }
                br; br;
            }

            button type="submit" { "Add Transfer" }
        }

        (message)

        br; br;

        a href="/transfer" { "Back to Transfer List" }

    };
    base_template("Add transfer", content)
}

pub fn add_form_get(
    tech: Vec<Tech>,
    units: Vec<Unit>,
    rooms: Vec<Room>,
    message: Message,
) -> Markup {
    input_form_add(tech, units, rooms, None, message.to_html())
}

pub fn add_form_post(
    tech: Vec<Tech>,
    units: Vec<Unit>,
    rooms: Vec<Room>,
    session: Option<Session>,
    message: Message,
) -> Markup {
    input_form_add(tech, units, rooms, session, message.to_html())
}
