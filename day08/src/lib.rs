use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Pixel {
    value: u8,
}
impl Pixel {
    const MAX_PIXEL: u8 = 9;

    fn value(&self) -> u8 {
        self.value
    }

    #[cfg(test)]
    fn values() -> Vec<Pixel> {
        PixelChoice::values()
            .into_iter()
            .map(|pc| Pixel::from(pc))
            .collect()
    }
}
impl TryFrom<u8> for Pixel {
    type Error = String;

    fn try_from(p: u8) -> Result<Self, Self::Error> {
        if p <= Pixel::MAX_PIXEL {
            Ok(Self { value: p })
        } else {
            Err(format!("cannot make pixel for p={}, expect 0 <= p <= 9", p))
        }
    }
}
impl From<PixelChoice> for Pixel {
    fn from(pc: PixelChoice) -> Self {
        let pcu8: u8 = pc.into();
        Pixel::try_from(pcu8).unwrap()
    }
}

#[derive(Debug)]
pub enum PixelChoice {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl From<PixelChoice> for u8 {
    fn from(pc: PixelChoice) -> u8 {
        match pc {
            PixelChoice::Zero => 0u8,
            PixelChoice::One => 1u8,
            PixelChoice::Two => 2u8,
            PixelChoice::Three => 3u8,
            PixelChoice::Four => 4u8,
            PixelChoice::Five => 5u8,
            PixelChoice::Six => 6u8,
            PixelChoice::Seven => 7u8,
            PixelChoice::Eight => 8u8,
            PixelChoice::Nine => 9u8,
        }
    }
}
impl PixelChoice {
    #[cfg(test)]
    fn values() -> Vec<PixelChoice> {
        vec![
            PixelChoice::Zero,
            PixelChoice::One,
            PixelChoice::Two,
            PixelChoice::Three,
            PixelChoice::Four,
            PixelChoice::Five,
            PixelChoice::Six,
            PixelChoice::Seven,
            PixelChoice::Eight,
            PixelChoice::Nine,
        ]
    }
}

#[derive(Debug)]
pub struct PixelGrid {
    m_rows: usize,
    n_cols: usize,
    grid: Vec<Vec<Pixel>>,
}
impl PixelGrid {
    pub fn count_pixel(&self, p: Pixel) -> u32 {
        self.grid
            .iter()
            .map(|row: &Vec<Pixel>| {
                row.iter()
                    .map(|p2| {
                        if p2.value() == p.value() {
                            1 as u32
                        } else {
                            0 as u32
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    pub fn prod_count_pixels(&self, ps: &HashSet<Pixel>) -> u64 {
        let mut pixel_counts: HashMap<&Pixel, u32> = HashMap::new();
        for p in ps {
            pixel_counts.insert(p, 0);
        }

        for row in &self.grid {
            for p in row {
                if let Some(pixel_count) = pixel_counts.get_mut(p) {
                    *pixel_count += 1;
                }
            }
        }

        pixel_counts.values().fold(1u64, |x, y| x * (*y as u64))
    }
}

pub struct PixelGridInputs {
    pub m: u16,
    pub n: u16,
    pub data: Vec<Pixel>,
}
impl TryFrom<PixelGridInputs> for PixelGrid {
    type Error = String;
    fn try_from(mut a: PixelGridInputs) -> Result<Self, Self::Error> {
        if a.data.len() == (a.m as usize) * (a.n as usize) {
            let grid: Vec<Vec<Pixel>> = (0..a.m)
                .map(|_| a.data.drain(0..a.n as usize).collect())
                .collect();
            Ok(PixelGrid {
                m_rows: a.m as usize,
                n_cols: a.n as usize,
                grid,
            })
        } else {
            Err(format!(
                "m:{}*n:{} = {}, but got {} pixels instead",
                a.m,
                a.n,
                (a.m * a.n),
                a.data.len()
            ))
        }
    }
}

#[cfg(test)]
mod pixel_grid_tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn init_all_zeros() {
        fn zero_pixel() -> Pixel {
            Pixel::from(PixelChoice::Zero)
        }
        fn all_other_pixels(exempt: Pixel) -> HashSet<Pixel> {
            HashSet::from_iter(Pixel::values().into_iter().filter(|p| *p != exempt))
        }

        for i in 0..10 {
            for j in 0..10 {
                let pg = PixelGrid::try_from(PixelGridInputs {
                    m: i,
                    n: j,
                    data: (0..i * j).map(|_| zero_pixel()).collect(),
                })
                .unwrap();

                assert_eq!(pg.count_pixel(zero_pixel()), (i as u32) * (j as u32));

                assert_eq!(pg.prod_count_pixels(&all_other_pixels(zero_pixel())), 0);
            }
        }
    }

    #[test]
    fn part1() {
        let pixels: Vec<Pixel> = std::fs::read_to_string("./data/part1.txt")
            .unwrap()
            .trim()
            .chars()
            .map(|c| {
                let c_to_digit = c.to_digit(10).unwrap();
                Pixel::try_from(c_to_digit as u8).unwrap()
            })
            .collect();
        let mut pixels_it = pixels.iter().cloned();
        let (m, n) = (6_u16, 25_u16);

        let mut pixel_grids: Vec<PixelGrid> = Vec::new();
        loop {
            let data: Vec<Pixel> = (&mut pixels_it)
                .take((m as usize) * (n as usize))
                .collect();

            if data.len() == 0 {
                break;
            }

            pixel_grids.push(
                PixelGrid::try_from(
                    PixelGridInputs {
                        m,
                        n,
                        data
                    }
                ).unwrap()
            );
        }

        let pg_least_zeros = pixel_grids.iter().min_by(|pg1, pg2| {
            let pg1_zeros = pg1.count_pixel(Pixel::from(PixelChoice::Zero));
            let pg2_zeros = pg2.count_pixel(Pixel::from(PixelChoice::Zero));
            pg1_zeros.cmp(&pg2_zeros)
        }).unwrap();

        let pixel_one_and_two = {
            let mut v: HashSet<Pixel> = HashSet::new();
            v.insert(Pixel::from(PixelChoice::One));
            v.insert(Pixel::from(PixelChoice::Two));
            v
        };

        assert_eq!(2904, pg_least_zeros.prod_count_pixels(&pixel_one_and_two));
    }
}
