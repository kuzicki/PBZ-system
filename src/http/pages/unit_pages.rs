use super::base_page::base_template;
use super::Message;
use crate::model::dao::unit::Unit;
use maud::{html, Markup};

pub fn table_page(unit_list: Vec<Unit>) -> Markup {
    let content = html! {
        h1 { "Unit list" }

        a href="/add-unit" {
            button type="button" { "Add Unit" }
        }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "Unit number" }
                    th { "Full name" }
                    th { "Short name" }
                    th { "Actions" }
                }
            }
            tbody {
                @for unit in unit_list {
                    tr {
                        td { (unit.id()) }
                        td { (unit.unit_number) }
                        td { (unit.full_name) }
                        td { (unit.short_name) }
                        td {
                            form action={(format!("/delete-unit/{}", unit.id()))} method="POST" {
                                input type="hidden" name="_method" value="DELETE" ;
                                button type="submit" { "Delete" }
                            }

                            form action={(format!("/edit-unit/{}", unit.id()))} method="GET" {
                                button type="submit" { "Edit" }
                            }
                        }
                    }
                }
            }
        }
    };

    base_template("Unit", content)
}

pub fn table_page_view(unit_list: Vec<Unit>) -> Markup {
    let content = html! {
        h1 { "Unit list" }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "Unit number" }
                    th { "Full name" }
                    th { "Short name" }
                }
            }


            tbody {
                @for unit in unit_list {
                    tr {
                        // Wrap each <td> in an <a> tag with the href
                        td {
                            a href=(format!("/view-unit-tech/{}", unit.id())) style="text-decoration=none !important"{
                                (unit.id())
                            }
                        }
                        td {
                            a href=(format!("/view-unit-tech/{}", unit.id()))  {
                                (unit.unit_number)
                            }
                        }
                        td {
                            a href=(format!("/view-unit-tech/{}", unit.id())) {
                                (unit.full_name)
                            }
                        }
                        td {
                            a href=(format!("/view-unit-tech/{}", unit.id())) {
                                (unit.short_name)
                            }
                        }
                    }
                }
            }
        };
    };
    base_template("Unit", content)
}


pub fn add_form_get() -> Markup {
    input_form_add(Message::none().to_html())
}

pub fn add_form_post(message: Message) -> Markup {
    input_form_add(message.to_html())
}

fn input_form_add(message: Markup) -> Markup {
    html! {
        h1 { "Add new Unit" }
        form action="/add-unit" method="POST" {
                label for="unit_number" { "Unit Number:" }
                input type="number" id="unit_number" name="unit_number" required;
                br;
                label for="full_name" { "Full name:" }
                input type="text" id="full_name" name="full_name" required;
                br;
                label for="short_name" { "Short name:" }
                input type="text" id="short_name" name="short_name" required;
                br;
                button type="submit" { "Add Unit" }
            }

        (message)
        br; br;

        a href="/unit" { "Back to Unit List" }
    }
}

pub fn edit_form_get(unit: &Unit) -> Markup {
    input_form_edit(unit, html!{})
}

pub fn edit_form_post(unit: &Unit, message: Message) -> Markup {
    input_form_edit(unit, html!{(message.content)})
}

fn input_form_edit(unit: &Unit, message: Markup) -> Markup {
    html! {
        h1 { "Edit Unit" }

        form action=({format!("/edit-unit/{}", unit.id())}) method="POST" {
            label for="unit_number" { "Unit number:" }
            input type="number" id="unit_number" name="unit_number" value=(unit.unit_number) required;
            br; br;

            label for="full_name" { "Full name:" }
            input type="text" id="full_name" name="full_name" value=(unit.full_name) required;
            br; br;

            label for="short_name" { "Short name:" }
            input type="text" id="short_name" name="short_name" value=(unit.short_name) required;
            br; br;

            button type="submit" { "Save Changes" }
        }
        (message)
        br; br;

        a href="/unit" { "Back to Unit List" }

    }
}


