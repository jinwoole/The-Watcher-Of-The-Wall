from pydantic import BaseModel
from datetime import datetime

class NewsItem(BaseModel):
    id: int
    title: str
    description: str
    publishedAt: datetime
    link: str

class NewsResponse(BaseModel):
    items: list[NewsItem]