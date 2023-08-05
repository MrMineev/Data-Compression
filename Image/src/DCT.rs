#[derive(Clone)]
pub struct DctBlock {
    block: Vec<Vec<f32>>,
}

impl DctBlock {
    pub fn empty() -> DctBlock {
        DctBlock {
            block: vec![
                vec![0.0; 8];
                8
            ],
        }
    }

    pub fn new(block: Vec<Vec<f32>>) -> DctBlock {
        DctBlock {
            block,
        }
    }

    pub fn export_dct(&self) -> Vec<Vec<f32>> {
        self.block.clone()
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.block[x][y]
    }
}

const PI: f32 = 3.14159;

fn alpha(n: usize) -> f32 {
    if n == 0 {
        return 0.707106; // 1 / sqrt(2)
    }
    1.0
}

pub fn dct(block: Vec<Vec<f32>>) -> DctBlock {
    let Quantization: Vec<Vec<usize>> = vec![
        /*
        vec![16, 11, 10, 16, 24, 40, 51, 61],
        vec![12, 12, 14, 19, 26, 58, 60, 55],
        vec![14, 13, 16, 24, 40, 57, 69, 56],
        vec![14, 17, 22, 29, 51, 87, 80, 62],
        vec![18, 22, 37, 56, 68, 109, 103, 77],
        vec![24, 35, 55, 64, 81, 104, 113, 92],
        vec![49, 64, 78, 87, 103, 121, 120, 101],
        vec![72, 92, 95, 98, 112, 100, 103, 99]
        */
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
    ];

    let mut mas: Vec<Vec<f32>> = vec![vec![0.0; block[0].len()]; block.len()];
    for u in 0..8 {
        for v in 0..8 {
            let mut total_amount: f32 = 0.25 * alpha(u) * alpha(v);

            let mut adding: f32 = 0.0;

            for x in 0..8 {
                for y in 0..8 {
                    let cosine_one: f32 = (((2.0 * (x as f32) + 1.0) * (u as f32) * PI) / 16.0).cos();
                    let consine_two: f32 = (((2.0 * (y as f32) + 1.0) * (v as f32) * PI) / 16.0).cos();
                    let add: f32 = (block[x][y] as f32) * cosine_one * consine_two;

                    adding += add;
                }
            }
            total_amount *= adding;

            mas[u][v] = (total_amount / (Quantization[u][v] as f32) ).round();
        }
    }

    return DctBlock::new(mas);
}

pub fn inverse_dct(block: Vec<Vec<f32>>) -> DctBlock {
    let Quantization: Vec<Vec<usize>> = vec![
        /*vec![16, 11, 10, 16, 24, 40, 51, 61],
        vec![12, 12, 14, 19, 26, 58, 60, 55],
        vec![14, 13, 16, 24, 40, 57, 69, 56],
        vec![14, 17, 22, 29, 51, 87, 80, 62],
        vec![18, 22, 37, 56, 68, 109, 103, 77],
        vec![24, 35, 55, 64, 81, 104, 113, 92],
        vec![49, 64, 78, 87, 103, 121, 120, 101],
        vec![72, 92, 95, 98, 112, 100, 103, 99]*/

        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
        vec![30, 30, 30, 30, 30, 30, 30, 30],
    ];

    let mut mas: Vec<Vec<f32>> = vec![vec![0.0; block[0].len()]; block.len()];
    for x in 0..8 {
        for y in 0..8 {
            let mut total_amount: f32 = 0.25;

            let mut adding: f32 = 0.0;

            for u in 0..8 {
                for v in 0..8 {
                    let cosine_one: f32 = (((2.0 * (x as f32) + 1.0) * (u as f32) * PI) / 16.0).cos();
                    let consine_two: f32 = (((2.0 * (y as f32) + 1.0) * (v as f32) * PI) / 16.0).cos();
                    let add: f32 = alpha(u) * alpha(v) * (block[u][v] as f32) * cosine_one * consine_two;

                    adding += add;
                }
            }

            total_amount *= adding;

            mas[x][y] = total_amount * (Quantization[x][y] as f32);
        }
    }

    return DctBlock::new(mas);
}

