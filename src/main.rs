// use std::borrow::Borrow;

mod q217;
mod q219;
mod q220;
fn main() {
    // 217. Contains Duplicate
    assert_eq!(q217::Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(
        q217::Solution::contains_duplicate_v2(vec![1, 2, 3, 4, 5, 6]),
        false
    );
    assert_eq!(
        q217::Solution::contains_duplicate_v2(vec![1, 2, 3, 4, 5, 6, 7]),
        false
    );
    assert_eq!(
        q217::Solution::contains_duplicate_v2(vec![1, 2, 3, 4, 5, 6, 7, 1]),
        true
    );

    // 219. Contains Duplicate II
    assert_eq!(
        q219::Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    );
    assert_eq!(
        q219::Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
        true
    );
    assert_eq!(
        q219::Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
    assert_eq!(
        q219::Solution::contains_nearby_duplicate(vec![1, 1], 1),
        true
    );
    assert_eq!(
        q219::Solution::contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6], 3),
        false
    );
    assert_eq!(
        q219::Solution::contains_nearby_duplicate(vec![99, 99], 2),
        true
    );

    // 220. Contains Duplicate III
    assert_eq!(
        q220::Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
        true
    );
    assert_eq!(
        q220::Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
        true
    );
    assert_eq!(
        q220::Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
        false
    );
    assert_eq!(
        q220::Solution::contains_nearby_almost_duplicate(vec![-2147483648,2147483647], 1, 1),
        false
    );
    assert_eq!(
        q220::Solution::contains_nearby_almost_duplicate(vec![2147483646, 2147483647], 3, 3),
        true
    );
    assert_eq!(
        q220::Solution::contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
        false
    )
}