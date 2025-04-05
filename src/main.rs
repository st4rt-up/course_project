use std::env;

use rand::prelude::*;
pub use rusqlite::{
    params, 
    Result};

use rusqlite::Connection;

const REFRESH_DB: bool = false;
fn main() -> Result <()> {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");

    if REFRESH_DB {
        clear_tables()?; // for testing purposes
        init_tables()?;
        
        println!("working...");
        // populate_db()?;
        println!("done!");
    }
    

    example_query_1();

    Ok(())
}

fn connect() -> Result<Connection> {
    Connection::open("hotel_database.db")
}

pub struct HotelChain {
    pub name: String,
    pub address: String,
    pub email: String,
    pub phone_nums: Vec<String>
}

impl HotelChain {
    fn get_phone_numbers(&self) -> Result<Vec<String>> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
            SELECT chain_phone 
            FROM HotelChainPhone
            WHERE chain_name = ?1
        ")?;

        let hc_phone = stmt.query_map(params![self.name], |row| {
            row.get(0)
        })?.collect::<Result<Vec<String>>>()?;

        println!("{:?}", hc_phone);
        Ok(hc_phone)
    }

    fn load_phone_numbers(&mut self) -> Result<()> {
        self.phone_nums = self.get_phone_numbers()?;
        Ok(())
    }

    pub fn get_from_name(name: &str) -> Result<HotelChain> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
            SELECT 
                hc.chain_name, 
                hc.chain_addr, 
                hc.chain_email,
                hc.phone_nums
            FROM HotelChain hc
            WHERE hc.chain_name = ?1
        ")?;

        match stmt.query_row(params![name], |row| {
            Ok(HotelChain{
                name: row.get(0)?,
                address: row.get(1)?,
                email: row.get(2)?,
                phone_nums: vec![],
            })
        }) {
            Err(e) => Err(e),
            Ok(mut hc) => {
                hc.load_phone_numbers()?;
                Ok(hc)
            }
        }
    }

    pub fn get_from_primary_key(key: &str) -> Result<HotelChain> {
        // wrapper
        HotelChain::get_from_name(key)
    }
    
    pub fn check_exists(key: &str) -> bool {
        match HotelChain::get_from_primary_key(key) {
            Ok(_c) => true,
            Err(_e) => false,
        }
    }

    pub fn primary_key(&self) -> String { self.name.clone() }

    pub fn insert(chain: &HotelChain) -> Result<()> {
        let db_conn = connect()?;
    
        db_conn.execute("INSERT INTO HotelChain (
            chain_name,
            chain_addr,
            chain_email
        ) VALUES (?1, ?2, ?3)", (
            &chain.name,
            &chain.address,
            &chain.email
        ))?;
    
        for num in chain.phone_nums.iter() {
            db_conn.execute("INSERT INTO HotelChainPhone (
                chain_name,
                chain_phone
            ) VALUES (?1, ?2)", (&chain.name, &num))?;
        }
    
        Ok(())
    }
}

pub struct Hotel {
    pub chain: String,
    
    pub address: String,
    pub phone: String,
    pub email: String,

    pub room_amount: i32,
    pub rating: Option<i32>,
}

impl Hotel {
    pub fn get_chain(&self) -> Result<HotelChain> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
            SELECT 
                hc.chain_name, 
                hc.chain_addr, 
                hc.chain_email
            FROM HotelChain hc JOIN Hotel h
            WHERE h.chain_name = hc.chain_name AND h.h_addr = ?1
        ")?;

        let hc_result = stmt.query_row(params![self.address], |row| {
            Ok(HotelChain{
                name: row.get(0)?,
                address: row.get(1)?,
                email: row.get(2)?,
                phone_nums: vec![]
            })
        });

