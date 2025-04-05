Kai Rasco
300 304 789

## For every hotel chain we need to know the address of its central offices, the number of its hotels, contact email addresses and phone numbers. The hotels of hotel chains are categorized (e.g. 1-star up to 5-star). 

### For hotels chains:
- address of central offices
- number of hotels
- contact email addresses
- phone numbers


## For each hotel we need to know the number of rooms, the address of the hotel and contact email and phone numbers for this hotel. 

### For hotels:
- number of rooms
- address of hotel
- contact email
- phone numbers
- rating (1-star up to 5-star)

## For the rooms in a hotel, we need to know their price, all amenities (e.g. TV, air condition, fridge etc), the capacity of the room (e.g. single, double etc), if they have sea view or mountain view, if they can be extended (e.g. adding one more bed) and if there are any problems/damages in the room. 

### For rooms:
- price
- amenities
- capacity of the room (single, double)
- sea / mountain view
- if it can be extended (adding one more bed)
- problems / damages

## For customers we need to store their full name, address and a type of ID, e.g. SSN/SIN/driving licence, the date of their registration into our system. 

### For customers:
- full name
- address
- type of ID (SSN / SIN / driving licence)
- date of user registration

## For employees of the hotels, we need to store their full name, address and SSN/SIN. The employees may have various roles/positions in a hotel. 

### For employees:
- full name
- address
- SSN/SIN
- roles/position

## Every hotel needs to have a manager. 

### For managers:
constraint: every hotel needs to have a manager
(assumption: hotel has one manager)

## The customers can search for and book rooms through the online application for specific dates.  When they check-in the hotel, their room booking is transformed to renting and they can also pay for this renting. 

### For reservations:
ui: users can search / create reservation entries online

## The employee that does the check-in for a customer is responsible for transforming the room booking to renting. 
Employees have to be associated with a reservation
- "employee" field 
- booking / renting field

## A customer may present physically at a hotel without a booking and directly ask for a room. In this case the employee at the hotel can do the renting of the room right away without prior booking. 
ui: user can only create reservation entry with booking status
ui: employee can create reservation to renting / create entry with renting status

## We need to store in the database the history of both bookings and rentings (archives), but we do not need to store the history of payments.  Information about an old (archived) room booking/renting must exist in the database, even if information about the room itself or the customer does not exist in the database anymore. 
implementation: 
- seperate table for bookings / rentings
- info is read at time of creation, and copied into this entry
- archive table must have all info because it can't point to anything

## We should be able to delete from our database hotel chains, hotels and rooms. 
ui: ability to delete hotel chains and rooms

## We cannot have in the database information about a room without having in the database the information about the corresponding hotel (i.e. the hotel in which the room belongs too). 
constraint: room has total participation in hotel

## In the same way, we cannot have in the database information about a hotel without having in the database the information about the corresponding hotel chain (i.e. the hotel chain in which the hotel belongs too).
constraint: hotel has total participation in hotel chain