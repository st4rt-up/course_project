student info

Kai Rasco
300 304 789

instructions to run (in terminal):
git clone https://github.com/st4rt-up/course_project.git
cd course_project
cargo run


I could not get the video to work, but:

1. I chose Rust / SQLite because I initially planned to use Rust/Flutter bridge, but I bit off more than I could chew (the Rust part should work just fine though)

2. Most INT values were changed to text (phone numbers, ids), and phone numbers were made multivalued

3. Almost every value cannot be nullable because they are a pain to work with in Rust, the SQL can be found in the init_tables() function

4. The data is randomly generated if REFRESH_DB is set to true. 5 hotels with the same data each time, but the amount of hotels and their locations are randomized. The amount of rooms and room data (amenities only right now) is also randomized

5. ah.... there is one example query and it doesn't work (NOT implemented)

6. One person cannot book 2 reservations on one day to prevent fraud, there is a trigger in the code around line 808

7. Indices are in the init_tables function and set for keys that are used a lot:
- Room / Employee -> hotel_address (foreign key to a few tables)
- Hotel -> chain_name (foreign key that is called a lot)
- HotelChain -> address is here too because ALL HotelChain info used to be stored in the structs instead of just the primary key, but I kept it becauwe1


8. Database views are NOT implemented


9. The UI is NOT implemented, I could not finish it in time