        match hc_result {
            Ok(mut hc) => {
                hc.load_phone_numbers()?;
                
                println!("plus + {}, {}, {} ,{:?}", &hc.name, &hc.address, &hc.email, &hc.phone_nums);

                Ok(hc)
            }
            Err(e) => Err(e)
        }

    }

    pub fn get_from_address(address: &str) -> Result<Hotel> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
        SELECT 
            h_addr, 
            h_phone, 
            h_email, 
            h_room_amt, 
            h_rating,
            chain_name
        FROM Hotel
        WHERE h_addr = ?1
    ")?;

    let hotel_result = stmt.query_row(params![address], |row| {
        Ok( Hotel {
            address: row.get(0)?,
            phone: row.get(1)?,
            email: row.get(2)?,
            room_amount: row.get(3)?,
            rating: row.get(4)?,
            chain: row.get(5)?,
        })
    });
        hotel_result
    } 
    
    pub fn get_from_primary_key(key: &str) -> Result<Hotel> {
        Hotel::get_from_address(key)
    }
    
    pub fn check_exists(key: &str) -> bool {
        match Hotel::get_from_primary_key(key) {
            Ok(_c) => true,
            Err(_e) => false,
        }
    }

    pub fn primary_key(&self) -> String { self.address.clone() }

    pub fn insert(hotel: &Hotel) -> Result<()> {
        let db_conn = connect()?;

        // handle nullables
    
        db_conn.execute("INSERT INTO Hotel (
            h_addr,
            h_room_amt,
            h_email,
            h_phone,
            chain_name
        ) VALUES (?1, ?2, ?3, ?4, ?5)", (
            &hotel.address,
            &hotel.room_amount,
            &hotel.email,
            &hotel.phone,
            &hotel.chain,
        ))?;
    
        // handle nullables after
        // update the entry that was just made using primary key
        match &hotel.rating {
            Some(rating) => db_conn.execute(
                &format!("
                UPDATE Hotel 
                SET h_rating = {} 
                WHERE h_addr = '{}';", rating, &hotel.address), []
            )?,
            None => 0,
        };
    
        Ok(())
    }

    pub fn get_all_hotels() -> Result<Vec<Hotel>> {
        let db_conn = connect()?;
        let mut stmt = db_conn.prepare("
        SELECT * FROM Hotel")?;
        let hotel_iter = stmt.query_map([], |row| {
            Ok( Hotel {
                address: row.get(0)?,
                phone: row.get(1)?,
                email: row.get(2)?,
                room_amount: row.get(3)?,
                rating: row.get(4)?,
                chain: row.get(5)?,
            })
        })?;
        let mut h_vec = vec![];
        for h in hotel_iter {
            let h_new = h?;
            h_vec.push(h_new);
        }

        Ok(h_vec)
    }

    pub fn get_all_hotel_names() -> Vec<String>{
        let mut names = vec![];

        let hotel_list = match Hotel::get_all_hotels() {
            Err(_e) => {
                println!("not supposed to happen");
                vec![]
            },
            Ok(hl) => hl,
        };
        for h in hotel_list {
            names.push(format!("{}, {}", h.chain.clone(), h.address.clone()))
        }
        names
    }
}


pub struct Room {
    pub hotel: String,

    pub room_number: i32,
    pub price: i32, // stored as cents
    pub capacity: i32,

    pub amenities: String,
    pub damages: Option<String>,
    pub other_info: Option<String>,
}

impl Room {
    pub fn get_hotel(&self) -> Result<Hotel> {
        Hotel::get_from_address(&self.hotel)
    }

