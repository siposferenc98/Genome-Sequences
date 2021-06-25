import sys
import math

def permutacio(lista, k, m, perm):
    if k == m:
        tomb = []
        for i in lista:
            tomb.append(i)
        perm.append(tomb)
    else:
        i = k
        while i < m:
            sv = lista[k]
            lista[k] = lista[i]
            lista[i] = sv
            permutacio(lista, k+1, m, perm)
            sv1 = lista[k]
            lista[k] = lista[i]
            lista[i] = sv1
            i+=1

def kivalaszt(perm):
    minim = 999999
    for a in perm:
        elso = a[0]
        j = 1
        while j < len(a):
            i = 0
            while i < len(elso):
                if a[j] in elso:
                    break
                elif elso[i:] == a[j][0:len(elso)-i]:
                    elso+= a[j][len(elso)-i:]
                    break
                i+=1
            if i == len(elso):
                elso+=a[j]
            j+=1
        minim = min(len(elso), minim)
    return minim





def main():
    perm = []
    lista = []
    n = int(input())
    for i in range(n):
        subseq = input()
        lista.append(subseq)

    permutacio(lista,0,len(lista),perm)
    print(perm,file=sys.stderr, flush=True)
    print(kivalaszt(perm))


main()

