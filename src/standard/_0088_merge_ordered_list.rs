pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
    let mut total:usize = (m+n) as usize;
    let mut m = (m) as usize;
    let mut n = (n) as usize;
    // let list1=&nums1[0_usize..m_real];
    while total !=0 {
        match (m,n){
            (0,0) => {break;}
            (0,1..=999) =>{
                nums1[total-1] = nums2[n-1];
                n-=1;
                total-=1;
            }
            (1..=999, 0) =>{
                nums1[total-1] = nums1[m-1];
                m-=1;
                total-=1;
            }
            _ =>{
                if nums1[m-1] > nums2[n-1] {
                    nums1[total-1] = nums1[m-1];
                    m-=1;
                    total-=1;
                    // println!("{:?},{:?},{:?},{:?},{:?}", nums1, nums2,m,n,total);
                } else {
                    nums1[total-1] = nums2[n-1];
                    n-=1;
                    total-=1;
                    // println!("{:?},{:?},{:?},{:?},{:?}", nums1, nums2,m,n,total);
                }
            }
        }
    }
    println!("{:?}", nums1);
    // println!("{:?}", m);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1,2,3,0,0,0];
        merge(&mut v, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(v, vec![1,2,2,3,5,6]);
    }

        #[test]
    fn a() {
        let mut v = vec![1];
        merge(&mut v, 1, &mut vec![], 0);
        assert_eq!(v, vec![1]);
    }

        #[test]
    fn b() {
        let mut v = vec![0];
        merge(&mut v, 0, &mut vec![1], 1);
        assert_eq!(v, vec![1]);
    }
}