    pub fn get_from_primary_key((address, room_number): (&str, &i32)) -> Result<Room> {
        let db_conn = connect()?;
        
        let mut stmt = db_conn.prepare("
            SELECT 
                r_number,
                r_price,
                r_capacity,
                r_amenities,
                r_damages,
                r_other_info,
                h_addr
            FROM Room
            WHERE r_number = ?1 AND h_addr = ?2
        ")?;

        let room_result = stmt.query_row(params!(address, room_number), |row| {
            Ok( Room {
                room_number: row.get(0)?,
                price: row.get(1)?,
                capacity: row.get(2)?,
                amenities: row.get(3)?,
                damages: row.get(4)?,
                other_info: row.get(5)?,
                hotel: row.get(6)?,
            })
        });
            room_result
    }

    pub fn check_exists((address, room_number): (&str, &i32)) -> bool {
        match Room::get_from_primary_key((address, room_number)) {
            Ok(_c) => true,
            Err(_e) => false,
        }
    }

    pub fn primary_key(&self) -> (String, i32) { 
        (self.hotel.clone(), self.room_number.clone()) 
    }

    pub fn insert(room: &Room, hotel: &Hotel) -> Result<()> {
        let db_conn = connect()?;

        db_conn.execute("INSERT INTO Room (
            r_number,
            r_price,
            r_capacity,
            r_amenities,
            h_addr
        ) VALUES (?1, ?2, ?3, ?4, ?5)", (
            &room.room_number,
            &room.price,
            &room.capacity,
            &room.amenities,
            &hotel.address,
        ))?;
    
    
        // update nullables after
        // see create_hotel
        match &room.damages {
            Some(damages) => db_conn.execute(
                &format!("
                UPDATE Room 
                SET r_damages = {} 
                WHERE h_addr = {} AND r_number = {}", 
                damages, &hotel.address, &room.room_number,), []
            )?,
            None => 0,
        };
    
        match &room.other_info {
            Some(damages) => db_conn.execute(
                &format!("
                UPDATE Room 
                SET r_other_info = {} 
                WHERE h_addr = {} AND r_number = {}", 
                damages, &hotel.address, &room.room_number,), []
            )?,
            None => 0,
        };
    
        Ok(())
    }
}

pub struct Customer {
    pub id: String,

    pub name: String,
    pub address: String,
    pub registration_date: String,
}

impl Customer {
    pub fn get_from_primary_key(key: &str) -> Result<Customer> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
            SELECT 
                c_id,
                c_name,
                c_address,
                c_reg_date
            FROM Customer
            WHERE cust_id = ?1
        ")?;

        // i wish i just did this with cursor ai or something instead of hand typing it
        let customer_result = stmt.query_row(params!(key), |row| {
            Ok( Customer {
                id: row.get(0)?,
                name: row.get(1)?,
                address: row.get(2)?,
                registration_date: row.get(3)?,
            })
        });
            customer_result
    }
    
    pub fn check_exists(key: &str) -> bool {
        // there HAS to be some way to automate this I just don't know how yet
        match Customer::get_from_primary_key(key) {
            Ok(_c) => true,
            Err(_e) => false,
        }
    }

    pub fn primary_key(&self) -> String { self.id.clone() }

    pub fn insert(customer: &Customer) -> Result<()> {
        let db_conn = connect()?;

        db_conn.execute("INSERT INTO Customer (
            c_id,
            c_name,
            c_addr,
            c_reg_date
        ) VALUES (?1, ?2, ?3, ?4)", (
           &customer.id,
           &customer.name,
           &customer.address,
           &customer.registration_date,
        ))?;
    
        Ok(())
    }
}

pub struct Employee {
    pub id: String,

    pub name: String,
    pub address: String,
    pub hotel: String,

    pub roles: Option<String>,

}

impl Employee {
    pub fn get_from_primary_key(key: &str) -> Result<Employee> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
            SELECT 
                e_id,
                e_name,
                e_address,
                e_roles,
                h_addr
            FROM Employee
            WHERE cust_id = ?1
        ")?;

