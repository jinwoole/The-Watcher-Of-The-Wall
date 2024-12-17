from fastapi import APIRouter
from model.Keywords import KeywordsResponse
from data.SQLite import Database


router = APIRouter(prefix="/keywords")

db = Database(db_path="database.sqlite")
db.connect()

@router.get("/")
def echo() -> KeywordsResponse:
    concatenated_string = db.fetch_all("SELECT * FROM items")
    print(concatenated_string)
    return KeywordsResponse(keywords=concatenated_string)
