from model.NewsItem import NewsItem, NewsResponse
from datetime import datetime
import urllib.request
import urllib.parse
import json
import os
from dotenv import load_dotenv

def get_news(keyword: str) -> NewsResponse:
    load_dotenv()
    client_id = os.getenv("NAVER_CLIENT_ID")
    client_secret = os.getenv("NAVER_CLIENT_SECRET")
    
    if not client_id or not client_secret:
        print("Error: NAVER_CLIENT_ID or NAVER_CLIENT_SECRET is not set.")
        return NewsResponse(items=[])
    
    encText = urllib.parse.quote(keyword)
    url = f"https://openapi.naver.com/v1/search/news.json?query={encText}&display=100&start=1&sort=date"  # JSON 결과

    request = urllib.request.Request(url)
    request.add_header("X-Naver-Client-Id", client_id)
    request.add_header("X-Naver-Client-Secret", client_secret)

    try:
        response = urllib.request.urlopen(request)
        rescode = response.getcode()
        if rescode == 200:
            response_body = response.read()
            data = json.loads(response_body.decode('utf-8'))
            items = [
                NewsItem(
                    id=index + 1,
                    title=item["title"],
                    description=item["description"],
                    publishedAt=datetime.strptime(item["pubDate"], "%a, %d %b %Y %H:%M:%S %z"),
                    link=item["link"]
                )
                for index, item in enumerate(data.get("items", []))
            ]
            return NewsResponse(items=items)
        else:
            print("Error Code:", rescode)
            return NewsResponse(items=[])
    except Exception as e:
        print("Exception occurred:", e)
        return NewsResponse(items=[])

