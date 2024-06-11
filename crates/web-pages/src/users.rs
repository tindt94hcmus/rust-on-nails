use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;

// Take a Vec<User> and create an HTML table.
#[component]
pub fn users(users: Vec<User>) -> Element {
    rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Email" }
                    }
                }
                tbody {
                    for user in users {
                        tr {
                            td {
                                strong {
                                    "{user.id}"
                                }
                            }
                            td {
                                "{user.email}"
                            }
                        }
                    }
                }
            }
        }
    }
}
