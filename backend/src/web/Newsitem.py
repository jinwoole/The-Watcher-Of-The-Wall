from fastapi import APIRouter, Query
from model.NewsItem import NewsItem, NewsResponse
from service.Newsitem import get_news

router = APIRouter(prefix="/news")

@router.get("", response_model=NewsResponse)
def read_news(keyword: str = Query(..., description="Keyword to search for")):
    if not keyword:
        print("No keyword provided.")
        return NewsResponse(items=[])
    print("Endpoint /news/ called with keyword:", keyword)  # Debugging statement
    return get_news(keyword)

