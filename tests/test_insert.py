"""Test insert."""
import io
import sqlite3

from icees_db.dbutils import _insert


def test_insert():
    """Test insert."""
    con = sqlite3.connect(":memory:")
    query = "CREATE TABLE test (a int, b int, c int);"
    con.execute(query)
    query = "INSERT INTO test (a, b, c) VALUES (3, 4, 5);"
    con.execute(query)
    con.commit()
    data = """
        a,b,c
        1,,2
    """
    data = "\n".join(line.strip() for line in data.split("\n") if line.strip())

    with io.StringIO(data) as stream:
        _insert("test", con, stream)
    
    result = con.execute("SELECT * from test")
    assert list(result)
    con.close()
