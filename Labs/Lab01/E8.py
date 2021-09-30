import json

def get_cpuinfo():
    lines = []
    count = 0
    limit = 20
    with open("/proc/cpuinfo","r") as cpuinfo:
        for line in cpuinfo:
            count+=1
            lines.append(line)
            if count >= limit:
                break              
             
    return lines

def hello_world(request):
    try:
        cpuinfo = get_cpuinfo()
        res = {}
        for info in cpuinfo:
            key,value = info.split(":")
            key = key.strip()
            value = value.strip()
            res[key] = value
        #return ",".join(cpuinfo)
        return json.dumps(res)
    except Exception as e:
        print(e)
        return "fail"
