from fastapi import APIRouter, HTTPException
from data.SQLite import Database
from pydantic import BaseModel

router = APIRouter(prefix="/db")

db = Database(db_path="database.sqlite")
db.connect()

# 데이터 모델 정의
class Item(BaseModel):
    value: str

# 테이블 생성 (없을 경우 생성)
db.create_table("CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY AUTOINCREMENT, value TEXT)")

# CREATE
@router.post("/items")
async def create_item(item: Item):
    try:
        # Check for duplicate value
        check_query = "SELECT COUNT(*) FROM items WHERE value = ?"
        db.execute(check_query, (item.value,))
        result = db.fetchone(check_query, (item.value,))
        if result[0] > 0:
            raise HTTPException(status_code=400, detail="중복된 아이템이 이미 존재합니다.")
        
        # Insert if no duplicate found
        query = "INSERT INTO items (value) VALUES (?)"
        params = (item.value,)
        db.execute(query, params)
        return {"message": "아이템이 추가되었습니다."}
    except HTTPException as e:
        raise e
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# DELETE
@router.delete("/items/{value}")
async def delete_item(value: str):
    try:
        query = "DELETE FROM items WHERE value = ?"
        params = (value,)
        db.execute(query, params)
        return {"message": "아이템이 삭제되었습니다."}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


