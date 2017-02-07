use std;
pub fn prime_filter(iter_size: usize) -> std::vec::Vec<bool>{
    //Sieve of Atkin
    if iter_size < 100 {
        old_prime_filter(iter_size)
    }else{
        let mut prime_filter = vec![false; iter_size];
        prime_filter[2] = true;
        prime_filter[3] = true;
        prime_filter[5] = true;
        let mut y_sq = 1;
        let mut to_next_y_sq = 3;
        while y_sq<iter_size {
            if y_sq%2 == 1 {
                //n_1 = 4x^2 + y^2 === 1 (mod 4)
                let (mut n_1, mut to_next_n_1) = (4 + y_sq, 12);
                while n_1<iter_size{
                    prime_filter[n_1] ^= true;
                    while{ //Do-while
                        n_1 += to_next_n_1;
                        to_next_n_1 *= 8;
                        to_next_n_1 += 4;
                        (n_1%3 == 0) | (n_1%5 == 0)
                    } {}
                };
            };
            if y_sq%3 == 1 {
                //n_2 = 3x^2 + y^2 === 1 (mod 6)
                let (mut n_2, mut to_next_n_2) = ((y_sq%2)*9+3 + y_sq, (y_sq%2)*12+24);
                while n_2<iter_size {
                    prime_filter[n_2] ^= true;
                    while{ //Do-while
                        n_2 += to_next_n_2;
                        to_next_n_2 += 1;
                        to_next_n_2 *= 12;
                        n_2%5==0
                    } {}
                };
            };
            if y_sq%12 == 1 {
                //n_3 = 3x^2 - y^2 === 11 (mod 12)
                let (mut n_3, mut to_next_n_3) = (2*y_sq + 3*to_next_y_sq, 24);
                while n_3<iter_size {
                    prime_filter[n_3] ^= true;
                    while{ //Do-while
                        n_3 += to_next_n_3;
                        to_next_n_3 += 1;
                        to_next_n_3 *= 12;
                        n_3%5==0
                    } {};
                };
            };
            while{ //Do-while
                y_sq += to_next_y_sq;
                to_next_y_sq += 2;
                y_sq%6 == 0
            } {};
        };
        //Eliminate non-squarefree numbers
        let mut n_sq = 49; // 7^2
        let mut next_n_sq = 32; //9^2 - 7^2, skip even numbers.
        while n_sq < iter_size {
            let mut non_sq_free = n_sq;
            while non_sq_free < iter_size {
                prime_filter[non_sq_free] = false;
                while{ //Do-while
                    non_sq_free += n_sq + n_sq;
                    (non_sq_free%3==0) | (non_sq_free%5==0)
                } {};
            };
            while{ //Do-while
                n_sq += next_n_sq;
                next_n_sq += 1;
                next_n_sq *= 4;
                (n_sq%3==0) | (n_sq%5 == 0)
            } {};
        }
        prime_filter
    }
}

pub fn old_prime_filter(iter_size: usize) -> std::vec::Vec<bool>{
    if iter_size < 5 {
         let mut ret = vec![false, false, true, true];
         ret.truncate(iter_size);
         return ret
     }
    let mut prime_filter = vec![true; iter_size];
    prime_filter[0] = false;
    prime_filter[1] = false;
    let mut cur_num = 2;
    'outer: loop{
        for i in (cur_num+1)..iter_size{
            if 0 == i%cur_num { prime_filter[i] = false; }
        }
        cur_num += 1;
        while !prime_filter[cur_num]{
            if cur_num*cur_num > iter_size {
                break 'outer
            }
            cur_num += 1;
        }
    }
    prime_filter
}
