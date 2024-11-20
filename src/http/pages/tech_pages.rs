use super::base_page::base_template;
use super::Message;
use crate::model::dao::tech::Tech;
use maud::{html, Markup};

pub fn table_page(tech_list: Vec<Tech>) -> Markup {
    let content = html! {
        h1 { "Tech list" }

        a href="/add-tech" {
            button type="button" { "Add Tech" }
        }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "Inventory Number" }
                    th { "Name" }
                    th { "Model" }
                    th { "Acquisition Date" }
                    th { "Actions" }
                }
            }
            tbody {
                @for tech in tech_list {
                    tr {
                        td { (tech.id()) }
                        td { (tech.inventory_number) }
                        td { (tech.name) }
                        td { (tech.model) }
                        td { (tech.acquisition_date) }
                        td {
                            form action={(format!("/delete-tech/{}", tech.id()))} method="POST" {
                                input type="hidden" name="_method" value="DELETE" ;
                                button type="submit" { "Delete" }
                            }

                            form action={(format!("/edit-tech/{}", tech.id()))} method="GET" {
                                button type="submit" { "Edit" }
                            }
                        }
                    }
                }
            }
        }
    };

    base_template("Tech", content)
}


pub fn table_page_view(tech_list: Vec<Tech>) -> Markup {
    let content = html! {
        h1 { "Tech list" }

        table border="1" {
            thead {
                tr {
                    th { "ID" }
                    th { "Inventory Number" }
                    th { "Name" }
                    th { "Model" }
                    th { "Acquisition Date" }
                    th { "Actions" }
                }
            }
            tbody {
                @for tech in tech_list {
                    tr {
                        td { (tech.id()) }
                        td { (tech.inventory_number) }
                        td { (tech.name) }
                        td { (tech.model) }
                        td { (tech.acquisition_date) }
                        td {
                            form action={(format!("/delete-tech/{}", tech.id()))} method="POST" {
                                input type="hidden" name="_method" value="DELETE" ;
                                button type="submit" { "Delete" }
                            }

                            form action={(format!("/edit-tech/{}", tech.id()))} method="GET" {
                                button type="submit" { "Edit" }
                            }
                        }
                    }
                }
            }
        }
    };

    base_template("Tech", content)
}


fn input_form_add(message: Markup) -> Markup {
    let content = html! {
        h1 { "Add new Tech" }
        form action="/add-tech" method="POST" {
                label for="inventory_number" { "Inventory Number:" }
                input type="number" id="inventory_number" name="inventory_number" required;
                br;
                label for="name" { "Name:" }
                input type="text" id="name" name="name" required;
                br;
                label for="model" { "Model:" }
                input type="text" id="model" name="model" required;
                br;
                label for="acquisition_date" { "Acquisition Date:" }
                input type="date" id="acquisition_date" name="acquisition_date" required;
                br;
                label for="price" { "Price:" }
                input type="number" id="price" name="price" required;
                br;
                button type="submit" { "Add Tech" }
            }

        (message)
        br; br;

        a href="/tech" { "Back to Tech List" }
    };
    base_template("Add tech", content)
}

pub fn add_form_get() -> Markup {
    input_form_add(Message::none().to_html())
}

pub fn add_form_post(message: Message) -> Markup {
    input_form_add(message.to_html())
}

fn input_form_edit(tech: &Tech, message: Markup) -> Markup {
    html! {
        h1 { "Edit Tech Item" }

        form action=({format!("/edit-tech/{}", tech.id())}) method="POST" {
            label for="inventory_number" { "Inventory Number:" }
            input type="number" id="inventory_number" name="inventory_number" value=(tech.inventory_number) required;
            br; br;

            label for="name" { "Name:" }
            input type="text" id="name" name="name" value=(tech.name) required;
            br; br;

            label for="model" { "Model:" }
            input type="text" id="model" name="model" value=(tech.model) required;
            br; br;

            label for="acquisition_date" { "Acquisition Date:" }
            input type="date" id="acquisition_date" name="acquisition_date" value=(tech.acquisition_date) required;
            br; br;

            label for="price" { "Price:" }
            input type="number" id="price" name="price" value=(tech.price) required;
            br; br;

            button type="submit" { "Save Changes" }
        }

        (message)
        br; br;

        a href="/tech" { "Back to Tech List" }

    }
}

pub fn edit_form_get(tech: &Tech) -> Markup {
    input_form_edit(tech, Message::none().to_html())
}

pub fn edit_form_post(tech: &Tech, message: Message) -> Markup {
    input_form_edit(tech, message.to_html())
}
