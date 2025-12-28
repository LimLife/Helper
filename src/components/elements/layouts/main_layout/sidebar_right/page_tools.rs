mod bookmark_action;
mod edit_on_github_action;
mod report_issue_action;
mod share_aqction;

use dioxus::prelude::*;
use super::page_tools::{
    bookmark_action::BookmarkAction,
    edit_on_github_action::EditOnGithubAction,
    report_issue_action::ReportIssueAction,
    share_aqction::ShareAction
};

#[component]
pub fn PageTools() -> Element {
    rsx!{
        BookmarkAction {},
        EditOnGithubAction {},
        ReportIssueAction {},
        ShareAction {}
    }
}