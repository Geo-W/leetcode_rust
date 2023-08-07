    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let mut vec = Vec::new();
        for i in nums1{
            set.insert(i);
        }

        for i in nums2{
            match set.remove(&i) {
                true => {
                    vec.push(i)
                }
                false =>{}
            }
        }
        vec
    }


    #[cfg(test)]
mod tests {
        use crate::standard::_0349_intersection_of_two_arrays::intersection;

        #[test]
    fn it_works() {
        assert_eq!(intersection(vec![1,2,2,1], vec![2,2]), vec![2]);
    }

}
