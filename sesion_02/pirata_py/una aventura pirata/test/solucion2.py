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


if __name__ == "__main__":
    p0 = {"id":"Isla","x":3,"v":0}
    path = Path(__file__).parent / "barcos.csv"

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

    print("Puntos:")
    [print(p) for p in lista]

    print("Solución simple al problema del tesoro")
    
    solucion_distancias = [distancia(p0,p) for p in lista]
    
    [print(sd) for sd in solucion_distancias]

    solucion_menor = min(solucion_distancias, key=lambda e: e[1])

    sol = solucion_menor[1]

    print(f"EL barco ubicado mas cerca es {solucion_menor}")
    for elem in solucion_distancias:
        if sol == elem[1]:
            print(elem)

    # # calcular tiempo
    def tiempo(d, vx):
        t = np.Inf
        if vx>0:
            t = abs(d/vx)
        return vx

    tiempos = [
        dict(zip(("id","d","sign","t"), (*sol, tiempo(sol[1], lista[i]["v"])))) for i,sol in
        enumerate(solucion_distancias)
    ]
    print(tiempos)

    menor_tiempo = min(tiempos, key=lambda e: e["t"])

    print(f"El barco que demora menos es {menor_tiempo}")

