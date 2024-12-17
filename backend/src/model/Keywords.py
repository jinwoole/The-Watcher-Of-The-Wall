from typing import List
from pydantic import BaseModel

class KeywordsResponse(BaseModel):
    keywords: List[str]