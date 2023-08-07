    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut vec = Vec::new();
        for i in nums1{
            match map.get_mut(&i) {
                Some(mut v) => {
                    *v +=1;
                }
                None => {
                    map.insert(i, 1);
                }
            }
        }

        for i in nums2{
            match map.get_mut(&i) {
                Some(mut v) => {
                    if *v > 1 {
                        *v -=1;
                    } else {
                        map.remove(&i);
                    }
                    vec.push(i);
                }
                None => {

                }
            }
        }
        vec
    }


    #[cfg(test)]
mod tests {
        use crate::standard::_0350_intersection_of_two_arrays_II::intersect;

        #[test]
    fn it_works() {
        assert_eq!(intersect(vec![1,2,2,1], vec![2,2]), vec![2,2]);
    }

}
