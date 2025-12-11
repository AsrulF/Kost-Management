Kost Management

A simple Rust-based management system for boarding houses (kost).
This project helps track room status, guest information, and monthly payments, including automatic reminders for unpaid tenants.

Features

Room management (number, vacancy, guest info)

Monthly payment tracking

Auto-reset payment status at the start of each month

Automatic reminder system (email / WhatsApp) for unpaid tenants

Clean and modular Rust code structure

Tech Stack

Rust

chrono for date handling

How It Works

System checks payment status for the current month

If month changes → payment status resets to false

If payment remains false → system triggers reminder (email/WA)

If paid → status set to true and date recorded
