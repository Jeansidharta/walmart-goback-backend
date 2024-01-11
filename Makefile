watch:
	DATABASE_URL=sqlite:./db_dev.sqlite3 PORT=8000 cargo watch -w src -x run

copy_prod:
	usql ~/.local/state/walmart-gobacks/db.sqlite3 -c "PRAGMA wal_checkpoint(TRUNCATE)"
	cp ~/.local/state/walmart-gobacks/db.sqlite3 ./db_dev.sqlite3
