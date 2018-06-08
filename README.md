# jira_peek
Grab top 5 tickets from a Jira Project, creates and/or switches to the branch with the same jira key.

## Installation
- Download and add to your `$PATH`.
- Set three environment variables:
  - `JIRA_HOST`: The URL of your Jira installation; example `https://example.atlassian.net`
  - `JIRA_USER`: Your Jira user name
  - `JIRA_PASS`: Your Jira password
- Have a package.json file with `bugs.jiraIdentifier` set to the project identifier in jira.

## What it does
- Sends a request to `JIRA_HOST` to get the top five issues from a project assigned to the `JIRA_USER` with a `status` of `to do` and `type` of `sub-task` order by priority.
- Displays top 5 tickets
- If you select a ticket it will display more information about the ticket.
- If you choose to start the ticket it will 
  - Fetch from `develop`
  - Try to create a new branch from develop
  - If the branch exists it will switch to that branch

## Commands
None at the moment.
  
## Notes
This is a pre-release.

.... Worst documentation ever ... 

![Worst documentation ever](https://www.giganews.com/blog/uploaded_images/image001.jpg)
