use std::io;
use std::cmp;
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}



fn permutacio(list: &mut Vec<String>, k: i32, m: i32, perm: &mut Vec<Vec<String>>)
{
    if k == m
    {
        let mut tomb: Vec<String> = Vec::new();
        for elemek in list
        {
            tomb.push(elemek.to_string());
        }
        perm.push(tomb);
    }
    else
    {
        let mut i: i32 = k;
        while i < m
        {
            list.swap(k as usize,i as usize);
            permutacio(list, k+1, m, perm);
            list.swap(k as usize,i as usize);
            i+=1;
        }
    }
}


fn kivalaszt(perm: &Vec<Vec<String>>) -> i32
{
    let mut min = 99999;
    for a in perm
    {
        let mut elso = String::new();
        let mut elso1 = elso.push_str(&a[0 as usize]);
        let mut j = 1;
        while j < a.iter().count()
        {
            let mut i = 0;
            while i < elso.len()
            {
                let substr = a[j].get(0..elso.len()-i);
                let mut substr = match substr
                {
                    Some(s) => s, 
                    None => &a[j],
                };
                

                if elso.contains(&a[j])
                {
                    break;
                }
                else if &elso[i..] == substr
                {
                    elso.push_str(&a[j][elso.len()- i..]);
                    break;
                }
                i+=1;
            }
            if i == elso.len()
            {
                elso.push_str(&a[j]);
            }
            j+=1;
        }
        min = cmp::min(elso.len(),min);
    }
    return min as i32;
}


fn main() {
    let mut perm: Vec<Vec<String>> = Vec::new();
    let mut lista: Vec<String> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let subseq = input_line.trim().to_string();
        lista.push(subseq);
    }

    let mut hossz: i32 = lista.iter().count() as i32;
    permutacio(&mut lista, 0, hossz, &mut perm);

    println!("{}",kivalaszt(&perm));

}

