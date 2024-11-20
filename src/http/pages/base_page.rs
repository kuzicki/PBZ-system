use maud::{html, Markup, DOCTYPE};

pub fn base_template(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                style {
                    // Inline CSS
                    r#"
                    body {
                        font-family: Arial, sans-serif;
                        background-color: #f4f4f4;
                        margin: 0;
                        padding: 0;
                    }

                    header {
                        background-color: #333;
                        color: white;
                        padding: 10px 0;
                        text-align: center;
                    }

                    nav a {
                        color: white;
                        text-decoration: none;
                        margin: 0 15px;
                    }

                    nav a:hover {
                        text-decoration: underline;
                    }

                    main {
                        padding: 20px;
                    }

                    footer {
                        background-color: #333;
                        color: white;
                        padding: 10px 0;
                        text-align: center;
                        position: fixed;
                        width: 100%;
                        bottom: 0;
                        height: 50px; /* Keeps space for footer */
                    }

                    table {
                        width: 100%;
                        border-collapse: collapse;
                        margin-top: 20px;
                    }

                    table, th, td {
                        border: 1px solid #ddd;
                    }

                    th, td {
                        padding: 8px;
                        text-align: left;
                    }

                    th {
                        background-color: #f2f2f2;
                    }

                    tr:hover {
                        background-color: #f5f5f5;
                    }
                    "# // End of CSS
                }
            }
            body {
                header {
                    h1 { "My Website" }
                    nav {
                        a href="/" { "Home" }
                        a href="/tech" { "Tech" }
                        a href="/unit" { "Unit" }
                        a href="/employee" { "Employee" }
                        a href="/transfer" { "Transfer" }
                        a href="/unit-tech" { "Unit tech" }
                        a href="/employee-tech" { "Employee tech" }
                    }
                }
                main {
                    (content)
                }
                footer {
                    ""
                }
            }
        }
    }
}

pub fn method_not_allowed() -> Markup {
    base_error_template("Method not allowed", "Method not allowed")
}

pub fn base_error_template(title: &str, error_name: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css";
            }
            body {
                header {
                    h1 { "My Website" }
                    nav {
                        a href="/" { "Home" }
                        a href="/tech" { "Tech" }
                        a href="/unit" { "Unit" }
                        a href="/employee" { "Employee" }
                        a href="/transfer" { "Transfer" }
                        a href="/unit-tech" { "Unit tech" }
                        a href="/employee-tech" { "Employee tech" }
                    }
                }
                main {
                    h2 { "An error has occured: "}
                    p {(error_name)}
                }
                footer {
                    p { "Footer information here." }
                }
            }
        }
    }
}
