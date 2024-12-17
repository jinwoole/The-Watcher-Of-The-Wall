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

app.include_router(Keywords.router)
app.include_router(DB.router)
app.include_router(Newsitem.router)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", reload=True)