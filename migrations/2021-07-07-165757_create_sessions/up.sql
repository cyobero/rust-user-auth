-- Your SQL goes here
CREATE TABLE sessions (
	session_id 	VARCHAR(255) NOT NULL,
	user_id 	INT NOT NULL,
	login_date  DATETIME DEFAULT CURRENT_DATETIME,
	PRIMARY KEY (session_id, user_id),
	FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
