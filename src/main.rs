// use std::borrow::Borrow;
use std::collections::HashMap;

#[path ="./ds/linked_list.rs"] mod linked_list;
mod q121;
mod q141;
mod q217;
mod q219;
mod q220;
mod q303;
mod q53;
mod q70;

fn main() {
    let mut condition = HashMap::new();
    condition.insert("q53", false);
    condition.insert("q70", false);
    condition.insert("q121", false);
    condition.insert("q217", false);
    condition.insert("q219", false);
    condition.insert("q220", false);
    condition.insert("q303", false);
    condition.insert("q141", false);

    if *condition.get("q53").unwrap_or(&false) {
        println!("Question 53 - Maximum SubArray");
        assert_eq!(
            q53::Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(q53::Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(q53::Solution::max_sub_array(vec![-1, -2]), -1);
        assert_eq!(q53::Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        println!("------- Version 2");
        assert_eq!(
            q53::Solution::max_sub_array_v2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(q53::Solution::max_sub_array_v2(vec![-1]), -1);
        assert_eq!(q53::Solution::max_sub_array_v2(vec![-1, -2]), -1);
        assert_eq!(q53::Solution::max_sub_array_v2(vec![5, 4, -1, 7, 8]), 23);
    }

    if *condition.get("q70").unwrap_or(&false) {
        println!("Question 70 - Climbing Stairs");
        // 70. Climbing stairs
        assert_eq!(q70::Solution::climb_stairs(2), 2);
        assert_eq!(q70::Solution::climb_stairs(3), 3);
        assert_eq!(q70::Solution::climb_stairs(4), 5);
        assert_eq!(q70::Solution::climb_stairs(5), 8);
        assert_eq!(q70::Solution::climb_stairs(6), 13);
        assert_eq!(q70::Solution::climb_stairs(7), 21);
        assert_eq!(q70::Solution::climb_stairs(8), 34);
        assert_eq!(q70::Solution::climb_stairs(9), 55);
        assert_eq!(q70::Solution::climb_stairs(10), 89);
        assert_eq!(q70::Solution::climb_stairs(11), 144);
        assert_eq!(q70::Solution::climb_stairs(12), 233);
        assert_eq!(q70::Solution::climb_stairs(13), 377);
        assert_eq!(q70::Solution::climb_stairs(14), 610);
        assert_eq!(q70::Solution::climb_stairs(15), 987);
        assert_eq!(q70::Solution::climb_stairs(16), 1597);
        assert_eq!(q70::Solution::climb_stairs(17), 2584);
        assert_eq!(q70::Solution::climb_stairs(18), 4181);
        assert_eq!(q70::Solution::climb_stairs(19), 6765);
        assert_eq!(q70::Solution::climb_stairs(20), 10946);
        assert_eq!(q70::Solution::climb_stairs(21), 17711);
        assert_eq!(q70::Solution::climb_stairs(22), 28657);
        assert_eq!(q70::Solution::climb_stairs(23), 46368);
        assert_eq!(q70::Solution::climb_stairs(24), 75025);
        assert_eq!(q70::Solution::climb_stairs(25), 121393);
        assert_eq!(q70::Solution::climb_stairs(26), 196418);
        assert_eq!(q70::Solution::climb_stairs(27), 317811);
        assert_eq!(q70::Solution::climb_stairs(28), 514229);
        assert_eq!(q70::Solution::climb_stairs(29), 832040);
        assert_eq!(q70::Solution::climb_stairs(30), 1346269);
        assert_eq!(q70::Solution::climb_stairs(31), 2178309);
        assert_eq!(q70::Solution::climb_stairs(32), 3524578);
        assert_eq!(q70::Solution::climb_stairs(33), 5702887);
        assert_eq!(q70::Solution::climb_stairs(34), 9227465);
        assert_eq!(q70::Solution::climb_stairs(35), 14930352);
        assert_eq!(q70::Solution::climb_stairs(36), 24157817);
        assert_eq!(q70::Solution::climb_stairs(37), 39088169);
        assert_eq!(q70::Solution::climb_stairs(38), 63245986);
        assert_eq!(q70::Solution::climb_stairs(39), 102334155);
        assert_eq!(q70::Solution::climb_stairs(40), 165580141);
        assert_eq!(q70::Solution::climb_stairs(41), 267914296);
        assert_eq!(q70::Solution::climb_stairs(42), 433494437);
        assert_eq!(q70::Solution::climb_stairs(43), 701408733);
        assert_eq!(q70::Solution::climb_stairs(44), 1134903170);
    }

    if *condition.get("q121").unwrap_or(&false) {
        println!("Question 121 - Best Time to Buy and Sell Stock");
        // 121. Best Time to Buy and Sell Stock
        assert_eq!(q121::Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(q121::Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(q121::Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(q121::Solution::max_profit(vec![7, 6, 5, 4, 3, 2, 1]), 0);
        assert_eq!(q121::Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7]), 6);
        assert_eq!(q121::Solution::max_profit(vec![2, 4, 1]), 2);
        assert_eq!(q121::Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
    }

    if *condition.get("q141").unwrap_or(&false) {
        println!("Question 141 - Linked list cycle");
        // 141. Linked list cycle
        let linked_list = q141::ListNode::init(vec![1, 2, 3, 4, 5]);
        assert_eq!(q141::Solution::has_cycle(&linked_list), false)
    }

    if *condition.get("q217").unwrap_or(&false) {
        println!("Question 217 - Contains Duplicate");
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
    }

    if *condition.get("q219").unwrap_or(&false) {
        println!("Question 219 - Contains Duplicate II");
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
    }

    if *condition.get("q220").unwrap_or(&false) {
        println!("Question 220 - Contains Duplicate III");
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
            q220::Solution::contains_nearby_almost_duplicate(vec![-2147483648, 2147483647], 1, 1),
            false
        );
        assert_eq!(
            q220::Solution::contains_nearby_almost_duplicate(vec![2147483646, 2147483647], 3, 3),
            true
        );
        assert_eq!(
            q220::Solution::contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
            false
        );
    }

    if *condition.get("q303").unwrap_or(&false) {
        println!("Question 303 - Range Sum Query Immutable");
        let obj = q303::NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(obj.sum_range(0, 2), 1);
        assert_eq!(obj.sum_range(2, 5), -1);
        assert_eq!(obj.sum_range(0, 5), -3);
    }
}
