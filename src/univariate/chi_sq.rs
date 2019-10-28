use crate::{
    Convolution, ConvolutionError, ConvolutionResult,
    prelude::*,
    validation::{Result, ValidationError},
};
use rand;
use spaces::real::PositiveReals;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct ChiSq {
    pub k: usize,
}

impl ChiSq {
    pub fn new(k: usize) -> Result<ChiSq> {
        ValidationError::assert_gte(k, 1)
            .map(|(k, _)| ChiSq::new_unchecked(k))
    }

    pub fn new_unchecked(k: usize) -> ChiSq {
        ChiSq { k }
    }
}

impl Into<rand_distr::ChiSquared<f64>> for ChiSq {
    fn into(self) -> rand_distr::ChiSquared<f64> {
        rand_distr::ChiSquared::new(self.k as f64).unwrap()
    }
}

impl Into<rand_distr::ChiSquared<f64>> for &ChiSq {
    fn into(self) -> rand_distr::ChiSquared<f64> {
        rand_distr::ChiSquared::new(self.k as f64).unwrap()
    }
}

impl Distribution for ChiSq {
    type Support = PositiveReals;

    fn support(&self) -> PositiveReals {
        PositiveReals
    }

    fn cdf(&self, x: f64) -> Probability {
        use special_fun::FloatSpecial;

        let k = self.k as f64;
        let ko2 = k / 2.0;

        Probability::new_unchecked(ko2.gammainc(x / 2.0) / ko2.gamma())
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        use rand_distr::Distribution;

        let sampler: rand_distr::ChiSquared<f64> = self.into();

        sampler.sample(rng)
    }
}

impl ContinuousDistribution for ChiSq {
    fn pdf(&self, x: f64) -> f64 {
        use special_fun::FloatSpecial;

        let k = self.k as f64;
        let ko2 = k / 2.0;
        let norm = 2.0f64.powf(ko2) * ko2.gamma();

        x.powf(ko2 - 1.0) * (-x / 2.0).exp() / norm
    }
}

impl UnivariateMoments for ChiSq {
    fn mean(&self) -> f64 {
        self.k as f64
    }

    fn variance(&self) -> f64 {
        (2 * self.k) as f64
    }

    fn skewness(&self) -> f64 {
        (8.0 / self.k as f64).sqrt()
    }

    fn excess_kurtosis(&self) -> f64 {
        12.0 / self.k as f64
    }
}

impl Quantiles for ChiSq {
    fn quantile(&self, _: Probability) -> f64 {
        unimplemented!()
    }

    fn median(&self) -> f64 {
        let k = self.k as f64;

        k * (1.0 - 2.0 / 9.0 / k).powi(3)
    }
}

impl Modes for ChiSq {
    fn modes(&self) -> Vec<f64> {
        vec![(self.k - 2).max(0) as f64]
    }
}

impl Entropy for ChiSq {
    fn entropy(&self) -> f64 {
        use special_fun::FloatSpecial;

        let k = self.k as f64;
        let ko2 = k / 2.0;

        ko2 + (2.0 * ko2.gamma()).ln() + (1.0 - ko2) * ko2.digamma()
    }
}

impl Convolution<ChiSq> for ChiSq {
    fn convolve(self, rv: ChiSq) -> ConvolutionResult<ChiSq> {
        Self::convolve_pair(self, rv)
    }

    fn convolve_pair(a: ChiSq, b: ChiSq) -> ConvolutionResult<ChiSq> {
        if a.k == b.k {
            Ok(ChiSq::new_unchecked(a.k + b.k))
        } else {
            Err(ConvolutionError::MixedParameters)
        }
    }
}

impl fmt::Display for ChiSq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChiSq({})", self.k)
    }
}
