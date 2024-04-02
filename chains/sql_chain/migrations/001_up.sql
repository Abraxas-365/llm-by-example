-- Migrations file

-- Create the 'users' table
CREATE TABLE users (
    id serial PRIMARY KEY, 
    name varchar(255), 
    address text
);

-- Create the 'more_info' table
CREATE TABLE more_info (
    id serial PRIMARY KEY, 
    user_id int references users(id), 
    ig_nickname varchar(255),
    phone_number varchar(255)
);


-- Dummy Data

-- Inserting into 'users' table
INSERT INTO users(name, address) 
VALUES 
    ('John Doe', '123 Main St'),
    ('Jane Doe', '456 Oak St'),
    ('Jim Doe', '789 Pine St');

-- Inserting into 'more_info' table
INSERT INTO more_info(user_id, ig_nickname, phone_number) 
VALUES 
    (1, 'john_ig', '123-456-7890'),
    (2, 'jane_ig', '456-789-0123'),
    (3, 'jim_ig', '789-012-3456');
