use std::collections::HashSet;
use std::process;

fn main() 
{let input = "+12
-13
+17
+17
-10
+6
+13
+13
-9
+13
-15
+16
+12
-19
-15
-11
+16
-4
+9
+9
+4
+8
+19
-15
-10
+4
-2
+17
-4
+7
+15
+8
+10
-13
+11
+12
-4
-7
+3
-6
-8
+19
+10
+14
+7
-3
-19
+16
+17
+15
-18
+19
+11
-7
+10
-17
+11
-20
+4
+9
-11
-12
+9
-13
+2
-12
-1
+17
+15
+10
+14
-7
-5
-13
-19
-6
+13
-14
-18
+4
+6
-4
+5
+18
+23
-6
+13
+17
+4
+16
-18
-6
-9
+11
+7
+12
+10
-16
-10
+12
+10
-2
+16
+10
-7
-11
+3
+9
-16
-9
+10
+2
+15
-4
-6
+12
+5
+5
+19
-18
-4
+14
-2
-4
+18
-15
-10
+5
+17
+15
-9
-5
+9
+1
+9
+14
+15
+11
-16
+15
+8
+15
+4
+10
+1
+18
+12
-5
-1
+16
-11
-19
-5
+15
-17
+19
-5
+14
+6
+12
+10
+16
+6
+8
-12
-1
+4
-17
-13
+2
+12
-3
-16
+17
-6
+14
+3
+5
+4
+10
+8
-17
+7
+19
+13
+18
+17
-9
+14
-4
-3
-11
-13
+6
-2
+16
-15
-15
-13
-7
-7
-7
+16
+17
-15
-10
-2
-7
+2
+16
+6
+20
-6
-13
-5
+1
+12
+6
+20
-22
-10
+5
-16
-17
+13
+18
-4
+5
+11
+14
-9
+1
-19
-1
+32
-14
-3
+9
+3
+18
+9
+15
-11
+17
-15
+2
-13
-14
+18
-3
-3
-9
-5
-48
-15
+13
-1
-5
-23
-10
+16
-17
-16
-2
+16
-13
-15
-4
-10
-1
+2
-18
+11
-12
+14
+19
-9
+19
-17
+11
+16
+6
+6
+6
-20
+19
+2
+5
-21
+20
+8
-35
-8
+17
-19
+1
-29
-8
+13
-18
-5
+6
+8
+13
-18
+11
+18
-10
-30
-18
-2
-1
+11
+14
+1
-18
-20
+7
-1
-11
-13
-5
+9
-18
+15
-7
-5
+3
-8
-4
+16
-9
+10
+8
+14
+2
-10
-18
+16
+20
+16
-9
+12
-5
-4
-2
+9
+20
-14
+5
-17
-17
-1
-2
-5
-9
+1
+9
-17
+11
-2
-5
+11
-23
-16
+15
-5
-13
-7
-4
-1
-10
-6
-3
-2
-12
-15
+16
-11
+7
-22
+5
+8
-10
-13
-19
+23
+3
-12
-10
-19
+26
-1
-1
+19
+2
+26
-18
-23
+20
+27
+20
+7
+8
+11
-34
+19
+11
-14
+13
-39
+5
+31
-21
+12
-6
+40
-16
-51
+23
-31
-33
-5
-20
+5
-10
-10
-18
-8
-17
-12
-14
-16
+10
+12
+19
-15
-11
+5
-7
-14
+4
+18
-4
-9
-15
+21
-20
-7
-17
+26
-6
-5
+17
+21
+11
+21
-9
-15
+17
+16
-13
-23
+1
+9
-20
-12
-11
+3
+13
+21
-20
-3
+31
+21
-11
-1
+8
-12
+15
+28
+3
-18
+6
-5
+13
+48
-152
-14
+20
-12
-4
-2
-136
+26
-87
-21
+23
-15
+8
+2
-12
+37
+43
-342
+328
+120
-72629
-10
-10
+4
-11
-16
-18
+5
-6
+9
+6
+12
-5
+2
-14
+4
+16
-18
-19
-2
+6
-17
+6
+13
-18
-5
+20
-9
-5
+7
-15
+9
+19
+14
+16
-1
-18
-19
-15
+6
-12
-18
-9
+11
+10
+5
-16
-18
+13
+14
-16
+1
+8
-16
-11
-17
-18
+1
-8
+13
-9
+14
+3
-1
+15
+11
+17
-9
+16
+9
+16
-23
+14
-15
-12
+5
+18
+17
+16
+4
+9
+19
-8
+10
+17
+3
-1
-1
-7
-9
-6
-16
+5
+2
+4
-3
+17
+23
+2
-29
+2
+4
+8
-2
+32
-22
+16
-66
-16
-26
+9
+20
+5
+4
-17
-20
-6
-17
-14
-14
+3
-7
+17
+4
+8
-10
-8
-15
-3
+9
+1
-18
+16
-3
-12
+11
-6
-1
-18
+4
-11
-21
+8
-14
+4
-3
-7
+11
+4
+10
-4
+13
+9
-7
-18
-5
+14
+10
-9
-9
+13
+16
-3
-6
-17
+16
-15
+12
+12
+10
+19
+7
+9
+6
-16
-9
+8
-3
-4
-18
-15
+11
+41
+14
+38
+13
-28
-15
-98
-12
-9
-2
-18
-1
-5
-3
-3
-5
-18
-14
+17
-4
-14
+6
+7
+4
+15
-8
-3
-9
+15
+7
+8
+3
+3
+10
+11
-6
-2
+15
-3
-18
+10
+1
-14
+10
-3
+1
+11
+6
-34
-15
-17
-1
-6
-11
-17
+5
-18
+12
-5
-8
+11
+14
+5
-10
+7
-12
+8
+10
+3
-15
+13
+3
+9
+14
-20
-8
-16
-20
+8
+2
-1
-7
-7
+2
-15
-7
-2
-8
-14
-10
+4
-19
+14
-17
-17
+7
-19
-10
-10
-12
+7
+14
-10
+12
-10
+1
+6
-15
-12
-14
+5
+19
+14
+1
+15
-6
+3
-5
+13
+7
-8
+18
-16
+4
-1
-14
+18
+19
-17
+12
-10
-3
+9
+10
+18
+11
+5
-9
+16
+1
-2
+10
+1
+12
-19
-12
+6
-7
-3
-18
+12
-11
+10
-1
-1
+8
-11
-2
+12
-23
+2
-17
+19
-16
+5
+6
-19
+9
+14
-20
+23
-5
-3
-4
-22
-1
-16
-20
-14
+7
+2
-11
-2
-18
+2
-4
+17
+8
-20
+2
+19
-14
-15
+17
-6
-2
+5
+18
+1
+6
+7
+17
-7
-18
+15
-6
-2
-10
-11
+28
-21
-13
-32
+4
+6
+3
+2
+21
-16
-22
-1
+12
+17
+14
+46
-6
+2
+3
+19
+6
+15
+7
+20
+21
-7
-1
-15
-3
+9
-3
-12
+19
+16
+12
-5
-13
+15
+15
-4
-18
-1
-5
+2
+18
-2
-11
+16
+7
-15
+16
-22
-6
-6
+13
-19
+52
-5
+7
+5
-11
+8
+21
+20
+18
-11
-1
-15
+3
+16
+17
-19
-11
+28
+29
-8
-18
-6
-20
-6
+21
+26
-5
-14
-13
-10
-25
-2
+16
-3
-19
-7
+17
+71
+26
-40
+129
-441
-12
+16
-166
-10
+2
+13
+22
+29
+14
+22
+57
-83
-82
+18
+73808";

    
    let s  =input.split_whitespace().try_fold(0i32, |acc,y|  y.parse::<i32>().and_then(|i| Ok(acc +i)));
    match s
    {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => println!("Error: {}", e),
    }
    
    let v: Vec<i32> = input.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    
    
    let mut m = HashSet::new();
    let mut acc =0;
    for num in v.iter().cycle()
    {
        acc += num;
        if !m.insert(acc)
        {
            println!("Part 2: {:?}", acc);
            process::exit(1);
        }
    }


}
