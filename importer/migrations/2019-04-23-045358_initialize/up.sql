-- Your SQL goes here
CREATE TABLE jots (
       jot_id BLOB NOT NULL PRIMARY KEY,
       jot_creation_date TEXT,
       jot_content TEXT,
       device_id BLOB
);

CREATE TABLE tags (
       tag_id BLOB NOT NULL PRIMARY KEY,
       tag_creation_date TEXT,
       tag_text TEXT NOT NULL,
       device_id BLOB
);

CREATE TABLE tag_map (
       tag_id BLOB NOT NULL,
       jot_id BLOB NOT NULL,
       mapping_date TEXT,
       mapping_id BLOB NOT NULL PRIMARY KEY,
       FOREIGN KEY (tag_id) REFERENCES tags (tag_id) ON DELETE CASCADE ON UPDATE NO ACTION,
       FOREIGN KEY (jot_id) REFERENCES jots (jot_id) ON DELETE CASCADE ON UPDATE NO ACTION
)
