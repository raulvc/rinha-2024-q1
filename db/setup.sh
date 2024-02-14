#!/bin/bash

# loads schema on docker build

sqld &

python3 << END
from libsql_client import dbapi2

with open('/var/lib/sqld/init.sql', 'r') as file:
  schema = file.read()

conn = dbapi2.connect("ws://localhost:8080", uri=True)
conn.executescript(schema)
conn.commit()
conn.close()
END