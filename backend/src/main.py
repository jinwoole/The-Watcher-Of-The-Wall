from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from web import DB, Keywords, Newsitem  # Removed DB

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(Keywords.router, include_in_schema=True)
app.include_router(DB.router, include_in_schema=True)
app.include_router(Newsitem.router, include_in_schema=True)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True, proxy_headers=True, forwarded_allow_ips="*")