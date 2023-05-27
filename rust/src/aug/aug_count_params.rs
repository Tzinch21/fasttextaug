/// Three parameters to calculate the number of elements that will be augmented
#[derive(Clone)]
pub struct AugCountParams {
    /// Minimum elements for augmentation
    aug_min: Option<usize>,
    /// Maximum elements for augmentation
    aug_max: Option<usize>,
    /// Fraction of the number of input elements for augmentation
    aug_p: Option<f32>,
}

impl AugCountParams {
    pub fn new(aug_min: Option<usize>, aug_max: Option<usize>, aug_p: Option<f32>) -> Self {
        AugCountParams {
            aug_min,
            aug_max,
            aug_p,
        }
    }

    /// Calculate number of elements to be augmented from input size
    pub fn calculate_aug_cnt(&self, size: usize) -> usize {
        let percent = self.aug_p.unwrap_or(0.3);
        if size == 0 {
            return 0;
        }
        let count = f32::ceil(percent * size as f32) as usize;
        if let Some(val) = self.aug_min {
            if val > count {
                return val;
            }
        }
        if let Some(val) = self.aug_max {
            if val < count {
                return val;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_aug_all_params() {
        // Test default behaviour
        let aug_params = AugCountParams::new(Some(3), Some(7), Some(0.5));
        let res = aug_params.calculate_aug_cnt(10);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_calc_aug_min_thres() {
        // Test minimum threshold
        let aug_params = AugCountParams::new(Some(5), Some(7), Some(0.2));
        let res = aug_params.calculate_aug_cnt(10);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_calc_aug_max_thres() {
        // Test maximum threshold
        let aug_params = AugCountParams::new(Some(5), Some(7), Some(0.9));
        let res = aug_params.calculate_aug_cnt(10);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_calc_aug_all_none() {
        // Test default input values
        let aug_params = AugCountParams::new(None, None, None);
        let res = aug_params.calculate_aug_cnt(10);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_calc_aug_zero() {
        // Test zero input
        let aug_params = AugCountParams::new(None, None, None);
        let res = aug_params.calculate_aug_cnt(0);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_calc_aug_p_zero() {
        // Test zero fraction, but non-zero minimum threshold
        let aug_params = AugCountParams::new(Some(3), None, Some(0.0));
        let res = aug_params.calculate_aug_cnt(10);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_calc_aug_p_negative() {
        // Test negative fraction
        let aug_params = AugCountParams::new(Some(3), None, Some(-0.3));
        let res = aug_params.calculate_aug_cnt(10);
        assert_eq!(res, 3);
    }
}
