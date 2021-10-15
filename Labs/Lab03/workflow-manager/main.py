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

class Mapping(enum.Enum):
    TO = enum.auto()
    OR = enum.auto()
    
    @staticmethod
    def map(s):
        m = {
            "|": Mapping.OR,
            "_": Mapping.TO,
            "-": Mapping.TO
        }
        return m[s]

    @staticmethod
    def is_two(a) -> bool:
        return a != Mapping.TO

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

    # make sure that the first is of type str
    if not isinstance(out[0], str):
        return None

    return out

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

    desc = map_description(work["description"])
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
    log.debug("pinging %s", name)
    start = datetime.datetime.now()
    async with session.post(url, json = data) as response:
        status = response.status
        json = await response.get_json()
    end = datetime.datetime.now()
    log.debug("status %d while pinging %s", status, name)
    return Results(name, url, start, end, status, json)

async def iping(urls, names):
    async with aiohttp.ClientSession() as session:
        work = [ipost(session, name, url, {}) for url, name in zip(urls, names)]
        results = await asyncio.gather(*work, return_exceptions=True)
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
        curr = tasks.pop()
        
        res = await ipost(session, curr, work.mapping[curr], start) 
        results.append(res)

        # Run functions
        while len(tasks) > 0:
            curr = tasks.pop()
            task = tasks.pop()
            if Mapping.is_two(task):
                # TODO: handle AND
                second = tasks.pop()
                curr = [curr, second][random.randint(0, 1)]
              
            # get data from previous call
            prev = results[-1]

            res = await ipost(session, curr, work.mapping[curr], prev) 
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
