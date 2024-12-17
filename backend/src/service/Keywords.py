from model.Keywords import KeywordsResponse
from data.SQLite import Database

db = Database(db_path="../data/keywords.db")
db.connect()

def query_keywords() -> KeywordsResponse:
    return
