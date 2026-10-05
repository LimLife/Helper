mod bookmark_action;
mod edit_on_github_action;
mod report_issue_action;
mod share_aqction;
mod page_tools_header;
use dioxus::prelude::*;
use super::page_tools::{
    bookmark_action::BookmarkAction,
    edit_on_github_action::EditOnGithubAction,
    report_issue_action::ReportIssueAction,
    share_aqction::ShareAction,
    page_tools_header::PageToolsHeader
};

#[component]
pub fn PageTools() -> Element {
    rsx!{
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