#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

void permutacio(vector<string> list, int k, int m, vector<vector<string>> &perm)
{
    if (k == m)
    {
        vector<string> tomb;
        for (string a : list)
        {
            tomb.push_back(a);
        }
        perm.push_back(tomb);
    }
    else
    {
        int i = k;
        while (i < m)
        {
            string sv = list[k];
            list[k] = list[i];
            list[i] = sv;
            permutacio(list, k + 1, m, perm);
            string sv1 = list[k];
            list[k] = list[i];
            list[i] = sv1;
            i++;
        }
    }
}


int kivalaszt(vector<vector<string>> &perm)
{
    int minim = 99999;
    for (vector<string> a :perm)
    {
        string elso = a[0];
        int j = 1;
        while (j < a.size())
        {
            int i = 0;
            while (i < elso.size())
            {
                string substr;
                try 
                {
                    substr = a[j].substr(0, elso.size() - i);
                }
                catch (exception)
                {
                    substr = a[j];
                }
                if (elso.find(a[j]) != -1)
                {
                    break;
                }
                else if (elso.substr(i) == substr)
                {
                    elso += a[j].substr(elso.size() - i);
                    break;
                }
                i++;
                

            }
            if (i == elso.size())
            {
                elso += a[j];
            }
            j++;
        }
        minim = min(static_cast<int>(elso.size()),minim);
    }
    return minim;
}

int main()
{
    vector <vector<string>> perm;
    vector <string> lista;
    int N;
    cin >> N; cin.ignore();
    for (int i = 0; i < N; i++) {
        string subseq;
        cin >> subseq; cin.ignore();
        lista.push_back(subseq);
    }
    
    permutacio(lista, 0, lista.size(), perm);

    cout << kivalaszt(perm) << endl;


   
}