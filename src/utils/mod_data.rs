// Database for the Kost

use chrono::{Local, NaiveDate, Datelike};
use crate::utils::mod_user::*;


pub struct Kosts {
    pub kost_database: Vec<Kost>,
}

impl Kosts {
    pub fn new() -> Self {
        Self { kost_database: Vec::new() }
    }
}

#[derive(Debug)]
pub struct Kost {
    pub rooms: Vec<KostRooms>,
    pub user_id: u64,
}

impl Kost {
    // num of room used to determine how many room in the Kost
    pub fn new(num_of_rooms: u32, user_id: u64) -> Self {
        let mut rooms: Vec<KostRooms> = Vec::with_capacity(num_of_rooms as usize);

        for _ in 0..num_of_rooms {
            let room_number = (rooms.len() + 1) as u32;

            let mut new_room = KostRooms::new();
            new_room.rooms_number = room_number;

            rooms.push(new_room);
        };
        
        Self {
            rooms,
            user_id,
        }
    }
}

#[derive(Debug, PartialEq)]
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
            payment_history: PaymentHistory::new(),
        }
    }
}

impl KostRooms {
    pub fn new() -> Self {
        KostRooms::default()
    }

    pub fn input_tenant(
        &mut self,
        guest_name: String,
        guest_contact: String,
        guest_mail: String,
    ) {
        self.vacant_status = true;
        self.guest_name = guest_name;
        self.guest_contact = guest_contact;
        self.guest_mail = guest_mail;
        self.start_date = Local::now().date_naive();
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

    pub fn payment_control(&mut self) {
        let today = Local::now().date_naive();

        // Check this month payment
        let this_month = self
            .payment_history
            .payment_date
            .iter()
            .any(|payment| payment.month() == today.month() && payment.year() == today.year());

        if !this_month {
            self.payment_status = false;
        }

        if self.payment_status {
            println!("This month payment was done")
        } else {
            println!("Payment was not done, initiating reminder procedure");
            send_reminder(&self.guest_mail, &self.guest_contact);
        }
            
    }
}

#[derive(Debug, PartialEq)]
pub struct PaymentHistory {
    payments: Vec<String>,
    payment_date: Vec<NaiveDate>,
}

impl PaymentHistory {
    pub fn new() -> Self {
        Self {
            payments: Vec::new(),
            payment_date: Vec::new(),
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;
    // Test for KostRooms
    #[test]
    fn new_room() {
        let expected: KostRooms = KostRooms {
            rooms_number: 0,
            vacant_status: false,
            guest_name: "".to_string(),
            guest_contact: "".to_string(),
            guest_mail: "".to_string(),
            start_date: Local::now().date_naive(),
            payment_status: false,
            payment_history: PaymentHistory::new(),
        };

        let room: KostRooms = KostRooms::new();

        assert_eq!(room, expected);
    }

    #[test]
    fn input_tenant() {
        let expected: KostRooms = KostRooms {
            rooms_number: 0,
            vacant_status: true,
            guest_name: "carrera".to_string(),
            guest_contact: "123456789".to_string(),
            guest_mail: "asdfg@gmail.com".to_string(),
            start_date: Local::now().date_naive(),
            payment_status: false,
            payment_history: PaymentHistory::new(),
        };

        let mut rooms: KostRooms = KostRooms::new();
        rooms.input_tenant(
        "carrera".to_string(), 
        "123456789".to_string(),
        "asdfg@gmail.com".to_string());

        assert_eq!(rooms, expected);
    }

    #[test]
    fn payment() {
        let mut room: KostRooms = KostRooms {
            rooms_number: 0,
            vacant_status: true,
            guest_name: "carrera".to_string(),
            guest_contact: "123456789".to_string(),
            guest_mail: "asdfg@gmail.com".to_string(),
            start_date: Local::now().date_naive(),
            payment_status: true,
            payment_history: PaymentHistory::new(),
        };

        room.payment_control();

        assert_eq!(room.payment_status, false);
    }

    // Test for Kost
    #[test]
    fn kost_rooms() {
        let expected_kost_rooms = 10;
        let user =  User {
            username: "admin1".to_string(),
            password: "123456".to_string(),
            user_role: crate::utils::mod_user::Role::Admin,
            user_id: 12345678,
        };

        let kost: Kost = Kost::new(10, user.user_id);

        let room_number = kost.rooms[3].rooms_number;

        assert_eq!(kost.rooms.len(), expected_kost_rooms);
        assert_eq!(room_number, 4u32);
    }

}