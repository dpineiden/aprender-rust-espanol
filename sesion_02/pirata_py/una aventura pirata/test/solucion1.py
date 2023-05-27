from rich import print
import sys
import numpy as np

def distancia(p0,p1):
    x0 = p0[1]
    x1 = p1[1]
    delta = x1-x0
    return abs(delta), np.sign(delta)


if __name__ == "__main__":
    print(sys.argv)
    # posicion y rapidez de A
    ax = float(sys.argv[1])
    vax = float(sys.argv[2])
    # posicion y rapidez de B
    bx = float(sys.argv[3])
    vbx = float(sys.argv[4])

    p0 = ("Isla", 3, 0)
    p1 = ("A", ax, vax)
    p2 = ("B", bx, vbx)

    print("Puntos:")
    print(f"B: {p1}")
    print(f"B: {p2}")
    print("Solución simple al problema del tesoro")

    d1, signA = distancia(p0,p1)
    d2, signB = distancia(p0,p2)
    
    # calcular tiempo
    ta = np.Inf
    if vax>0:
        ta = abs(d1/(signA*vax))

    tb = np.Inf
    if vbx>0:
        tb = abs(d2/(signB*vbx))
    print(f"Tiempos->A {ta}, B {tb}")
    if ta<tb:
        print("Gana A")
    elif ta>tb:
        print("Gana B")
    else:
        print("Llegan al mismo tiempo")

    print("Segunda parte: rapidez de A para llegar al mismo tiempo que  B")
    # rapidez de A
    step = .1
    flag = False
    signo = np.sign(vbx-vax)
    while  not (tb - .1 <= ta <= tb + .1):
        signo_antes = signo
        signo = np.sign(vbx-vax)
        if signo != signo_antes:
            break
        vax =  vax +  signo* step
        ta = np.Inf
        if vax>0:
            ta = abs(d1/(signA*vax))

        tb = np.Inf
        if vbx>0:
            tb = abs(d2/(signB*vbx))

    print(f"Tiempos->A {ta}[s], B {tb}[s], deberían llegar casi al mismo tiempo")
    print(f"rapidez A {vax}, B {vbx}")

    
