import flask
import aiohttp
import asyncio
import enum

def check_input(work: dict) -> bool:
    contains = ["description", "urls", "names", "ping"]

    for c in contains:
        if c not in work:
            return False
    if len(work["urls"]) != len(work["names"]):
        return False
    
    return True

class Mapping(enum.Enum):
    TO = enum.auto()
    OR = enum.auto()
    
    @classmethod
    def map(cls, s):
        m = {
            "|": Mapping.OR,
            "_": Mapping.TO
        }
        return m[s]

    @classmethod
    def is_two(cls,a) -> bool:
        return a == Mapping.TO

def map_description(desc: str):
    out = []
    starts = 0 

    for i, c in enumerate(desc):
        if c in ("|", "-"):
            out.append(desc[starts:i])
            a = Mapping.map(c)
            if a is None:
                return None
            out.append(a)

    return out

class Work:
    def __init__(self, work, tasks):
        self.description = work["description"]
        self.ping = work["ping"]
        self.urls = work["urls"]
        self.namse = work["names"]
        self.tasks = tasks 


def setup(request: flask.Request):
    # get json
    work = request.get_json()
    if work is None:
        return None 

    # json 
    # {
    #   "description": "f0-f1-f2|f3-f4",
    #   "ping": False,
    #   "urls":  [],
    #   "names": []
    # }

    # check input 
    if not check_input(work):
        return None 

    desc = map_description(work["description"])
    if not desc:
        return None
    
    return Work(work, desc)

async def iget(session, url, data):
    async with session.post(url, json = data) as response:
        return await response.get_json()

async def iping(urls):
    async with aiohttp.ClientSession() as session:
        results = await asyncio.gather(*[iget(session, url, {}) for url in urls], return_exceptions=True)
        return results

def ping(work):
    loop = asyncio.get_event_loop()
    loop.run_until_complete(iping(work.urls))

async def irun_task(work):
    async with aiohttp.ClientSession() as session:
        # Run functions
        starts = {"start": 1}
        # TODO: run functions

def run_task(work):
    loop = asyncio.get_event_loop()
    loop.run_until_complete(iping(work.urls))

def start_workflow(work):
    if work.ping:
        ping(work) 
    
    return run_task(work)

def main(request: flask.Request):
    if request.method == "POST":
        if out := setup(request):
            return start_workflow(out)

    flask.abort(405)
