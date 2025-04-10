DROP INDEX IF EXISTS idx_returned_checkouts_book_id;
DROP INDEX IF EXISTS idx_returned_checkouts_user_id;
DROP TABLE IF EXISTS returned_checkouts;
DROP TABLE IF EXISTS checkouts;
DROP INDEX IF EXISTS idx_checkouts_book_id;
DROP INDEX IF EXISTS idx_checkouts_user_id;
DROP TRIGGER IF EXISTS books_updated_at_trigger ON books;
DROP TABLE IF EXISTS books;

DROP TRIGGER IF EXISTS users_updated_at_trigger ON users;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;


DROP FUNCTION IF EXISTS set_updated_at;