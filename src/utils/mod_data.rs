// Database for the Kost

use chrono::{Local, NaiveDate, Datelike};

pub struct Kost {
    rooms: Vec<KostRooms>,
}

pub struct KostRooms {
    rooms_number: u32,
    vacant_status: bool,
    guest_name: String,
    guest_contact: String,
    guest_mail: String,
    start_date: NaiveDate,
    payment_status: bool,
    payment_history: PaymentHistory,
}

impl Default for KostRooms {
    fn default() -> Self {
        Self {
            rooms_number: 0,
            vacant_status: false,
            guest_name: "".to_string(),
            guest_contact: "".to_string(),
            guest_mail: "".to_string(),
            start_date: Local::now().date_naive(),
            payment_status: false,
            payment_history: PaymentHistory { payments: Vec::new(), payment_date: Vec::new() },
        }
    }
}

impl KostRooms {
    pub fn new() -> Self {
        KostRooms::default()
    }

    pub fn input_tenant(
        room: &mut KostRooms,
        guest_name: String,
        guest_contact: String,
        guest_mail: String,
    ) {
        room.vacant_status = true;
        room.guest_name = guest_name;
        room.guest_contact = guest_contact;
        room.guest_mail = guest_mail;
        room.start_date = Local::now().date_naive();
    }

    pub fn room_status(room: &KostRooms) {
        println!("
        {:<15}: {}
        {:<15}: {}
        {:<15}: {}
        {:<15}: {}
        {:<15}: {}
        {:<15}: {}
        {:<15}: {}
        ",
            "Room Number",room.rooms_number,
            "Vacant", room.vacant_status,
            "Guest Name", room.guest_name,
            "Guest Contact", room.guest_contact,
            "Guest Mail", room.guest_mail,
            "Stays", (Local::now().date_naive() - room.start_date).num_days(),
            "Payment Status", room.payment_status,
        )
    }

    pub fn payment_control(room: &mut KostRooms) {
        let today = Local::now().date_naive();

        //Check this month payment
        let this_month = room
            .payment_history
            .payment_date
            .iter()
            .any(|payment| payment.month() == today.month() && payment.year() == today.year());

        if !this_month {
            room.payment_status = false;
        }

        if room.payment_status {
            println!("This month payment was done")
        } else {
            println!("Payment was not done, initiating reminder procedure");
            send_reminder(&room.guest_mail, &room.guest_contact);
        }
            
    }
}

pub struct PaymentHistory {
    payments: Vec<String>,
    payment_date: Vec<NaiveDate>,
}


// Helper function
fn send_reminder(email: &str, contact: &str) {
    println!("Sending email to: {}", email);
    println!("Sending message to contact: {}", contact);
    /* 
        Future update:
         - Integrating to API to actually send notification to tenants mail and contact
    */
    
}