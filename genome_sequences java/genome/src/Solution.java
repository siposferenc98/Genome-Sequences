import java.util.*;
import java.io.*;
import java.math.*;

class Solution {

    public static void permutacio(ArrayList<String> lista, int k, int m, ArrayList<ArrayList<String>> perm)
    {
        if (k == m)
        {
            ArrayList<String> tomb = new ArrayList<String>();
            for (String i : lista)
            {
                tomb.add(i);
            }
            perm.add(tomb);
        }
        else
        {
            int i = k;
            while (i < m)
            {
                String sv = lista.get(k);
                lista.set(k,lista.get(i));
                lista.set(i,sv);
                permutacio(lista, k+1, m, perm);
                String sv1 = lista.get(k);
                lista.set(k,lista.get(i));
                lista.set(i,sv1);
                i++;

            }
        }
    }

    public static int kivalaszt(ArrayList<ArrayList<String>> perm)
    {
        int minim = 99999;
        for (ArrayList<String> a : perm)
        {
            String elso = a.get(0);

            int j = 1;
            while (j < a.size())
            {
                int i = 0;
                while (i < elso.length())
                {
                    String substr = "";
                    try
                    {
                        substr = a.get(j).substring(0,elso.length()-i);
                    }
                    catch(Exception IndexOutOfBoundsException)
                    {
                          
                        substr = a.get(j);
                    }
                    
                    if (elso.contains(a.get(j)))
                    {
                        break;
                    }
                    else if (elso.substring(i).equals(substr))
                    {
                        elso += a.get(j).substring(elso.length()-i);
                        break;
                    }
                    i++;

                }
                if (i == elso.length())
                {
                   
                    elso += a.get(j);
                }
                j++;
            }
            minim = Math.min(elso.length(), minim);
        }
        return minim;
    }



    public static void main(String args[]) {
        ArrayList<ArrayList<String>> perm = new ArrayList<ArrayList<String>>();
        ArrayList<String> lista = new ArrayList<String>(); 
        Scanner in = new Scanner(System.in);
        int N = in.nextInt();
        for (int i = 0; i < N; i++) {
            String subseq = in.next();
            lista.add(subseq);
        }

        permutacio(lista, 0, lista.size(), perm);
        System.out.println(kivalaszt(perm));
        


        // Write an answer using System.out.println()
        // To debug: System.err.println("Debug messages...");

    }
}