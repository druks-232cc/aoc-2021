struct BingoCard {
    list: [u32; 25],
    bitmap: [bool; 25],
    win_score: u32,
}

impl BingoCard {
    fn from_slices(s: &[&str]) -> BingoCard {
        let mut l = [0; 25];

        for (i, line) in s.iter().enumerate() {
            let part: [u32; 5] = line.split_ascii_whitespace()
                                     .map(|x| u32::from_str_radix(x, 10).unwrap())
                                     .collect::<Vec<u32>>()
                                     .try_into().unwrap();
            l[i*5..5*(i+1)].copy_from_slice(&part);
        }

        BingoCard { list: l, bitmap: [false; 25], win_score: 0 }
    }

    fn mark_numbers(&mut self, n: u32) {
        let mut iter = self.list.iter();
        while let Some(index) = iter.position(|x| *x == n) {
            self.bitmap[index] = true;
        }
    }

    fn check_win(&mut self) -> bool {
        let mut win;
        for i in 0..5 {
            //check line
            if self.bitmap[i*5..5*(i+1)].eq(&[true; 5]) {
                self.win_score = self.sum_unmarked();
                return true
            }
            //check col
            win = true;
            for j in 0..5 {
                if self.bitmap.get(i+j*5).unwrap().eq(&false) {
                    win = false;
                    break;
                }
            }
            if win {
                self.win_score = self.sum_unmarked();
                return true
            }
        }
        return false
    }

    fn sum_unmarked(&self) -> u32 {
        let iter = self.list.iter().zip(self.bitmap.iter());

        let (list, _bitmap): (Vec<u32>, Vec<bool>) = iter.filter(|(_x, y)| y.eq(&&false)).unzip();

        list.iter().sum()
    }
}

fn run(input: &str) -> Option<()> {
    let mut lines: Vec<&str> = input.lines().collect();

    // get first line with values
    let values: Vec<u32> = lines.get(0)?
                                .split(',')
                                .map(|x| u32::from_str_radix(x,10).unwrap())
                                .collect();
    // skip two lines
    lines.drain(0..2);

    let mut cards: Vec<BingoCard> = vec![];

    //Build bingo grid array
    for chunk in lines.chunks(6) {
        cards.push(BingoCard::from_slices(chunk.get(0..5)?));
    }

    //Build bitmap
    let mut first_res=0;
    let mut last_res=0;
    for v in values {
        for c in cards.iter_mut() {
            if c.win_score.eq(&0) {
                c.mark_numbers(v);
                if c.check_win() {
                    last_res = c.win_score*v;
                    if first_res == 0 {
                        first_res = last_res;
                    }
                }
            }
        }
    }

    println!("Day4 p1 : {}", first_res);
    println!("Day4 p2 : {}", last_res);

    Some(())
}

fn main() {
    run(include_str!("input.txt"));
}