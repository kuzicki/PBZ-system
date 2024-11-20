use super::base_page::base_template;
use super::Message;
use crate::model::dao::employee::Employee;
use crate::model::dao::unit::Unit;
use maud::{html, Markup};

pub fn table_page(employee_list: Vec<Employee>) -> Markup {
    let content = html! {
        h1 { "Employee list" }

        a href="/add-employee" {
            button type="button" { "Add Employee" }
        }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "First name" }
                    th { "Middle name" }
                    th { "Last name" }
                    th { "Is supervisor" }
                    th { "Is accountable" }
                    th { "Job title" }
                    th { "Actions" }
                }
            }
            tbody {
                @for employee in employee_list {
                    tr {
                        td { (employee.id()) }
                        td { (employee.first_name) }
                        td { (employee.middle_name) }
                        td { (employee.last_name) }
                        td { (employee.is_supervisor) }
                        td { (employee.is_accountable) }
                        td { (employee.job_title) }
                        td {
                            form action={(format!("/delete-employee/{}", employee.id()))} method="POST" {
                                input type="hidden" name="_method" value="DELETE" ;
                                button type="submit" { "Delete" }
                            }

                            form action={(format!("/edit-employee/{}", employee.id()))} method="GET" {
                                button type="submit" { "Edit" }
                            }
                        }
                    }
                }
            }
        }
    };

    base_template("Employee", content)
}

pub fn table_page_view(employee_list: Vec<Employee>) -> Markup {
    let content = html! {
        h1 { "Employee list" }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "First name" }
                    th { "Middle name" }
                    th { "Last name" }
                    th { "Is supervisor" }
                    th { "Is accountable" }
                    th { "Job title" }
                }
            }


            tbody {
                @for employee in employee_list {
                    tr {
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id())) style="text-decoration=none !important"{
                                (employee.id())
                            }
                        }
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id()))  {
                                (employee.first_name)
                            }
                        }
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id())) {
                                (employee.middle_name)
                            }
                        }
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id())) {
                                (employee.last_name)
                            }
                        }
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id())) {
                                (employee.is_supervisor)
                            }
                        }
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id())) {
                                (employee.is_accountable)
                            }
                        }
                        td {
                            a href=(format!("/view-employee-tech/{}", employee.id())) {
                                (employee.job_title)
                            }
                        }
                    }
                }
            }
        };
    };
    base_template("Employee", content)
}

fn input_form_edit(employee: &Employee, units: Vec<Unit>, message: Markup) -> Markup {
    html! {
        h1 { "Edit Employee" }

        form action=({format!("/edit-employee/{}", employee.id())}) method="POST" {
            label for="first_name" { "First name:" }
            input type="text" id="first_name" name="first_name" value=(employee.first_name) required;
            br; br;

            label for="middle_name" { "Middle name:" }
            input type="text" id="middle_name" name="middle_name" value=(employee.middle_name) required;
            br; br;

            label for="last_name" { "Last name:" }
            input type="text" id="last_name" name="last_name" value=(employee.last_name) required;
            br; br;

            label for="is_supervisor" { "Is supervisor:" }
            input type="checkbox" id="is_supervisor" name="is_supervisor" checked=(employee.is_supervisor);
            br; br;

            label for="is_accountable" { "Is accountable:" }
            input type="checkbox" id="is_accountable" name="is_accountable" checked=(employee.is_accountable);
            br; br;

            label for="job_title" { "Job title:" }
            input type="text" id="job_title" name="job_title" value=(employee.job_title);
            br; br;
            label for="unit" { "Unit: "}
            select id="unit" name="unit" required {
                @for unit in units {
                    option value=(unit.id()) {
                        @if unit.id() == employee.unit_id {
                            "Now in: "
                        }
                        (unit.full_name)
                    }
                }
            }
            br; br;

            button type="submit" { "Save Changes" }
        }
        (message)

        br; br;

        a href="/employee" { "Back to Employee List" }

    }
}

pub fn edit_form_get(employee: &Employee, units: Vec<Unit>) -> Markup {
    input_form_edit(employee, units, Message::none().to_html())
}

pub fn edit_form_post(employee: &Employee, units: Vec<Unit>, message: Message) -> Markup {
    input_form_edit(employee, units, message.to_html())
}

fn input_form_add(units: Vec<Unit>, message: Markup) -> Markup {
    let content = html! {
        h1 { "Add new Employee" }
        form action="/add-employee" method="POST" {
                label for="first_name" { "First name:" }
                input type="text" id="first_name" name="first_name" required;
                br;
                label for="middle_name" { "Middle name:" }
                input type="text" id="middle_name" name="middle_name" required;
                br;
                label for="last_name" { "Last name:" }
                input type="text" id="last_name" name="last_name" required;
                br;
                label for="is_supervisor" { "Is supervisor:" }
                input type="checkbox" id="is_supervisor" name="is_supervisor";
                br;
                label for="is_accountable" { "Is accountable:" }
                input type="checkbox" id="is_accountable" name="is_accountable";
                br;
                label for="job_title" { "Job title" }
                input type="text" id="job_title" name="job_title" required;
                br;
                label for="unit" { "Select Unit:" }
                select id="unit" name="unit" required {
                    @for unit in units {
                        option value=(unit.id()) { (unit.full_name) }
                    }
                }
                br;

                button type="submit" { "Add Employee" }
            }

        (message)
        br; br;

        a href="/employee" { "Back to Employee List" }
    br; br;
    };
    base_template("Add employee", content)
}

pub fn add_form_get(units: Vec<Unit>) -> Markup {
    input_form_add(units, html! {})
}

pub fn add_form_post(units: Vec<Unit>, message: Message) -> Markup {
    input_form_add(units, html! {(message.content)})
}
