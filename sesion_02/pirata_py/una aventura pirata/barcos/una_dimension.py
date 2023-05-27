from rich import print
import sys
import numpy as np
from pathlib import Path
import csv

def distancia(p0,p1):
    x0 = p0["x"]
    x1 = p1["x"]
    delta = x1-x0
    return p1["id"], abs(delta), np.sign(delta)

# # calcular tiempo
def tiempo(d, vx):
    t = np.Inf
    if vx>0:
        t = abs(d/vx)
    return vx

def obtener_barcos(path):
    lista = []

    if path.exists():
        with path.open() as f:
            reader = csv.DictReader(f, delimiter=";")
            for elem in reader:
                elem["x"] = float(elem["x"])
                elem["v"] = float(elem["v"])
                lista.append(elem)
    else:
        print("No exista ruta a barcos.csv")
    return lista


def barco_ganador(lista, solucion_distancias):
    tiempos = [
        dict(zip(("id","d","sign","t"), (*sol, tiempo(sol[1], lista[i]["v"])))) for i,sol in
        enumerate(solucion_distancias)
    ]
    menor_tiempo = min(tiempos, key=lambda e: e["t"])
    return menor_tiempo
