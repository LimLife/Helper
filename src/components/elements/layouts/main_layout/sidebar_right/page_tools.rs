mod bookmark_action;
mod edit_on_github_action;
mod page_tools_header;
mod report_issue_action;
mod share_aqction;
use super::page_tools::{
    bookmark_action::BookmarkAction, edit_on_github_action::EditOnGithubAction,
    page_tools_header::PageToolsHeader, report_issue_action::ReportIssueAction,
    share_aqction::ShareAction,
};
use dioxus::prelude::*;
#[component]
pub fn PageTools() -> Element {
    rsx! {
        div { class: "page-tools",
            PageToolsHeader {}
            div { class: "page-tools-actions",
                BookmarkAction {}
                EditOnGithubAction {}
                ReportIssueAction {}
                ShareAction {}
            }
        }
    }
}
