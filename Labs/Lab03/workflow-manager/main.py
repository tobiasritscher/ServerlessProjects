import enum
import random
import datetime
import asyncio

import flask
import logging
import aiohttp

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

class TaskNode:
    def __init__(self, of_type, fullname, *name) -> None:
        self.of_type = of_type
        self.fullname = fullname
        self.names = name


class Mapping(enum.Enum):
    START = enum.auto()  
    ATOM = enum.auto()
    TO = enum.auto()
    OR = enum.auto()
    AND = enum.auto()

    @staticmethod
    def map():
        return {
            "|": Mapping.OR,
            "-": Mapping.TO,
            "&": Mapping.AND
        }

    @staticmethod
    def is_two(a) -> bool:
        return a != Mapping.TO

class Description:
    @staticmethod
    def _to_block(block):
        oor = "|" in block 
        oand = "&" in block
        if not oor and not oand:
            return TaskNode(Mapping.ATOM, block, block)
        if oor:
            return TaskNode(Mapping.OR, block, *block.split("|"))
        return TaskNode(Mapping.AND, block, *block.split("&"))

    @staticmethod
    def get(desc: str):
        blocks = [i.strip() for i in desc.split("-")]

        # check for invalid states
        if len(blocks) == 0 or blocks[0] == "":
            return None
        
        # check that no task starts or ends with an operator
        for block in blocks:
            for pos in [0, -1]:
                if block[pos] in Mapping.map():
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
        self.urls = url
        self.status = status

    def to_dict(self):
        res = self.__dict__
        res["start"] = res["start"].isoformat()
        res["end"] = res["end"].isoformat()
        return res

async def post(session, name, url, data, is_type = "POST"):
    status = None
    json = None
    log.debug("calling %s while %s", name, is_type)
    start = datetime.datetime.now()
    async with session.post(url, json = data) as response:
        status = response.status
        json = await response.json()
    end = datetime.datetime.now()
    log.debug("status %d while %s to %s", status, is_type, name)
    return Results(name, [url], start, end, status, json)

async def ping(work):
    async with aiohttp.ClientSession() as session:
        load = [post(session, name, url, {}, "PING") for url, name in zip(work.urls, work.names)]
        results = await asyncio.gather(*load, return_exceptions=False)
        return results

async def op_and(session, work, task, prev):
    names = task.names
    log.info("%s", task.names)
    todo = [post(session, name, work.mapping[name], prev.json) for name in names]
    start = datetime.datetime.now()
    raw_results = await asyncio.gather(*todo, return_exceptions=False)
    end = datetime.datetime.now()
    d = {}
    for s in raw_results:
        d.update(s.json)

    url = [work.mapping[name] for name in names]
    return Results(task.fullname, url, start, end, 200, d) 

async def op_or(session, work, task, prev):
    name = task.names[random.randint(0, len(task.names)) - 1]
    return await post(session, name, work.mapping[name], prev.json)

async def op_to(session, work, task, prev):
    name = task.names[0]
    return await post(session, name, work.mapping[name], prev.json)

async def run_task(work):
    results = []

    results.append(Results(None, None, None, None, None, {"start": 1}))

    async with aiohttp.ClientSession() as session:
        # Run functions
        for task in work.tasks:
            # get data from previous call
            prev = results[-1]

            if task.of_type == Mapping.OR:
                res = op_or(session, work, task, prev)
            elif task.of_type == Mapping.AND:
                res = op_and(session, work, task, prev)
            else:
                res = op_to(session, work, task, prev)
              
            results.append(await res)

    # slice results to remove the unneeded "first" prev 
    return results[1:]

async def run_workflow(work):
    log.info("starting workflow")
    results = {}

    if work.ping:
        log.info("pinging")
        results["pinged"] = [f.to_dict() for f in await ping(work)]

    results["tasks"] = [f.to_dict() for f in await run_task(work)]

    return results

def start_workflow(work):
    return asyncio.run(run_workflow(work))

def main(request: flask.Request):
    log.debug("running request type %s", request.method)
    if request.method == "POST":
        log.info("Got POST request")
        if out := setup(request):
            return start_workflow(out)
        log.info("invalid json")

    flask.abort(405)
