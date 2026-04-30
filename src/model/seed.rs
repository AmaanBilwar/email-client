use crate::model::email::EmailSummary;

pub fn placeholder_accounts() -> Vec<&'static str> {
    vec!["Gmail", "Outlook"]
}

pub fn placeholder_emails() -> Vec<EmailSummary> {
    vec![
        EmailSummary {
            sender: "GitHub",
            subject: "PR Review Requested",
            preview: "Amaan asked you to review #42 in email-client.",
            time: "10:24 AM",
            body: "Hey,\n\nAmaan requested your review on pull request #42.\nPlease take a quick look at the UI scaffold updates.\n\nThanks,\nGitHub",
        },
        EmailSummary {
            sender: "Notion",
            subject: "Weekly Workspace Summary",
            preview: "You completed 7 tasks this week.",
            time: "9:10 AM",
            body: "Hi,\n\nHere is your weekly summary:\n- 7 tasks completed\n- 3 pages updated\n- 2 comments resolved\n\nKeep up the great work.",
        },
        EmailSummary {
            sender: "Stripe",
            subject: "Payment Receipt",
            preview: "Receipt for your monthly subscription is ready.",
            time: "Yesterday",
            body: "Hello,\n\nYour payment receipt for April is now available.\nAmount: $19.99\nStatus: Paid\n\nRegards,\nStripe Billing",
        },
        EmailSummary {
            sender: "LinkedIn",
            subject: "5 new opportunities match your profile",
            preview: "Remote Rust roles in your area.",
            time: "Yesterday",
            body: "Hi,\n\nWe found new opportunities you may like:\n- Senior Rust Engineer\n- Desktop UI Developer\n- Platform Engineer\n\nVisit LinkedIn to explore.",
        },
        EmailSummary {
            sender: "Calendar",
            subject: "Reminder: Product Sync at 4:00 PM",
            preview: "Don't forget the weekly product sync.",
            time: "Mon",
            body: "Reminder:\n\nProduct Sync starts at 4:00 PM today.\nAgenda:\n1) Inbox UI progress\n2) Account switching\n3) Next milestone",
        },
    ]
}
