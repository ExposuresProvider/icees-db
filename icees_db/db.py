"""Database tools."""
from contextlib import contextmanager
import os
from pathlib import Path

from sqlalchemy import create_engine
from sqlalchemy.engine import Connection

db = os.environ.get("ICEES_DB", "sqlite")
serv_user = os.environ["ICEES_DBUSER"]
serv_password = os.environ["ICEES_DBPASS"]
serv_host = os.environ["ICEES_HOST"]
serv_port = os.environ["ICEES_PORT"]
serv_database = os.environ["ICEES_DATABASE"]
serv_max_overflow = int(os.environ["ICEES_DB_MAX_OVERFLOW"])
serv_pool_size = int(os.environ["ICEES_DB_POOL_SIZE"])

engine = None

DATAPATH = Path(os.environ["DATA_PATH"])


def get_db_connection():
    """Get database connection."""
    global engine
    if engine is None:
        if db == "sqlite":
            engine = create_engine(
                f"sqlite:///{DATAPATH / 'example.db'}?check_same_thread=False",
            )
        elif db == "postgres":
            engine = create_engine(
                f"postgresql+psycopg2://{serv_user}:{serv_password}@{serv_host}:{serv_port}/{serv_database}",
                pool_size=serv_pool_size,
                max_overflow=serv_max_overflow,
            )
        else:
            raise ValueError(f"Unsupported database '{db}'")

    return engine


@contextmanager
def DBConnection() -> Connection:
    """Database connection."""
    engine = get_db_connection()
    conn: Connection = engine.connect()
    try:
        yield conn
    finally:
        conn.close()
