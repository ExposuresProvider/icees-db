# ICEES DB

Tools for building the ICEES backend database(s).

## Instructions

Generate a `.env` file of the following sort:

```text
ICEES_PORT=5432
ICEES_HOST=db
ICEES_DB=sqlite
ICEES_DBUSER=icees_dbuser
ICEES_DBPASS=icees_dbpass
POSTGRES_PASSWORD=icees_postgres_password
ICEES_DATABASE=icees_database
ICEES_DB_POOL_SIZE=10
ICEES_DB_MAX_OVERFLOW=0
DATA_PATH=./db/data
DB_PATH=./data
CONFIG_PATH=./config
```

Add a `features.yml` file to your `CONFIG_PATH` of the following sort:

```yaml
table_0:
  feature_0:
    type: integer
    minimum: 0
    maximum: 1
  feature_1:
    type: string
    enum:
      - 'alligator'
      - 'baboon'
      - 'cockatoo'
```

Run:

```bash
./initdb.sh
```