        let employee_result = stmt.query_row(params!(key), |row| {
            Ok( Employee {
                id: row.get(0)?,
                name: row.get(1)?,
                address: row.get(2)?,
                roles: row.get(3)?,
                hotel: row.get(4)?
            })
        });
            employee_result
    }

    pub fn check_exists(key: &str) -> bool {
        match Employee::get_from_primary_key(key) {
            Ok(_c) => true,
            Err(_e) => false,
        }
    }

    pub fn primary_key(&self) -> String { self.id.clone() }

    pub fn insert(employee: &Employee, hotel: &Hotel) -> Result<()> {
        let db_conn = connect()?;

        db_conn.execute("INSERT INTO Employee (
            e_id,
            e_name,
            e_address,
            h_addr
        ) VALUES (?1, ?2, ?3, ?4)", (
            &employee.id,
            &employee.name,
            &employee.address,
            &hotel.address
        ))?;
    
        match &employee.roles {
            Some(roles) => db_conn.execute(
                &format!("
                UPDATE Employee 
                SET e_roles = '{}' 
                WHERE e_id = '{}' AND h_addr = '{}'", 
                roles, &employee.id, &hotel.address), []
            )?,
            None => 0,
        };
        
        Ok(())
    }
}

pub struct Manager {
    pub employee_id: i32,
    pub hotel: String,
}

impl Manager {
    pub fn insert(employee: &i32, hotel: &str) -> Result<()> {
        if Hotel::check_exists(hotel) {
            ()
        }

        Ok(())
    }
}

pub struct Reservation {
    pub room: (i32, String),

    pub start_date: String,
    pub end_date: String,

    pub is_renting: bool,

    pub customer: String,
    pub payment_info: String,

    pub related_employee: Option<i32>,
}

impl Reservation {
    
    pub fn insert(res: &Reservation) -> Result <()>{
        let db_conn = connect()?;

        db_conn.execute(
            "INSERT INTO Reservation (
                r_number,
                h_addr,
                res_start_date, 
                res_end_date, 
                is_renting, 
                c_id, 
                payment_info, 
                e_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", (
                &res.room.0,
                &res.room.1,
                &res.start_date,
                &res.end_date,
                &res.is_renting,
                &res.customer,
                &res.payment_info,
                0,
            ))?;

        match &res.related_employee {
            Some(employee) => db_conn.execute(
                &format!("
                UPDATE Reservation
                SET e_id = '{}' 
                WHERE c_id = '{}' AND res_start_date = '{}'", 
                employee, &res.customer, &res.start_date), []
            )?,
            None => 0,
        };
        
        Ok(())
    }

    pub fn get_from_primary_key((c_id, start_date): (&str, &str)) -> Result<Reservation> {
        let db_conn = connect()?;

        let mut stmt = db_conn.prepare("
            SELECT 
                r_number,
                h_addr,
                res_start_date,
                res_end_date,
                is_renting,
                c_id,
                payment_info,
                e_id
            FROM Reservation
            WHERE c_id = ?1 AND res_start_date = ?2
        ")?;

        let res_result = stmt.query_row(params!(c_id, start_date), |row| {
            Ok( Reservation {
                room: (row.get(0)?, row.get(1)?),
                start_date: row.get(2)?,
                end_date: row.get(3)?,
                is_renting: row.get(4)?,
                customer: row.get(5)?,
                payment_info: row.get(6)?,
                related_employee: row.get(7)?,
            })
        });
            res_result
    } 

    pub fn primary_key(&self) -> (String, String) {
        (self.customer.clone(), self.start_date.clone())
    }
}

pub struct ReservationArchive {
    pub archive_id: i32,

    pub hotel_address: String,
    pub room_number: i32,
    pub customer_id: i32,

    pub start_date: String,
    pub end_date: String,
}

impl ReservationArchive {
    pub fn archive_by_reservation(res: Reservation) {}

    pub fn archive_by_key((address, room_number): (&str, &str) ) -> Result<()> {
        let res = Reservation::get_from_primary_key((address, room_number))?;
        Ok(())
    }
    
    pub fn primary_key(&self) -> &i32 { &self.archive_id }

}

pub fn clear_tables() -> Result<()> {
    let db_conn = connect()?;
    
    let table_names  = vec![
        "ReservationArchive",
        "Reservation",
        "Manager",
        "Employee",
        "Customer",
        "Room",
        "Hotel",
        "HotelChainPhone",
        "HotelChain"
        ];

    for table in table_names.iter() {
        let query = format!("DROP TABLE IF EXISTS {table}");
        db_conn.execute (&query,[])?;
    }
    
    Ok(())
}

