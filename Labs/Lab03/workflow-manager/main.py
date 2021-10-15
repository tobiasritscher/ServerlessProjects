import enum
import random
import datetime

import nest_asyncio
import flask
import logging
import aiohttp
import asyncio

logging.basicConfig(format='%(asctime)s %(message)s', datefmt='%m/%d/%Y %I:%M:%S %p')
log = logging.getLogger(__name__)
log.setLevel(logging.DEBUG)

def check_input(work: dict) -> bool:
    contains = ["description", "urls", "names", "ping"]

    for c in contains:
        if c not in work:
            log.debug("missing field %s", c)
            return False

    if len(work["urls"]) != len(work["names"]):
        log.debug("lenght of workloads is different urls: %d - names: %d", len(work["urls"]), len(work["names"]))
        return False
    
    return True

class WorkNode:
    def __init__(self, of_type, *name) -> None:
        self.of_type = of_type
        self.names= list(name)

class Mapping(enum.Enum):
    START = enum.auto()  
    ATOM = enum.auto()
    TO = enum.auto()
    OR = enum.auto()

    @staticmethod
    def map():
        return {
            "|": Mapping.OR,
            "-": Mapping.TO
        }

    @staticmethod
    def is_two(a) -> bool:
        return a != Mapping.TO

class Description:
    @staticmethod
    def _to_block(block):
        # TODO: handle AND
        if "|" not in block:
            return WorkNode(Mapping.ATOM, block)
        return WorkNode(Mapping.OR, *block.split("|"))

    @staticmethod
    def get(desc: str):
        blocks = [i.strip() for i in desc.split("-")]

        # check for invalid states
        if len(blocks) == 0 or blocks[0] == "" or blocks[0] in Mapping.map():
            return None

        return [Description._to_block(block) for block in blocks]

class Work:
    def __init__(self, work, tasks):
        self.description = work["description"]
        self.ping = work["ping"]
        self.urls = work["urls"]
        self.names = work["names"]
        self.tasks = tasks 

        self.mapping = {}
        for name, url in zip(self.names, self.urls):
            self.mapping[name] = url
    
def setup(request: flask.Request):
    # get json
    work = request.get_json()
    if work is None:
        log.info("work is None")
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
        log.info("work is invalid")
        return None 

    desc = Description.get(work["description"])
    if not desc:
        return None
    
    return Work(work, desc)

class Results:
    def __init__(self, name, url, start, end, status, json):
        self.start = start
        self.end = end
        self.json = json
        self.name = name
        self.url = url
        self.status = status

    def to_dict(self):
        return self.__dict__

async def ipost(session, name, url, data):
    status = None
    json = None
    log.debug("calling %s", name)
    start = datetime.datetime.now()
    async with session.post(url, json = data) as response:
        status = response.status
        json = await response.json()
    end = datetime.datetime.now()
    log.debug("status %d while pinging %s", status, name)
    return Results(name, url, start.isoformat(), end.isoformat(), status, json)

async def iping(urls, names):
    async with aiohttp.ClientSession() as session:
        work = [ipost(session, name, url, {}) for url, name in zip(urls, names)]
        results = await asyncio.gather(*work, return_exceptions=False)
        return results

def ping(work):
    loop = asyncio.get_running_loop()
    return loop.run_until_complete(iping(work.urls, work.names))

async def irun_task(work):
    results = []

    # reverse
    tasks = work.tasks[::-1]

    # build tree
    # (f0) -> (f1) -> (f2 | f3) -> (f4)

    async with aiohttp.ClientSession() as session:
        # run first function
        start = {"start": 1}
        task = tasks.pop()
        name = task.names[0]
        
        res = await ipost(session, name, work.mapping[name], start) 
        results.append(res)

        # Run functions
        while len(tasks) > 0:
            task = tasks.pop()
            if Mapping.is_two(task.of_type):
                # TODO: handle AND
                name = task.names[random.randint(0, len(task.names))]
              
            # get data from previous call
            prev = results[-1]

            res = await ipost(session, name, work.mapping[name], prev.to_dict()) 
            results.append(res)

    return results

def run_task(work):
    loop = asyncio.get_running_loop()
    return loop.run_until_complete(irun_task(work))

async def irun_workflow(work):
    nest_asyncio.apply()
    log.info("starting workflow")
    results = {}

    if work.ping:
        log.info("pinging")
        results["pinged"] = [f.to_dict() for f in ping(work)]
    else:
        results["tasks"] = [f.to_dict() for f in run_task(work)]

    return results


def start_workflow(work):
    return asyncio.run(irun_workflow(work))

def main(request: flask.Request):
    log.debug("running request type %s", request.method)
    if request.method == "POST":
        log.info("Got POST request")
        if out := setup(request):
            return start_workflow(out)
        log.info("invalid json")

    flask.abort(405)
