use clap::Subcommand;
use anyhow::Result;
use crate::api::FBoxClient;
use crate::models::contact::AddContactRequest;
use crate::output::{OutputFormat, print_list, print_single, print_success};
use crate::t;

#[derive(Subcommand)]
pub enum ContactCmd {
    /// List all contacts
    List,
    /// Get a contact by UID
    Get {
        /// Contact UID
        uid: i64,
    },
    /// Add a new contact
    Add {
        /// Contact name
        name: String,
        /// Email address
        #[arg(long)]
        email: Option<String>,
        /// Phone number
        #[arg(long)]
        phone: Option<String>,
        /// Notice type: 0=none, 1=sms, 2=voice, 3=both
        #[arg(long, default_value = "0")]
        notice_type: i32,
    },
    /// Update a contact
    Update {
        /// Contact UID
        uid: i64,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New email
        #[arg(long)]
        email: Option<String>,
        /// New phone
        #[arg(long)]
        phone: Option<String>,
    },
    /// Delete a contact
    Delete {
        /// Contact UID
        uid: i64,
    },
}

pub async fn handle(cmd: ContactCmd, client: &mut FBoxClient, format: OutputFormat) -> Result<()> {
    match cmd {
        ContactCmd::List => {
            let contacts = crate::api::contact::list_contacts(client).await?;
            print_list(&contacts, format)?;
        }
        ContactCmd::Get { uid } => {
            let contact = crate::api::contact::get_contact(client, uid).await?;
            print_single(&contact, format)?;
        }
        ContactCmd::Add { name, email, phone, notice_type } => {
            let req = AddContactRequest {
                name,
                email,
                cellphone: phone,
                enabled: true,
                notice_type,
                memo: None,
            };
            let result = crate::api::contact::add_contact(client, &req).await?;
            print_single(&result, format)?;
        }
        ContactCmd::Update { uid, name, email, phone } => {
            let mut data = serde_json::json!({ "uid": uid });
            if let Some(n) = name { data["name"] = serde_json::json!(n); }
            if let Some(e) = email { data["email"] = serde_json::json!(e); }
            if let Some(p) = phone { data["cellphone"] = serde_json::json!(p); }
            crate::api::contact::update_contact(client, &data).await?;
            print_success(&t!("Contact updated.", "联系人已更新。"), format)?;
        }
        ContactCmd::Delete { uid } => {
            crate::api::contact::delete_contact(client, uid).await?;
            print_success(&t!("Contact deleted.", "联系人已删除。"), format)?;
        }
    }
    Ok(())
}
