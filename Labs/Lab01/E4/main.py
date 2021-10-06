import numpy as np


def hello_world(request):
    a = np.array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]])
    res = a.T @ a 
    return {"result": res.tolist() }

