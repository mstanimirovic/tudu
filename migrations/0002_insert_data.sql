INSERT INTO users (name, email, password) VALUES ('Mladen', 'ms@email', 'huha');

INSERT INTO categories (user_id, name, color) VALUES (1, 'School', '#00ff00');

INSERT INTO todos (user_id, category_id, title, priority) VALUES (1, 1, 'Study', 1);
INSERT INTO todos (user_id, category_id, title, priority) VALUES (1, 1, 'Learn', 2);
INSERT INTO todos (user_id, category_id, title, priority) VALUES (1, 1, 'Play', 3);
INSERT INTO todos (user_id, category_id, title, priority) VALUES (1, 1, 'Eat', 4);
