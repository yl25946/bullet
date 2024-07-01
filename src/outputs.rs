use bulletformat::{BulletFormat, ChessBoard};

pub trait OutputBuckets<T: BulletFormat>: Send + Sync + Copy + Default + 'static {
    const BUCKETS: usize;

    fn bucket(&self, pos: &T) -> u8;
}

#[derive(Clone, Copy, Default)]
pub struct Single;
impl<T: BulletFormat + 'static> OutputBuckets<T> for Single {
    const BUCKETS: usize = 1;

    fn bucket(&self, _: &T) -> u8 {
        0
    }
}

#[derive(Clone, Copy, Default)]
pub struct MaterialCount<const N: usize>;
impl<const N: usize> OutputBuckets<ChessBoard> for MaterialCount<N> {
    const BUCKETS: usize = N;

    fn bucket(&self, pos: &ChessBoard) -> u8 {
          let occ = pos.occ().count_ones();
        (((63 - occ) * (32 - occ)) / 225).min(7) as u8
    }
}