pub fn init_tables() -> Result<()> {
    let db_conn = connect()?;

    // seperate statements for easier debugging
    
    db_conn.execute (
        "CREATE TABLE IF NOT EXISTS HotelChain (
            chain_name TEXT UNIQUE NOT NULL,
            chain_addr TEXT UNIQUE NOT NULL,
            chain_email TEXT NOT NULL,
            PRIMARY KEY (chain_name)
        )",[],
    )?;

    db_conn.execute(
        "CREATE TABLE IF NOT EXISTS HotelChainPhone (
            chain_name TEXT NOT NULL,
            chain_phone TEXT UNIQUE NOT NULL,
            PRIMARY KEY (chain_phone),
            FOREIGN KEY (chain_name) REFERENCES HotelChain(chain_name)
        )",[],
    )?;

    db_conn.execute (
        "CREATE TABLE IF NOT EXISTS Hotel (
            h_addr TEXT UNIQUE NOT NULL,
            h_room_amt INT NOT NULL,
            h_email TEXT NOT NULL,
            h_phone TEXT NOT NULL,
            h_rating INT,
            chain_name TEXT NOT NULL,
            PRIMARY KEY (h_addr),
            FOREIGN KEY (chain_name) REFERENCES HotelChain(chain_name),
            CHECK (h_rating >= 1 AND h_rating <= 5),
            CHECK (h_room_amt > 0)
        )",[]
    )?;

    // trigger 1
    // 
    db_conn.execute ("
        CREATE TRIGGER IF NOT EXISTS prevent_hc_delete
        BEFORE DELETE ON HotelChain
        FOR EACH ROW
        BEGIN
            SELECT CASE
                WHEN (SELECT COUNT(*) FROM Hotel WHERE chain_name = OLD.chain_name) > 0
                THEN RAISE(ABORT, 'Cannot delete hotel chain with associated hotels.')
            END;
        END;", []
    )?;

    db_conn.execute (
        "CREATE TABLE IF NOT EXISTS Room (
        r_number TEXT NOT NULL,
        r_price INT NOT NULL,
        r_capacity INT NOT NULL,
        r_amenities TEXT NOT NULL,
        r_damages TEXT,
        r_other_info TEXT,
        h_addr TEXT NOT NULL,
        PRIMARY KEY (h_addr, r_number),
        FOREIGN KEY (h_addr) REFERENCES Hotel(h_addr),
        CHECK (r_capacity >= 1),
        CHECK (r_price >= 0)
        )", []
    )?;

    db_conn.execute (
        "CREATE TABLE IF NOT EXISTS Customer (
        c_id TEXT UNIQUE NOT NULL,
        c_name TEXT NOT NULL,
        c_addr TEXT NOT NULL,
        c_reg_date DATE NOT NULL,
        PRIMARY KEY (c_id)
        )", []
    )?;

    db_conn.execute (
        "CREATE TABLE IF NOT EXISTS Employee (
        e_id TEXT UNIQUE NOT NULL,
        e_name TEXT NOT NULL,
        e_address TEXT NOT NULL,
        e_roles TEXT,
        h_addr TEXT NOT NULL,
        PRIMARY KEY (e_id, h_addr),
        FOREIGN KEY (h_addr) REFERENCES Hotel(h_addr)
        )", []
    )?;

    db_conn.execute (
        "CREATE TABLE IF NOT EXISTS Manager (
        e_id INT UNIQUE NOT NULL, 
        h_addr TEXT UNIQUE NOT NULL,
        PRIMARY KEY (e_id, h_addr),
        FOREIGN KEY (e_id) REFERENCES Employee(e_id),
        FOREIGN KEY (h_addr) REFERENCES Hotel(h_addr)
        )", []
    )?;

    db_conn.execute ( 
        "CREATE TABLE IF NOT EXISTS Reservation (
        r_number TEXT NOT NULL,
        h_addr TEXT NOT NULL,
        res_start_date DATE NOT NULL,
        res_end_date DATE NOT NULL,
        is_renting BOOL NOT NULL,
        c_id TEXT NOT NULL,    
        payment_info TEXT NOT NULL,
        e_id INT,
        CHECK (is_renting = TRUE AND e_id IS NOT NULL),
        PRIMARY KEY (c_id, res_start_date),
        FOREIGN KEY (h_addr, r_number) REFERENCES Room(h_addr, r_number),
        FOREIGN KEY (c_id) REFERENCES Customer(c_id),
        FOREIGN KEY (e_id) REFERENCES Employee(e_id)
        )", []
    )?;

    // trigger 3

    db_conn.execute("
        CREATE TRIGGER IF NOT EXISTS prevent_overlapping_res
        BEFORE INSERT ON Reservation
        FOR EACH ROW
        BEGIN
            SELECT CASE
                WHEN (SELECT COUNT(*) 
                    FROM Reservation 
                    WHERE h_addr = NEW.h_addr
                    AND r_number = NEW.r_number
                    AND NOT (
                    res_end_date < NEW.res_start_date OR 
                    res_start_date > NEW.res_end_date)
                    ) > 0
                THEN RAISE(ABORT, 'Room is already booked')
            END;
        END;
    ", [])?;

    db_conn.execute( 
        "CREATE TABLE IF NOT EXISTS ReservationArchive (
            archive_id INT NOT NULL,
            h_addr TEXT NOT NULL,
            r_number TEXT NOT NULL,
            c_id TEXT NOT NULL,
            res_start_date DATE NOT NULL,
            res_end_date DATE NOT NULL
        )", [])?;

    // indices
    db_conn.execute("
        CREATE INDEX idx_hotel_on_room ON Room(h_addr);
        CREATE INDEX idx_hotel_on_employee ON Employee(h_addr);

        CREATE INDEX idx_address_on_hc ON HotelChain(chain_addr);
        CREATE INDEX idx_hc_on_hotel ON Hotel(chain_name);
        
    ", [])?;
    
    Ok(())
}

pub fn populate_db() -> Result<()> {
    // populate chains
    let hc_list = vec![
        HotelChain {
            name: "Smalliday Inn".to_string(),
            address: "1 Smalliday Road".to_string(),
            email: "smalliday@inn.com".to_string(),
            phone_nums: vec!["999-100-100".to_string()],
        },
        HotelChain {
            name: "Cali Fornia".to_string(),
            address: "123 Sesame Street".to_string(),
            email: "OfficialEmail@california.gov".to_string(),
            phone_nums: vec!["999-200-200".to_string()],
        },
        HotelChain {
            name: "Hotel Mario".to_string(),
            address: "20 Bowser Avenue".to_string(),
            email: "nintendo.com".to_string(),
            phone_nums: vec!["+1-425-376-1270".to_string()],
        },
        HotelChain {
            name: "juddson's house".to_string(),
            address: "921-2 Bank St".to_string(),
            email: "thejuddster@gmail.com".to_string(),
            phone_nums: vec!["don't have that".to_string()],
        },
        HotelChain {
            name: "Four Seasons".to_string(),
            address: "1 seasons road".to_string(),
            email: "fourseasons@gmail.com".to_string(),
            phone_nums: vec!["999-300-300".to_string()],
        }
    ];

    for hc in hc_list.iter() {
        HotelChain::insert(&hc)?;
    }

    // populate hotels
    let mut area_list:Vec<String> = vec![
        "Toronto".to_string(),
        "Hamilton".to_string(),
        "Minneapolis".to_string(),
        "Ottawa".to_string(),
        "Nashville".to_string(),
        "Rio De Janiero".to_string(),
        "Oslo".to_string(),
        "Mushroom Kingdom".to_string(),
        "Sydney".to_string(),
        "Lumiose City".to_string(),
        "Mordor".to_string(),
    ];

    let mut rng = rand::rng();
    area_list.shuffle(&mut rng);

    let mut h_list: Vec<Hotel> = vec![];
    for hc in hc_list.iter() {
        
        let area = match area_list.choose(&mut rng) {
            None => "Tronno".to_string(),
            Some(s) => format!("{} {}", 
                rand::rng().random_range(1..1000).to_string(),
                s.to_string()),
        };
        
        for n in 0..(rand::rng().random_range(8..12)){
            let h = Hotel{
                chain: hc.name.clone(),
                address: format!("{} {}", n, area.clone()),
                phone: rand::rng().random_range(0..100).to_string(),
                email: format!(
                    "{}{}@{}.com", 
                        area.clone(), 
                        rand::rng().random_range(0..100).to_string(),
                        hc.name.clone(),                
                    ),
                room_amount: rand::rng().random_range(4..15),
                rating: Some(rand::rng().random_range(1..5)),
            };

            h_list.push(h);
        }
    }

    let mut amenities_list:Vec<String> = vec![
        "Shower".to_string(),
        "Mini-fridge".to_string(),
        "Wifi".to_string(),
        "Working lights".to_string(),
        "Bowser surprise".to_string(),
        "Kitchentte".to_string(),
        "Pool access".to_string(),
    ];

    let mut r_list: Vec<(Room, &Hotel)> = vec![];

    for h in h_list.iter() {
        Hotel::insert(&h)?;

        for n in 1..(h.room_amount) {
            
            let amt = rand::rng().random_range(0..3);
            let amenities: String = match amt {
                0 => "".to_string(),
                1.. => {
                    &amenities_list.shuffle(&mut rng);
                    amenities_list[0..amt].join(", ").to_string()
                }
            };

            let room = Room{
                room_number: n.clone(),
                amenities: amenities,
                hotel: h.primary_key(),
                capacity: rand::rng().random_range(1..5),
                damages: None,
                other_info: None,
                price: rand::rng().random_range(16..200) * 5,
            };

            r_list.push((room, &h));
            
        }
    }

    for r_tuple in r_list {
        Room::insert(&r_tuple.0, r_tuple.1);
    }
    

    let c_list = vec![
        Customer{
            name: "Juan".to_string(),
            id: "1".to_string(),
            address: "Juan road".to_string(),
            registration_date: "1992-07-03".to_string(),
        },
        Customer{
            name: "Joseph Espino".to_string(),
            id: "2".to_string(),
            address: "Queens Quay".to_string(),
            registration_date: "1992-07-04".to_string(),
        },
        Customer{
            name: "Bowser".to_string(),
            id: "3".to_string(),
            address: "Koopa Kingdom".to_string(),
            registration_date: "1992-07-05".to_string(),
        },
        Customer{
            name: "Phigston".to_string(),
            id: "4".to_string(),
            address: "Tryler road".to_string(),
            registration_date: "1992-07-06".to_string(),
        },
        Customer{
            name: "Cohort Road".to_string(),
            id: "5".to_string(),
            address: "Juan road".to_string(),
            registration_date: "1992-07-07".to_string(),
        },
        Customer{
            name: "Sigurd".to_string(),
            id: "6".to_string(),
            address: "Dragon Avenue".to_string(),
            registration_date: "1992-07-08".to_string(),
        },
    ];

    for c in c_list {
        Customer::insert(&c)?;
    }

    let mut e_list: Vec<Employee> = vec![];

    for n in 1..20 {
        e_list.push(
            Employee { 
                id: n.to_string(), 
                name: format!("Employee {}", &n), 
                address: format!("{} worker road", &n), 
                hotel: match h_list.choose(&mut rng) {
                    None => "uh oh".to_string(),
                    Some(h) => h.primary_key()
                }, 
                roles: None }
        );
    }

    for e in e_list.iter() {
        let h = Hotel::get_from_primary_key(&e.hotel)?;
        Employee::insert(e, &h)?;
    }

    Ok(())
}

fn example_query_1() -> Result<()>{    
    println!("{:?}", Hotel::get_all_hotel_names());
    Ok(())
}