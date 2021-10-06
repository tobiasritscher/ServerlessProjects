
def get_cpuinfo():
    lines = []
    curr = []
    with open("/proc/cpuinfo","r") as cpuinfo:
        for line in cpuinfo:
            line = line.strip()
            if line == "":
                lines.append(curr)
                curr = []
                continue
            curr.append(line)

    return lines

def hello_world(request):
    cpuinfo = get_cpuinfo()
    
    res = {}
    for processor in cpuinfo:
        curr = {}
        for info in processor:
            parts = info.split(":")
            if len(parts) == 1:
                parts.append("")

            key, value = parts
            key = key.strip()
            value = value.strip()
            if key in ("flags", "bugs"):
                value = value.split(" ")
            curr[key] = value
        res[curr["processor"]] = curr
    return res

