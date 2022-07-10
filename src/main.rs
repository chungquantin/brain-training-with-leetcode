// use std::borrow::Borrow;
use std::{collections::HashMap, vec};

mod algo;
#[path = "./ds/linked_list.rs"]
mod linked_list;
mod others;
mod q1143_longest_common_subsequence;
mod q118_generate;
mod q1192_critical_connection;
mod q11_container_with_most_water;
mod q121_max_profit;
mod q1334_find_the_city;
mod q139_word_break;
mod q141_has_cycle;
mod q150_eval_rpn;
mod q153_find_min;
mod q155_min_stack;
mod q15_three_sum;
mod q162_find_peak_element;
mod q189_rotate_array;
mod q1963_min_swaps;
mod q1_two_sum;
mod q200_number_of_islands;
mod q207_can_finish;
mod q208_trie;
mod q209_min_sub_array_len;
mod q217_contains_duplicate;
mod q219_contains_nearby_duplicate;
mod q220_contains_nearby_almost_duplicate;
mod q232_queue;
mod q2335_fill_cups;
mod q238_product_except_self;
mod q239_sliding_window_maximum;
mod q268_missing_number;
mod q283_move_zeroes;
mod q287_find_duplicate;
mod q300_length_of_lis;
mod q303_sum_range;
mod q304_range_sum_query_2D_immutable;
mod q322_coin_change;
mod q33_search;
mod q344_reverse_string;
mod q350_intersect;
mod q35_search_insert;
mod q383_can_construct;
mod q39_combination_sum;
mod q3_length_of_longest_substring;
mod q407_trapping_rain_water_II;
mod q409_longest_palindrome;
mod q417_pacific_alantic;
mod q42_trapping_rain_water;
mod q438_find_all_anagrams_in_a_string;
mod q46_permutation;
mod q47_permutation_unique;
mod q496_next_greater_element_I;
mod q51_n_queens;
mod q53_max_sub_array;
mod q542_update_matrix;
mod q547_number_of_provinces;
mod q54_spiral_matrix;
mod q566_matrix_reshape;
mod q567_check_inclusion;
mod q56_merge_intervals;
mod q57_insert_intervals;
mod q58_length_of_last_word;
mod q67_add_binary;
mod q695_max_area_of_island;
mod q6_zigzag_conversion;
mod q70_climb_stairs;
mod q713_subarray_product_smaller_than_k;
mod q733_flood_fill;
mod q76_minimum_window_substring;
mod q78_subsets;
mod q82_remove_duplicates_from_sorted_list_II;
mod q844_backspace_string_compare;
mod q84_largest_retangle_area;
mod q862_shortest_subarray_with_sum_at_least_k;
mod q88_merge;
mod q896_monotonic_array;
mod q90_subsets_with_duplicate;
mod q921_min_add_to_make_valid;
mod q973_k_closest;
mod q977_sorted_squares;
mod q986_interval_list_intersections;
mod q994_oranges_rotting;

pub use algo::*;
use futures::future::join_all;
pub use rand::Rng;

fn gen_random(number_of_elements: u32, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..number_of_elements)
        .map(|_| rng.gen_range(min..max))
        .collect()
}

async fn sort_algorithm_test(run_this_test: bool) {
    if run_this_test {
        let s = SortAlgorithm {
            inputs: vec![
                gen_random(10, 0, 50),
                gen_random(100, 0, 100),
                gen_random(1000, 0, 1000),
                gen_random(10000, 0, 100000),
            ],
            sort: 1,
        };

        let methods = vec![
            (SortAlgorithmName::BubbleSort, false),
            (SortAlgorithmName::InsertionSort, false),
            (SortAlgorithmName::MergeSort, false),
        ];
        let mut async_handle = vec![];
        for method in methods {
            let handle = async {
                let s_clone = s.clone();
                let output_shown = false;
                s_clone.frame(method.0, output_shown);
            };
            async_handle.push(handle);
        }
        join_all(async_handle).await;
    }
}

#[tokio::main]
async fn main() {
    let mut condition = HashMap::new();
    let (run_this_test, no) = (true, false);
    sort_algorithm_test(no).await;
    condition.insert("q1", no);
    condition.insert("q3", no);
    condition.insert("q6", run_this_test);
    condition.insert("q11", no);
    condition.insert("q15", no);
    condition.insert("q33", no);
    condition.insert("q35", no);
    condition.insert("q39", no);
    condition.insert("q42", no);
    condition.insert("q46", no);
    condition.insert("q47", no);
    condition.insert("q53", no);
    condition.insert("q54", no);
    condition.insert("q56", no);
    condition.insert("q57", no);
    condition.insert("q58", no);
    condition.insert("q67", no);
    condition.insert("q70", no);
    condition.insert("q76", no);
    condition.insert("q78", no);
    condition.insert("q84", no);
    condition.insert("q88", no);
    condition.insert("q90", no);
    condition.insert("q118", no);
    condition.insert("q121", no);
    condition.insert("q139", no);
    condition.insert("q141", no);
    condition.insert("q150", no);
    condition.insert("q153", no);
    condition.insert("q200", no);
    condition.insert("q207", no);
    condition.insert("q209", no);
    condition.insert("q217", no);
    condition.insert("q219", no);
    condition.insert("q220", no);
    condition.insert("q238", no);
    condition.insert("q239", no);
    condition.insert("q268", no);
    condition.insert("q283", no);
    condition.insert("q287", no);
    condition.insert("q303", no);
    condition.insert("q304", no);
    condition.insert("q322", no);
    condition.insert("q350", no);
    condition.insert("q383", no);
    condition.insert("q407", no);
    condition.insert("q417", no);
    condition.insert("q438", no);
    condition.insert("q496", no);
    condition.insert("q542", no);
    condition.insert("q547", no);
    condition.insert("q566", no);
    condition.insert("q567", no);
    condition.insert("q695", no);
    condition.insert("q713", no);
    condition.insert("q733", no);
    condition.insert("q844", no);
    condition.insert("q862", no);
    condition.insert("q921", no);
    condition.insert("q973", no);
    condition.insert("q977", no);
    condition.insert("q986", no);
    condition.insert("q994", no);
    condition.insert("q1192", no);
    condition.insert("q1334", no);
    condition.insert("q2335", no);

    if *condition.get("q268").unwrap_or(&false) {
        println!("Question 268 - Missing Number");
        assert_eq!(
            q268_missing_number::Solution::missing_number(vec![3, 0, 1]),
            2
        );
        assert_eq!(q268_missing_number::Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(
            q268_missing_number::Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }

    if *condition.get("q287").unwrap_or(&false) {
        println!("Question 268 - Find the Duplicate Number");
        assert_eq!(
            q287_find_duplicate::Solution::find_duplicate(vec![1, 3, 4, 2, 2]),
            2
        );
        assert_eq!(
            q287_find_duplicate::Solution::find_duplicate(vec![1, 3, 4, 2, 1]),
            1
        );
        assert_eq!(
            q287_find_duplicate::Solution::find_duplicate_tortoise_hare(vec![1, 3, 4, 2, 1]),
            1
        );
        assert_eq!(
            q287_find_duplicate::Solution::find_duplicate_tortoise_hare(vec![
                2, 5, 9, 6, 9, 3, 8, 9, 7, 1
            ]),
            9
        );
    }

    if *condition.get("q1").unwrap_or(&false) {
        assert_eq!(
            q1_two_sum::Solution::two_sum(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
        assert_eq!(q1_two_sum::Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    if *condition.get("q53").unwrap_or(&false) {
        println!("Question 53 - Maximum SubArray");
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_prefix_sum(vec![
                -2, 1, -3, 4, -1, 2, 1, -5, 4
            ]),
            6
        );
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_prefix_sum(vec![-1]),
            -1
        );
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_prefix_sum(vec![-1, -2]),
            -1
        );
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_prefix_sum(vec![5, 4, -1, 7, 8]),
            23
        );
        println!("------- Version 2");
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_v2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(q53_max_sub_array::Solution::max_sub_array_v2(vec![-1]), -1);
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_v2(vec![-1, -2]),
            -1
        );
        assert_eq!(
            q53_max_sub_array::Solution::max_sub_array_v2(vec![5, 4, -1, 7, 8]),
            23
        );
    }

    if *condition.get("q70").unwrap_or(&false) {
        println!("Question 70 - Climbing Stairs");
        // 70. Climbing stairs
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(2), 2);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(3), 3);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(4), 5);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(5), 8);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(6), 13);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(7), 21);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(8), 34);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(9), 55);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(10), 89);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(11), 144);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(12), 233);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(13), 377);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(14), 610);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(15), 987);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(16), 1597);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(17), 2584);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(18), 4181);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(19), 6765);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(20), 10946);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(21), 17711);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(22), 28657);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(23), 46368);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(24), 75025);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(25), 121393);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(26), 196418);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(27), 317811);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(28), 514229);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(29), 832040);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(30), 1346269);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(31), 2178309);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(32), 3524578);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(33), 5702887);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(34), 9227465);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(35), 14930352);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(36), 24157817);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(37), 39088169);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(38), 63245986);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(39), 102334155);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(40), 165580141);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(41), 267914296);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(42), 433494437);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(43), 701408733);
        assert_eq!(q70_climb_stairs::Solution::climb_stairs(44), 1134903170);
    }

    if *condition.get("q121").unwrap_or(&false) {
        println!("Question 121 - Best Time to Buy and Sell Stock");
        // 121. Best Time to Buy and Sell Stock
        assert_eq!(
            q121_max_profit::Solution::max_profit(vec![7, 1, 5, 3, 6, 4]),
            5
        );
        assert_eq!(
            q121_max_profit::Solution::max_profit(vec![7, 6, 4, 3, 1]),
            0
        );
        assert_eq!(
            q121_max_profit::Solution::max_profit(vec![1, 2, 3, 4, 5]),
            4
        );
        assert_eq!(
            q121_max_profit::Solution::max_profit(vec![7, 6, 5, 4, 3, 2, 1]),
            0
        );
        assert_eq!(
            q121_max_profit::Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7]),
            6
        );
        assert_eq!(q121_max_profit::Solution::max_profit(vec![2, 4, 1]), 2);
        assert_eq!(
            q121_max_profit::Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]),
            2
        );
    }

    if *condition.get("q141").unwrap_or(&false) {
        println!("Question 141 - Linked list cycle");
        // 141. Linked list cycle
        let linked_list = &mut q141_has_cycle::ListNode::default();
        linked_list.init(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(q141_has_cycle::Solution::has_cycle(&linked_list), false)
    }

    if *condition.get("q203").unwrap_or(&false) {
        println!("Question 203 - Remove elements from linked list");
    }

    if *condition.get("q217").unwrap_or(&false) {
        println!("Question 217 - Contains Duplicate");
        // 217. Contains Duplicate
        assert_eq!(
            q217_contains_duplicate::Solution::contains_duplicate(vec![1, 2, 3, 1]),
            true
        );
        assert_eq!(
            q217_contains_duplicate::Solution::contains_duplicate_v2(vec![1, 2, 3, 4, 5, 6]),
            false
        );
        assert_eq!(
            q217_contains_duplicate::Solution::contains_duplicate_v2(vec![1, 2, 3, 4, 5, 6, 7]),
            false
        );
        assert_eq!(
            q217_contains_duplicate::Solution::contains_duplicate_v2(vec![1, 2, 3, 4, 5, 6, 7, 1]),
            true
        );
    }

    if *condition.get("q219").unwrap_or(&false) {
        println!("Question 219 - Contains Duplicate II");
        // 219. Contains Duplicate II
        assert_eq!(
            q219_contains_nearby_duplicate::Solution::contains_nearby_duplicate(
                vec![1, 2, 3, 1],
                3
            ),
            true
        );
        assert_eq!(
            q219_contains_nearby_duplicate::Solution::contains_nearby_duplicate(
                vec![1, 0, 1, 1],
                1
            ),
            true
        );
        assert_eq!(
            q219_contains_nearby_duplicate::Solution::contains_nearby_duplicate(
                vec![1, 2, 3, 1, 2, 3],
                2
            ),
            false
        );
        assert_eq!(
            q219_contains_nearby_duplicate::Solution::contains_nearby_duplicate(vec![1, 1], 1),
            true
        );
        assert_eq!(
            q219_contains_nearby_duplicate::Solution::contains_nearby_duplicate(
                vec![1, 2, 3, 4, 5, 6],
                3
            ),
            false
        );
        assert_eq!(
            q219_contains_nearby_duplicate::Solution::contains_nearby_duplicate(vec![99, 99], 2),
            true
        );
    }

    if *condition.get("q220").unwrap_or(&false) {
        println!("Question 220 - Contains Duplicate III");
        // 220. Contains Duplicate III
        assert_eq!(
            q220_contains_nearby_almost_duplicate::Solution::contains_nearby_almost_duplicate(
                vec![1, 2, 3, 1],
                3,
                0
            ),
            true
        );
        assert_eq!(
            q220_contains_nearby_almost_duplicate::Solution::contains_nearby_almost_duplicate(
                vec![1, 0, 1, 1],
                1,
                2
            ),
            true
        );
        assert_eq!(
            q220_contains_nearby_almost_duplicate::Solution::contains_nearby_almost_duplicate(
                vec![1, 5, 9, 1, 5, 9],
                2,
                3
            ),
            false
        );
        assert_eq!(
            q220_contains_nearby_almost_duplicate::Solution::contains_nearby_almost_duplicate(
                vec![-2147483648, 2147483647],
                1,
                1
            ),
            false
        );
        assert_eq!(
            q220_contains_nearby_almost_duplicate::Solution::contains_nearby_almost_duplicate(
                vec![2147483646, 2147483647],
                3,
                3
            ),
            true
        );
        assert_eq!(
            q220_contains_nearby_almost_duplicate::Solution::contains_nearby_almost_duplicate(
                vec![-1, 2147483647],
                1,
                2147483647
            ),
            false
        );
    }

    if *condition.get("q303").unwrap_or(&false) {
        println!("Question 303 - Range Sum Query Immutable");
        let obj = q303_sum_range::NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(obj.sum_range(0, 2), 1);
        assert_eq!(obj.sum_range(2, 5), -1);
        assert_eq!(obj.sum_range(0, 5), -3);
    }

    if *condition.get("q3").unwrap_or(&false) {
        println!("Question 3 - Longest Substring Without Repeating Characters");
        assert_eq!(
            q3_length_of_longest_substring::Solution::length_of_longest_substring(String::from(
                "dvdf"
            )),
            3
        );
        assert_eq!(
            q3_length_of_longest_substring::Solution::length_of_longest_substring(String::from(
                "pwfwge"
            )),
            4
        );
        assert_eq!(
            q3_length_of_longest_substring::Solution::length_of_longest_substring(String::from(
                "pwwkew"
            )),
            3
        );
        assert_eq!(
            q3_length_of_longest_substring::Solution::length_of_longest_substring(String::from(
                "abcdabcde"
            )),
            5
        );
        assert_eq!(
            q3_length_of_longest_substring::Solution::length_of_longest_substring(String::from(
                "abfklameqsadonxzvjbasd"
            )),
            13
        );
    }

    if *condition.get("q58").unwrap_or(&false) {
        println!("Question 58 - Length of last word");
        assert_eq!(
            q58_length_of_last_word::Solution::length_of_last_word(String::from("hello world")),
            "world".len() as i32
        );

        assert_eq!(
            q58_length_of_last_word::Solution::length_of_last_word(String::from(
                "   fly me   to   the moon  "
            )),
            "moon".len() as i32
        )
    }

    if *condition.get("q88").unwrap_or(&false) {
        println!("Question 88 - Merged Sorted Array");
        assert_eq!(
            q88_merge::Solution::merge(&mut vec![1, 2, 3], 3, &mut vec![2, 5, 6], 3),
            vec![1, 2, 2, 3, 5, 6]
        );
        assert_eq!(
            q88_merge::Solution::merge(&mut vec![1], 1, &mut vec![], 0),
            vec![1]
        );
        assert_eq!(
            q88_merge::Solution::merge(&mut vec![1], 1, &mut vec![1, 0], 1),
            vec![1, 1]
        );
        assert_eq!(
            q88_merge::Solution::merge(
                &mut vec![1, 2, 3, 3, 3, 6, 10],
                7,
                &mut vec![2, 5, 6, 6, 8, 9, 13],
                7
            ),
            vec![1, 2, 2, 3, 3, 3, 5, 6, 6, 6, 8, 9, 10, 13]
        );
    }

    if *condition.get("q350").unwrap_or(&false) {
        println!("Question 350 - Intersection of Two Arrays II");
        assert_eq!(
            q350_intersect::Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            q350_intersect::Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }

    if *condition.get("q35").unwrap_or(&false) {
        println!("Question 35 - Search Insert Position");
        assert_eq!(
            q35_search_insert::Solution::search_insert(vec![1, 3, 5, 6], 5),
            2
        );
        assert_eq!(
            q35_search_insert::Solution::search_insert(vec![1, 3, 5, 6], 2),
            1
        );
        assert_eq!(
            q35_search_insert::Solution::search_insert(vec![1, 3, 5, 6], 7),
            4
        );
        assert_eq!(
            q35_search_insert::Solution::search_insert(vec![1, 3, 5, 6], 0),
            0
        );
    }

    if *condition.get("q977").unwrap_or(&false) {
        println!("Question 977 - Squares of Sorted Array");
        assert_eq!(
            q977_sorted_squares::Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        )
    }

    if *condition.get("q118").unwrap_or(&false) {
        println!("Question 118 - Pascal's Triangle");
        assert_eq!(
            q118_generate::Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );

        assert_eq!(q118_generate::Solution::generate(4), vec![vec![1]]);
    }

    if *condition.get("q283").unwrap_or(&false) {
        println!("Question 283 - Move Zeros");
        let mut arr1 = vec![0, 1, 0, 3, 12];
        q283_move_zeroes::Solution::move_zeroes(&mut arr1);
        assert_eq!(arr1, vec![1, 3, 12, 0, 0]);
    }

    if *condition.get("q567").unwrap_or(&false) {
        println!("Question 567 - Permutation in String");
        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("ab"),
                String::from("eidbaooo")
            ),
            true
        );
        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("ab"),
                String::from("eidboaoo")
            ),
            false
        );
        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("adc"),
                String::from("dcda")
            ),
            true
        );
        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("abc"),
                String::from("aaaacccccbbbb")
            ),
            false
        );
        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(String::from("ab"), String::from("a")),
            false
        );

        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("park"),
                String::from("spake")
            ),
            false
        );
        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("r"),
                String::from("pilmtnzraxj")
            ),
            true
        );

        assert_eq!(
            q567_check_inclusion::Solution::check_inclusion(
                String::from("vxqakfyaqahufxfizupjrkkifjlbfqfeprqrfjvxnubntdtlvz"),
                String::from("oumgfmlrbivgnrvsfslnheghnbhhicbvaddqadwicekguhjairhpqtebqvzyxdfodntxmoqtffgmsomnhndbrffxmuyjvqazwfvugyvmshxignfenmkihorjkshwyuxxkxidpkalqmdnxxmnovhangcwggjwjrletxhelehcipflvqyueptgjxugyipegamjbweqdfeswrjepjjlviuhfikfaojbhrujdfpnenrvajrdplwaevpbzkcdkyhidbgizwofjoxfvnkzhmwvegnfipgmnikljmmcrleffceqsxrxgsjmjmaflnxtigfoeaafsrjaxaqumhatpdeueridyxfjajsjkcrfwkwguclqtrefwevwalmcvoiragprwwnxpwgqoyulsadfhstduevikkkwmofklomdvpdcnxxsotudmkjgakbzopzengbfrvhiikzrsmmurewnnquydhntpqprtyvouatjevvoerbnzunwtdiigohcctulnvpgfbtdjmjsvszphulflmolunohifxsycaiifkngpuycnlihebmhaapureljxlqozydijsbcitfrdjhbdppvhpfjfuxujwfkcezyvhbityajxcccbuyvoluzndgjsyvrbluvagqtuujpsxjcgdtkbhaxsatmellmotdeoxsxjsywuavjqpzjkcpsrowqgogovcouosjhblhdmbkeidafpimjaiypownlcilpahbszmmcwbqmztpdmbqeuizjwdifruojquahdtdukydxgehpourjytveajjvtpiphihkmgqpmnuukpqtjyyqhfsiezugszdbjsseaxxdxomhouvetgcvfeuvwszmhgpkkorubvahxoufpfcukgchdubddpijsolrkvgiizhqefdhvpzvpmuxnwpcaepnpuicgvoxdqfcaabhsxgnorbpkboueplrpztqehrufldndxuenmvynranzqontfihfwvtxmiadbogsawlpmyycaztkyrqxwoxocbtngrmaiyafqqornrknnsmulglzyjnydvwpfefbuwhcocvyczklwywgwergmboqagcsngavorldbpsefoaynkmfqrjzdtbylibwwhhuhfqkoorihjewpcpuxhjnbeiegpplsyrzzyxsnnslvbkslzhwqjzrijdyucnnoshkxbagqdzxwnarwbzivhjpzgvaqrtywqwdetcqtsxviebsnpeflvegyjgxaskwuesfaqdjfdvuklayyekvumbuciranyvymtwshowdxyxbtpuchskraonnwvvvbdehhjdqiluauesssredqoeypadjsahxjqrgxmguppwrydbsiuvhjxyvsseeisbymztpaxclvxsvypdnabnhcnlzelogrfcvvzpwrujkdvwvzlgdbdqgiruxygolkamiluvmfuoaslakmsaehubjxlcfvonmcnqenxwjgypqfedszrisegdtlwhxdsqeyhkynmtdyrjwwslwwtucbtuxxlkymelgqumhgyjnnjtqvaatmcfpbupfszdcuwgbyudffdpxszrqskmnpdqeauilbwnojibjfqwaalwnmevjbbzvptawuemsvlqfmonxycnfnzmgfmwpmjcyvhjfjpyhywjqbtjwhxudqhuclhyfbiuazzvnklkjfiupntduclftpddhfkioqkptklhynubibyjwllgexbjafcgubnelxzdapnxsqtmnqcdscrgpawigubbugtnxvebnrlujkxranwzbiemsvykjlmnyyfkrplutmpohohtqntxmkdfmlwvspedxlgylwbqfbhtolkfbfjfmvbhxafbwmlolagckjeffnwsiesgkhpyeupuyvcmfhlxygrmkkvuarjviqxfgkwturzkzwquefcyelduecwqrachowsxbdcgrvtkmlkyloqjjlyctuzsztafqearzuiogjrfqbseqibzddpabbfhbkeiorgslhecyicrvxvlzynxctnydzpgghjocuqimuxlvwymkhvbaqyljurieblfffxplmfwszrhvqiyzaqruihxvabytqvhmauzmrgcxtiotlkhtqcrugthgerkvixykexnlyxntpgagigdepzchsplujazcqabzvjcvutxriirzeydvffvzhhhihlfwewusmuzeecuytcwermdxpcffsewvyfearxirlmdwqfxsaljqnyvfiibqgxpfgkuoqadxcbdgcrhhrhvrdluzlefeerwhwinuqzuwuvairwqraajnifdihhzuqtciqmkrxafiuixbqacmgxfybmdghunmgwbidvanucokddovmznczycruztcayuwafcvjnqxhzarfvdqfounjciebxngwfwjknqmrappyuiewzdjshsbespzfcrwswtpayqaldwdvwiyoullwxshgzdluqbwecxkiyrqrguyepbcqzgafvjfxkzvglnghbmogenghozxncfenobraxqrhybxcoyovwreawnxiurnggbecnybohhicmvxlexizgvskafbxabzkzuvbrksbanikuewjgbkwcxoqibcpissudhljplgqifaebgxueiukmqxfcjwkysjjyuamlcatwqfhltyrwjdutbqoclvpcyhczybrmsatuwaswlziqqasyedtoxhmhktrjfwytfhxhncwiovjbpimtjpkwmxkalmxdjkwqhtakjraqiykrrmgncirghthsyfuhubmkhduibikgkorispskluycaepxdgxsodeqstbreyfuxtwksqqyqlvzvqtnoirjujqeltakrbiubuoqzyorkgcdkormvctirrowpemxtfgiqpzkadbpxaghodshuxorilbjxfigpcixuhcbaoulyxaoweiaojcbmbptwcfzgvylwcfyvmhoepvexcmkqvlrbknbhvctplytkqjjvmfadhksevedhfhmbhhqzgooudgxuazdskajgvuejhpvgngpfsayyyvupptpjvveuwrsrdwewhwivpmojlvvhnahpqwvrspjpradwkylbsketjriqqpvhhedbhkkvswcvwadunencyxtxmindabykaqoaoeifqdwihohbueikhixdfumrldazczdlorgtcwrrylwjxyxzgmdujwiapwpzrikupdeijmqrarvwhfcgzrkysnwsjeqhqtembrwkcjxtvjwwwkarolvbeczwinodvtgvbbqdwipckezgxwdizrnaxfexnjoafnrqxgywruytmfktkoygmbdwogjfjdvohglnxngnijysafyrlzofdovuslmcnhxfjjitvbdvwbluorpwtmtynrmzjrbiaqvzcqrxppojxnqfebtwpmfvuhigbahvjraidtmmceaxctnbtolhuiqcforksdbypcvjagrvsqguhgbyqgqiruvvqszezfmlntffbdrlcwyjcgriqycabijkokoqzntelifjxzyvytnbfflkoiipilzafyicgniewbuapmfzqcaximlfjhvuurfscozgpgghvsuvpqqtsojdphbumqrluvgnbhlymulcmqziytcampdpdrqvfwxygwbysofektetfrwedzklwgtjytibpdzakxjfvczcfxtdjpbngqzrmvyudslvlqsezwkqkkcbxoipifopzynllvbuujssspdsycqbyvdlgoauisrkvahwipcwukmvllgsaijkdrpuxlzyzsjabsatdgzmbebovnlydvqgjphwxqgfzaqyxadgfdmtbbqlpvciucjdrhlluumnqubughdaauyhkvdekpoamoliyttnyondnmphcesvowghyktattrxhvlbdlrsfjssflnvtyltawmuqoofhvvobkistzasjhmbkessjtditkmxpzlskojwokiatbwhdzjbzbsqkzmnzvdpyaxygrehqgfqqaxnnerizmarbooohufcdhwpvsulsstmcoivxapufjqtnaviffihkpyrbrzfjenqtxlxfqkfjvazubkrdqluffwaluvqilsfrqrdmnhdhrjkbpkgtkznoqnvpswoyndtqqckqqtcubtutrbxrforaitavuazzugvgczvsyfvqnuakykuwczsithogunmpejpmrxcjjvvzbxzyrltihlsirevhjohihonylwsekburnndcwxkybjifubalefhmiiitpqqzjjazsbfgtxopdjvbdgdmcfrfiebeopggbukfxxxcjotbiltihiddnuwrlzvrzsfmgkrulvcwbeqnkkhgobokkkspdzomfgvodqvzrvajlanwbyeioxtxbyfrapsjtgqvqzdoerwkfvzmsbzhumuotznpiozkvodscfaffpgiapftqlrthudvgvenfzyaxaummpmyhyuswvegysoppwnysdpqyzzrubdrenplyiqmmvthtfreynmltqbizpycniwwnunqynfyeigduomqqulemmdjqpndhuxvmizutfzxztrbavfopefglqyvrqwggjnrpnkcyghelmcnmtmtzsepopwzoirjcdjenpuqpvutueinapitkhsmmraytgftzpjlekyhcwdltjjxsycydcvfmzwgbcxcoboqcbxzkszwlhxgpnpynjjhaqucnwhxhoyhqrkeeqgggqliogjuqyxgzcfvhmtkevipopxmowjotswevzmsxngshovradrjxihwbnqqpuhobnmlfassydwmlcgirrhrlxhkchzwrokrllbibjecgqfpttvuerckuegvooxssxvdcemgbuuirzfckxnxwsszrpcuogfuusrplrielkgvhvygnjkfzfzxsqhifoxctfhuipqxctdlvgdkewprxfuxsgfvfqtclxgzsadtvhuvpqrxjjjjrrxqgovoyuipqnhosvoufbqvugpplwzrphrjrnrllxpoxtuwlwkzotpebuamennaapntnneyvhahgxdbhtbqplduxaodhsskkpqynrzonhagrwxcoeagawjvcucsvbotsaelkywjoyeehsbfdwgoqnyyaunvhqdjvtsbxbbbpgfxnngxmikrrgsdbsokwdpyevqoplxqudieqpisrsbfxugybymbjwzoyevpigikboxhdntktxmzvjzywtbvcjrhuhwosrijcqmkfjcqxbitntabzbroolnxjllqfnznkaystntwegkkofkxsyxuamsxpubavzwfoixtjlluujjwifajginhtcywidlmkkhysmulzvzldxwyopdodzryxpkqrdukecragplzfysuwqcqaoqqfjuyfqliwkmvwvfubcklvdhzutgtgumayojpvovumxemspycidbnnmcxyarkxttcvqqhomkilqaqoovnrqlvubeeofiwxjqyayzcycgoiircgxgzhnqrbcgkrofcotpkqouabcqbdrouxxageejbjlfuzmbmfckrqbbyyhzvjrrpzbefamvoufqogovzaokciuzmevfyuaaypyjqggwipkgzkbmlbafbzlljsfmhscsodcrbbfylkfyqxokfgolmosuxpdsjspfciammxqirioodfxngosyzlkltpmsxpafmabupevthayfxdbiovsqxkrenmfkoctidrdqinrqojnelptkuzrcebddkboyxkhrfwrgvikyrvjihcfitdhdtexfnxtcsgmxkntaimhvdrndqprsddtqctysmzounvmsvxxhwgoxegqjqtouwapoxuvbkmpriuanvqoxorjcpprofqdspoosgmmlnztpaubosqqknbbqnhpyyubonpkwmlowfjgpmtgzlqujxljdadtosbhnmngepkjlmvuescozvzclobocahnkyplhqstopknrhufneybwebebhgxvwyydolesgbojzvvndmsgbhxhoybgakgdesrsmvrzhmijyjytdmpdxehxrhgsyiedfrxlgvxpxwyjaxjowwnuujnabjqcylypgduzrwituqvwckkworwlkcluicfwgydzothrmcisvfrhgpnpecbhxisfzchndxftawqybqnfuxetoupeplgxatqxytnfmokteeqmxjemgwrufklkwaxazzgelzfgglhirzjffkoqxmbtwkhhbodamujvwljmlivdbbirtubccsgrvkylyfmppilsxezcdozjpbdfjtnuwgfsreodmpafkzclgtzrgvbewdbaoephgkmwwidfcvzjkvrcafkzfdxxxdpgbqrybzejaaezxrwpxbeziscdwfbfahiefzokbnhgfqxkbcuixenmmnvolepkzkpeasrygpyoummmsoticswegyqyxkaqscmrtsnoblraczloiqcznabchbatdicakdpulzkddhcagdeogibasyddubcezmztaukusexmsjymljapvllrqfhpgkfudyedlcbhwpfimlplclrcxpttnxamzucpfagisbajnyrzvxxlzjolxxisexrojzwcratimkamjliplucbdejtcgwryaweatwbkqardkbmrdbdseggojlslcjhmcumhxikgdzxjuyyjcyyvkvseyvqjewdlrrpnwxltnnedjupeclekzyvoormnowjkrpuwfavricrvjoqqbjqijpqlmsnwmonmbjxvbpokumfvdfnxbhxxsfzxcozwhzmgrwkdczrcigxbuvlnyyuvsraobwuznkxdizyqifjwgtwnmxunhukoeqpxafupdxwzfymwsblpvyyktdspctcxmwsxfqgsgedimmhcqjztpmrbavhogwuhzbngycbggsoakrvuceiditgcwxaopcirtivcibqpxbqdstcokmypyuzlaowongknpzitpdhjkoszoqvdgrtqvckyjmduketumefixobawbekqfrnwebsijgvihedwmegkssznkwgwqgavcakfhdwesqyjvotcevhgltzcknzqrindzbywgiibjsnvsttugwizaxayngnmfsmjopzqygphvxdhdwnrnksdqwnahecbzpxeetihclinvbxmrpoiynuxsyabdmhtaqnjtsiearotujspwgmnjoqgeqbcrbrmssjtbuqciflfjjfxkiapfttwymziapbxjenhwrzcdrjtjudhcztpwqungqidwzjhsgwlxlujrpeepubglthkbnhbdqwwlbjbnndfjupllxyluagyieeurajyhdqcinkmosympawgreeihdjcfggtomoeooqbklqfxoavtpcdhrlcsezabwufygrghnmtdvisfrzfjmmgnqwnxdyvidbxizsunzosikevlwzgrybbidzyytsaspbgbgzrmgimcmmulodqbqljxeusveynagjkphulpqnqkfrbjcvtntzmjwsfifmhsphicrtchdydlklhrdghmtacbjfktgvphhlhhkkcqkytcfgjuoblfdjpkuocherhufixafdbgotxrxrcuohgpxpogfggvkdrkixzfkahtnwhvbntqbrpqxvutbldhrfirzxfupybrvolteycfjkdcaubwtqomzfepcevmpecqpteevaubtbchlaakgqpzvjwqxzbcneektiwhvyoexdufbirqhukdmtlfqtjhboncqciumvxccncrjpecyuctxfdsekblmnpmjkotsfopoeakeetsvagoayijofejkjixevcvopuymjxdpgrjupgbpjpqacdbbcpzuqwaztmxfriypcfdybjamjxflzinuxcszriqnsokpegxzfzgidrjsbfftnzxgcxtbrordopldbrtxqeeeeiixdnedgoaohbadmrnstciefemzhepbfwccdulrgduxhebifzivhzgiueajetcrwqvmeailiyyjclgfeizotkkjiaedwsqsngsoqrpekysgzlmtijsxdrcpjchecchkxhjuyevtdlohohbgrkyfsmygplztmvrgeuqjuenepnsarcopkadsbvvpzmcacliqagsfvsfylfoinibat")
            ),
            true
        );
    }

    if *condition.get("q733").unwrap_or(&false) {
        println!("Question 733 - Flood Fill");
        assert_eq!(
            q733_flood_fill::Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1),
            vec![vec![], vec![]]
        )
    }

    if *condition.get("q695").unwrap_or(&false) {
        println!("Question 695 - Max Area of Island");
        assert_eq!(
            q695_max_area_of_island::Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        )
    }

    if *condition.get("q566").unwrap_or(&false) {
        println!("Question 566 - Reshape the matrix");
        assert_eq!(
            q566_matrix_reshape::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
        assert_eq!(
            q566_matrix_reshape::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
        assert_eq!(
            q566_matrix_reshape::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 4, 1),
            vec![vec![1], vec![2], vec![3], vec![4]]
        );
        assert_eq!(
            q566_matrix_reshape::Solution::matrix_reshape(vec![vec![1, 2]], 1, 1),
            vec![vec![1, 2]]
        );
        assert_eq!(
            q566_matrix_reshape::Solution::matrix_reshape(
                vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]],
                2,
                5
            ),
            vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]]
        );
    }

    if *condition.get("q994").unwrap_or(&false) {
        println!("Question 994 - Rotting Oragnes");
        assert_eq!(
            q994_oranges_rotting::Solution::oranges_rotting(vec![
                vec![2, 1, 1],
                vec![1, 1, 0],
                vec![0, 1, 1]
            ]),
            4
        );
        assert_eq!(
            q994_oranges_rotting::Solution::oranges_rotting(vec![
                vec![2, 1, 1],
                vec![0, 1, 1],
                vec![1, 0, 1]
            ]),
            -1
        );
        assert_eq!(
            q994_oranges_rotting::Solution::oranges_rotting(vec![vec![0, 2]]),
            0
        );
        assert_eq!(
            q994_oranges_rotting::Solution::oranges_rotting(vec![
                vec![2, 1, 1],
                vec![1, 1, 1],
                vec![0, 1, 2]
            ]),
            2
        );
    }

    if *condition.get("q542").unwrap_or(&false) {
        println!("Question 542 - Update Matrix");
        // Write test case here
        assert_eq!(
            q542_update_matrix::Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
        assert_eq!(
            q542_update_matrix::Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![1, 1, 1]
            ]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
        assert_eq!(
            q542_update_matrix::Solution::update_matrix(vec![
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![1, 1, 1]
            ]),
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 2, 2]]
        );
        assert_eq!(
            q542_update_matrix::Solution::update_matrix(vec![vec![0, 1], vec![1, 1]]),
            vec![vec![0, 1], vec![1, 2]]
        );
        assert_eq!(
            q542_update_matrix::Solution::update_matrix(vec![
                vec![1, 1, 0],
                vec![0, 1, 1],
                vec![1, 1, 1]
            ]),
            vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
        );

        assert_eq!(
            q542_update_matrix::Solution::update_matrix(vec![
                vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 1],
                vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
                vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1],
                vec![0, 0, 1, 1, 1, 0, 1, 1, 1, 1],
                vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
                vec![0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
                vec![1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0]
            ]),
            vec![
                vec![0, 0, 1, 0, 1, 2, 1, 0, 1, 2],
                vec![1, 1, 2, 1, 0, 1, 1, 1, 2, 3],
                vec![2, 1, 2, 1, 1, 0, 0, 0, 1, 2],
                vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 2],
                vec![0, 0, 1, 1, 1, 0, 1, 1, 2, 3],
                vec![1, 0, 1, 2, 1, 1, 1, 2, 1, 2],
                vec![1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
                vec![0, 1, 0, 0, 0, 1, 0, 0, 1, 2],
                vec![1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1, 2, 1, 0]
            ]
        )
    }

    if *condition.get("q139").unwrap_or(&false) {
        println!("Question 139 - Word Break");
        assert_eq!(
            q139_word_break::Solution::word_break(
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")]
            ),
            true
        );
        assert_eq!(
            q139_word_break::Solution::word_break(
                String::from("applepenapple"),
                vec![
                    String::from("apple"),
                    String::from("pen"),
                    String::from("apple")
                ]
            ),
            true
        );
        assert_eq!(
            q139_word_break::Solution::word_break(
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            ),
            false
        );
        assert_eq!(
            q139_word_break::Solution::word_break(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
            vec![String::from("a"),String::from("aa"),String::from("aaa"),String::from("aaaa"),String::from("aaaaa"),String::from("aaaaaa"),String::from("aaaaaaa"),String::from("aaaaaaaa"),String::from("aaaaaaaaa"),String::from("aaaaaaaaaa")]),
        false);
    }

    if *condition.get("q322").unwrap_or(&false) {
        println!("Question 322 - Coin Change");
        assert_eq!(
            q322_coin_change::Solution::coin_change(vec![1, 2, 5], 11),
            3
        );
    }

    if *condition.get("q921").unwrap_or(&false) {
        println!("Question 921 - Minimum Add to Make Parentheses Valid");
        assert_eq!(
            q921_min_add_to_make_valid::Solution::min_add_to_make_valid(String::from("()))(()")),
            3
        );
        assert_eq!(
            q921_min_add_to_make_valid::Solution::min_add_to_make_valid(String::from(")))")),
            3
        );
        assert_eq!(
            q921_min_add_to_make_valid::Solution::min_add_to_make_valid(String::from("()")),
            0
        );
    }

    if *condition.get("q56").unwrap_or(&false) {
        println!("Question 56 - Merge Intervals");
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![vec![1, 4], vec![2, 5], vec![7, 9]]),
            vec![vec![1, 5], vec![7, 9]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![vec![1, 2], vec![2, 5], vec![5, 9]]),
            vec![vec![1, 9]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![vec![1, 1], vec![2, 5],]),
            vec![vec![1, 1], vec![2, 5]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![
                vec![1, 3],
                vec![2, 6],
                vec![8, 10],
                vec![15, 18]
            ]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![vec![1, 4], vec![0, 4]]),
            vec![vec![0, 4]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![vec![1, 4], vec![0, 0]]),
            vec![vec![0, 0], vec![1, 4]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![
                vec![2, 3],
                vec![4, 6],
                vec![6, 7],
                vec![8, 9],
                vec![1, 10]
            ]),
            vec![vec![1, 10]]
        );
        assert_eq!(
            q56_merge_intervals::Solution::merge(vec![
                vec![2, 3],
                vec![2, 2],
                vec![3, 3],
                vec![1, 3],
                vec![5, 7],
                vec![2, 2],
                vec![4, 6]
            ]),
            vec![vec![1, 3], vec![4, 7]]
        )
    }

    if *condition.get("q57").unwrap_or(&false) {
        println!("Question 57 - Insert Intervals");
        assert_eq!(
            q57_insert_intervals::Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            q57_insert_intervals::Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
        assert_eq!(
            q57_insert_intervals::Solution::insert(vec![], vec![5, 7]),
            vec![vec![5, 7]]
        );
        assert_eq!(
            q57_insert_intervals::Solution::insert(vec![vec![1, 1]], vec![5, 7]),
            vec![vec![1, 1], vec![5, 7]]
        );
        assert_eq!(
            q57_insert_intervals::Solution::insert(vec![vec![1, 1], vec![2, 6]], vec![5, 7]),
            vec![vec![1, 1], vec![2, 7]]
        );
    }

    if *condition.get("q973").unwrap_or(&false) {
        println!("Question 973 - K Closest Points to Origin");
        assert_eq!(
            q973_k_closest::Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
        assert_eq!(
            q973_k_closest::Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
        assert_eq!(
            q973_k_closest::Solution::k_closest(
                vec![vec![6, 10], vec![-3, 3], vec![-2, 5], vec![0, 2]],
                3
            ),
            vec![vec![0, 2], vec![-3, 3], vec![-2, 5]]
        );
        assert_eq!(
            q973_k_closest::Solution::k_closest(
                vec![
                    vec![-95, 76],
                    vec![17, 7],
                    vec![-55, -58],
                    vec![53, 20],
                    vec![-69, -8],
                    vec![-57, 87],
                    vec![-2, -42],
                    vec![-10, -87],
                    vec![-36, -57],
                    vec![97, -39],
                    vec![97, 49]
                ],
                5
            ),
            vec![
                vec![17, 7],
                vec![-2, -42],
                vec![53, 20],
                vec![-36, -57],
                vec![-69, -8]
            ]
        );
    }

    if *condition.get("q383").unwrap_or(&false) {
        println!("Question 383 - Ransom Note");
        assert_eq!(
            q383_can_construct::Solution::can_construct(
                String::from("bg"),
                String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj")
            ),
            true
        )
    }

    if *condition.get("q15").unwrap_or(&false) {
        println!("Question 15 - 3Sum");
        assert_eq!(
            q15_three_sum::Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            q15_three_sum::Solution::three_sum(vec![1, 2, -2, 1]),
            vec![vec![]]
        );
    }

    if *condition.get("q150").unwrap_or(&false) {
        println!("Question 150 - Evaluate Reverse Polish Notation");
        assert_eq!(
            q150_eval_rpn::Solution::eval_rpn(vec![
                "1".to_string(),
                "2".to_string(),
                "+".to_string()
            ]),
            3
        );
        assert_eq!(
            q150_eval_rpn::Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
        assert_eq!(
            q150_eval_rpn::Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "/".to_string()
            ]),
            1
        );
        assert_eq!(
            q150_eval_rpn::Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
        assert_eq!(
            q150_eval_rpn::Solution::eval_rpn(vec![
                "-1".to_string(),
                "1".to_string(),
                "*".to_string(),
                "-1".to_string(),
                "+".to_string()
            ]),
            -2
        );
        assert_eq!(
            q150_eval_rpn::Solution::eval_rpn(vec![
                "4".to_string(),
                "-2".to_string(),
                "/".to_string(),
                "2".to_string(),
                "-3".to_string(),
                "-".to_string(),
                "-".to_string()
            ]),
            -7
        );
    }

    if *condition.get("q207").unwrap_or(&false) {
        println!("Question 207 - Course Schedule");
        assert_eq!(
            q207_can_finish::Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]),
            false
        );
        assert_eq!(
            q207_can_finish::Solution::can_finish(2, vec![vec![1, 0]]),
            true
        );
    }

    if *condition.get("q238").unwrap_or(&false) {
        println!("Question 238 - Product of Array Except Self");

        assert_eq!(
            q238_product_except_self::Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        )
    }

    if *condition.get("q200").unwrap_or(&false) {
        println!("Question 200 - Number of Islands");

        assert_eq!(
            q200_number_of_islands::DFSSolution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            q200_number_of_islands::BFSSolution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
        );

        assert_eq!(
            q200_number_of_islands::DFSSolution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '0', '1'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            q200_number_of_islands::BFSSolution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '0', '1'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
        )
    }

    if *condition.get("q33").unwrap_or(&false) {
        println!("Question 33 - Search in Rotated Sorted Array");

        assert_eq!(
            q33_search::Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0),
            4
        )
    }

    if *condition.get("q417").unwrap_or(&false) {
        println!("Question 417 - Pacific Alantic Water Flow");

        assert_eq!(
            q417_pacific_alantic::Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]),
            vec![[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        )
    }

    if *condition.get("q90").unwrap_or(&false) {
        println!("Question 90 = Subsets II");
        assert_eq!(
            q90_subsets_with_duplicate::Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 2, 2]
            ]
        );
    }

    if *condition.get("q46").unwrap_or(&false) {
        println!("Question 46 - Permutations");

        assert_eq!(
            q46_permutation::Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    if *condition.get("q47").unwrap_or(&false) {
        println!("Question 47 - Permutations II (contains duplicate)");

        assert_eq!(
            q47_permutation_unique::Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }

    if *condition.get("q78").unwrap_or(&false) {
        println!("Question 78 = Subsets");
        assert_eq!(
            q78_subsets::Solution::subsets(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
        assert_eq!(
            q78_subsets::Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        )
    }

    if *condition.get("q39").unwrap_or(&false) {
        println!("Question 39 - Combination Sum");
        assert_eq!(
            q39_combination_sum::Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(
            q39_combination_sum::Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        )
    }

    if *condition.get("q1334").unwrap_or(&false) {
        println!("Question 1334 - Find the City With the Smallest Number of Neighbors at a Threshold Distance");

        assert_eq!(
            q1334_find_the_city::Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        );

        assert_eq!(
            q1334_find_the_city::Solution::find_the_city(
                6,
                vec![
                    vec![0, 3, 7],
                    vec![2, 4, 1],
                    vec![0, 1, 5],
                    vec![2, 3, 10],
                    vec![1, 3, 6],
                    vec![1, 2, 1]
                ],
                417
            ),
            5
        );
    }

    if *condition.get("q1192").unwrap_or(&false) {
        println!("Question 1192 - Critical Connections in a Network");

        assert_eq!(
            q1192_critical_connection::Solution::critical_connections(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]],
            ),
            vec![vec![1, 3]]
        );
        assert_eq!(
            q1192_critical_connection::Solution::critical_connections(
                5,
                vec![
                    vec![1, 0],
                    vec![2, 0],
                    vec![3, 2],
                    vec![4, 2],
                    vec![4, 3],
                    vec![3, 0],
                    vec![4, 0]
                ]
            ),
            vec![vec![0, 1]]
        );
    }

    if *condition.get("q67").unwrap_or(&false) {
        println!("Question 67 - Add Binary");

        assert_eq!(
            q67_add_binary::Solution::add_binary(String::from("11"), String::from("1")),
            String::from("100")
        )
    }

    if *condition.get("q153").unwrap_or(&false) {
        println!("Question 153 - Find Minimum in Rotated Sorted Array");

        assert_eq!(q153_find_min::Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    if *condition.get("q209").unwrap_or(&false) {
        println!("Question 209 - Minimuim Size Subarray Sum");

        assert_eq!(
            q209_min_sub_array_len::Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]),
            2
        );

        assert_eq!(
            q209_min_sub_array_len::Solution::min_sub_array_len(4, vec![1, 4, 4]),
            1
        );

        assert_eq!(
            q209_min_sub_array_len::Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]),
            3
        );
    }

    if *condition.get("q76").unwrap_or(&false) {
        println!("Question 76 - Minimum Window Substring");

        assert_eq!(
            q76_minimum_window_substring::Solution::min_window(
                String::from("ADOBECODEBANC"),
                String::from("ABC")
            ),
            String::from("BANC")
        );
        assert_eq!(
            q76_minimum_window_substring::Solution::min_window(
                String::from("ADOBECODEBANC"),
                String::from("ECO")
            ),
            String::from("ECO")
        );
        assert_eq!(
            q76_minimum_window_substring::Solution::min_window(
                String::from("a"),
                String::from("a")
            ),
            String::from("a")
        );
        assert_eq!(
            q76_minimum_window_substring::Solution::min_window(
                String::from("b"),
                String::from("aa")
            ),
            String::from("")
        );
    }

    if *condition.get("q239").unwrap_or(&false) {
        println!("Question 239 - Sliding Window Maximum");

        assert_eq!(
            q239_sliding_window_maximum::Solution::max_sliding_window(
                vec![1, 3, -1, -3, 5, 3, 6, 7],
                3
            ),
            vec![3, 3, 5, 5, 6, 7]
        );

        assert_eq!(
            q239_sliding_window_maximum::Solution::max_sliding_window(
                vec![1, 3, -1, -3, 5, 6, 6, 7],
                3
            ),
            vec![3, 3, 5, 6, 6, 7]
        );

        assert_eq!(
            q239_sliding_window_maximum::Solution::max_sliding_window(vec![1, 2, 4, 5, 2, 1, 3], 3),
            vec![4, 5, 5, 5, 3]
        );

        assert_eq!(
            q239_sliding_window_maximum::Solution::max_sliding_window(
                vec![1, 2, 4, 5, 7, 2, 4, 6, 1, -10, 2, 4, -3, -4, 5, 2, 1, 3],
                3
            ),
            vec![4, 5, 7, 7, 7, 6, 6, 6, 2, 4, 4, 4, 5, 5, 5, 3]
        );

        assert_eq!(
            q239_sliding_window_maximum::Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
            vec![3, 3, 2, 5]
        );
    }

    if *condition.get("q862").unwrap_or(&false) {
        println!("Question 862 - Shortest Subarray with Sum at Least K");

        assert_eq!(
            q862_shortest_subarray_with_sum_at_least_k::Solution::shortest_subarray(
                vec![84, -37, 32, 40, 95],
                167
            ),
            3
        );
        assert_eq!(
            q862_shortest_subarray_with_sum_at_least_k::Solution::shortest_subarray(
                vec![84, 100, -37, 32, 40, 95],
                167
            ),
            2
        );
        assert_eq!(
            q862_shortest_subarray_with_sum_at_least_k::Solution::shortest_subarray(
                vec![2, -1, 2, 3, 1, 2, 5, 6, 1, 2, 6, 8, 10],
                10
            ),
            1
        );
        assert_eq!(
            q862_shortest_subarray_with_sum_at_least_k::Solution::shortest_subarray(
                vec![31, 63, -38, 43, 65, 74, 90, -23, 45, 22],
                341
            ),
            9
        );
        assert_eq!(
            q862_shortest_subarray_with_sum_at_least_k::Solution::shortest_subarray(
                vec![-28, 81, -20, 28, -29],
                89
            ),
            3
        );

        assert_eq!(
            q862_shortest_subarray_with_sum_at_least_k::Solution::shortest_subarray(
                vec![-1000000; 1000],
                10000000
            ),
            -1
        );
    }

    if *condition.get("q496").unwrap_or(&false) {
        println!("Question 496 - Next Greater Element I");

        assert_eq!(
            q496_next_greater_element_I::Solution::next_greater_element(
                vec![4, 1, 2],
                vec![1, 3, 4, 2]
            ),
            vec![-1, 3, -1]
        );

        assert_eq!(
            q496_next_greater_element_I::Solution::next_greater_element(
                vec![1, 3, 4, 2],
                vec![6, 5, 4, 3, 2, 1, 7]
            ),
            vec![7, 7, 7, 7]
        );
    }

    if *condition.get("q84").unwrap_or(&false) {
        println!("Question 84 - Largest Rectangle in Histogram");

        assert_eq!(
            q84_largest_retangle_area::Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
            10
        );

        assert_eq!(
            q84_largest_retangle_area::Solution::largest_rectangle_area(vec![2, 4, 2, 1]),
            6
        );

        assert_eq!(
            q84_largest_retangle_area::Solution::largest_rectangle_area(vec![2, 4, 2]),
            6
        );

        assert_eq!(
            q84_largest_retangle_area::Solution::largest_rectangle_area(vec![2, 4]),
            4
        );

        assert_eq!(
            q84_largest_retangle_area::Solution::largest_rectangle_area(vec![
                10, 1, 9, 2, 8, 3, 7, 11
            ]),
            14
        );

        assert_eq!(
            q84_largest_retangle_area::Solution::largest_rectangle_area(vec![2, 1, 2]),
            3
        );
    }

    if *condition.get("q304").unwrap_or(&false) {
        let obj = q304_range_sum_query_2D_immutable::NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        // assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
        assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
        assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
    }

    if *condition.get("q42").unwrap_or(&false) {
        println!("Question 42 - Trapping Rain Water");
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![3, 2, 1, 2, 1, 3]),
            6
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![1, 0, 0, 0, 0, 0, 1]),
            5
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![4, 2, 0, 3, 2, 5]),
            9
        );

        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![4, 2, 0, 3, 2, 5]),
            9
        );

        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![2, 0, 0, 1, 3]),
            5
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![3, 0, 0, 1, 2]),
            5
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![3, 0, 0, 1, 3]),
            8
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0]),
            0
        );
        assert_eq!(
            q42_trapping_rain_water::Solution::trap(vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5]),
            25
        );

        assert_eq!(
            q42_trapping_rain_water::Solution::trap_using_two_pointers(vec![
                5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5
            ]),
            25
        );
    }

    if *condition.get("q407").unwrap_or(&false) {
        println!("Question 407 - Trapping Rain Water II");
        assert_eq!(
            q407_trapping_rain_water_II::Solution::trap_rain_water(vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 2, 1, 2, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 3, 3, 3, 3]
            ]),
            10
        );
        assert_eq!(
            q407_trapping_rain_water_II::Solution::trap_rain_water(vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1]
            ]),
            4
        );

        assert_eq!(
            q407_trapping_rain_water_II::Solution::trap_rain_water(vec![
                vec![12, 13, 1, 12],
                vec![13, 4, 13, 12],
                vec![13, 8, 10, 12],
                vec![12, 13, 12, 12],
                vec![13, 13, 13, 13]
            ]),
            14
        );

        assert_eq!(
            q407_trapping_rain_water_II::Solution::trap_rain_water(vec![
                vec![5, 5, 5, 1],
                vec![5, 1, 1, 5],
                vec![5, 1, 5, 5],
                vec![5, 2, 5, 8]
            ]),
            3
        );

        assert_eq!(
            q407_trapping_rain_water_II::Solution::trap_rain_water(vec![
                vec![9, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9],
                vec![9, 0, 9, 0, 0, 0, 1, 0, 0, 0, 9],
                vec![9, 7, 0, 0, 0, 0, 0, 0, 0, 0, 9],
                vec![9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
                vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
            ]),
            200
        );

        assert_eq!(
            q407_trapping_rain_water_II::Solution::trap_rain_water(vec![
                vec![19383, 10886, 12777, 16915, 17793, 18335, 15386, 10492, 16649, 11421],
                vec![12362, 27, 8690, 59, 7763, 3926, 540, 3426, 9172, 5736],
                vec![15211, 5368, 2567, 6429, 5782, 1530, 2862, 5123, 4067, 3135],
                vec![13929, 9802, 4022, 3058, 3069, 8167, 1393, 8456, 5011, 8042],
                vec![16229, 7373, 4421, 4919, 3784, 8537, 5198, 4324, 8315, 4370],
                vec![16413, 3526, 6091, 8980, 9956, 1873, 6862, 9170, 6996, 7281],
                vec![12305, 925, 7084, 6327, 336, 6505, 846, 1729, 1313, 5857],
                vec![16124, 3895, 9582, 545, 8814, 3367, 5434, 364, 4043, 3750],
                vec![11087, 6808, 7276, 7178, 5788, 3584, 5403, 2651, 2754, 2399],
                vec![19932, 5060, 9676, 3368, 7739, 12, 6226, 8586, 8094, 7539]
            ]),
            79058
        );
    }

    if *condition.get("q844").unwrap_or(&false) {
        println!("Question 844 - Backspace string compare");
        assert_eq!(
            q844_backspace_string_compare::Solution::backspace_compare(
                String::from("ab#c"),
                String::from("ad#c"),
            ),
            true
        );

        assert_eq!(
            q844_backspace_string_compare::Solution::backspace_compare(
                String::from("ab##"),
                String::from("c#d#"),
            ),
            true
        );

        assert_eq!(
            q844_backspace_string_compare::Solution::backspace_compare(
                String::from("a#c"),
                String::from("b"),
            ),
            false
        );
    }

    if *condition.get("q986").unwrap_or(&false) {
        println!("Question 986 - Interval List Intersections");
        assert_eq!(
            q986_interval_list_intersections::Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
            ),
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ]
        )
    }

    if *condition.get("q11").unwrap_or(&false) {
        println!("Question 11 - Container With Most Water");

        assert_eq!(
            q11_container_with_most_water::Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7],),
            49
        );
    }

    if *condition.get("q438").unwrap_or(&false) {
        println!("Question 438 - Find All Anagrams in a String");

        assert_eq!(
            q438_find_all_anagrams_in_a_string::Solution::find_anagrams(
                String::from("cbaebabacd"),
                String::from("abc")
            ),
            vec![0, 6]
        );

        assert_eq!(
            q438_find_all_anagrams_in_a_string::Solution::find_anagrams(
                String::from("abab"),
                String::from("ab")
            ),
            vec![0, 1, 2]
        );

        assert_eq!(
            q438_find_all_anagrams_in_a_string::Solution::find_anagrams(
                String::from("baa"),
                String::from("aa")
            ),
            vec![1]
        );

        assert_eq!(
            q438_find_all_anagrams_in_a_string::Solution::find_anagrams(
                String::from("abaacbabc"),
                String::from("abc")
            ),
            vec![3, 4, 6]
        );
    }

    if *condition.get("q713").unwrap_or(&false) {
        println!("Question 713 - Subarray Product Smaller than K");
        assert_eq!(
            q713_subarray_product_smaller_than_k::Solution::num_subarray_product_less_than_k(
                vec![10, 5, 2, 5],
                100
            ),
            8
        );
        assert_eq!(
            q713_subarray_product_smaller_than_k::Solution::num_subarray_product_less_than_k(
                vec![1, 2, 3],
                0
            ),
            0
        );
        assert_eq!(
            q713_subarray_product_smaller_than_k::Solution::num_subarray_product_less_than_k(
                vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3],
                19
            ),
            18
        );
    }

    if *condition.get("q547").unwrap_or(&false) {
        println!("Question 547 - Number of Provinces");
        assert_eq!(
            q547_number_of_provinces::Solution::find_circle_num(vec![
                vec![1, 1, 1],
                vec![1, 1, 0],
                vec![1, 0, 1]
            ]),
            1
        );

        assert_eq!(
            q547_number_of_provinces::Solution::find_circle_num(vec![
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            ]),
            8
        );

        assert_eq!(
            q547_number_of_provinces::Solution::find_circle_num(vec![
                vec![1, 1, 1],
                vec![1, 1, 1],
                vec![1, 1, 1]
            ]),
            1
        )
    }

    if *condition.get("q54").unwrap_or(&false) {
        println!("Question 54 - Spiral Matrix");
        assert_eq!(
            q54_spiral_matrix::Solution::spiral_order(vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        assert_eq!(
            q54_spiral_matrix::Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16]
            ]),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );

        assert_eq!(
            q54_spiral_matrix::Solution::spiral_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 4, 3]
        );

        assert_eq!(
            q54_spiral_matrix::Solution::spiral_order(vec![vec![1], vec![2]]),
            vec![1, 2]
        );
    }

    if *condition.get("q2335").unwrap_or(&false) {
        println!("Question 2335 - Fill Cups");

        assert_eq!(
            q2335_fill_cups::Solution::fill_cups(vec![100, 101, 200]),
            201
        );
    }

    if *condition.get("q6").unwrap_or(&false) {
        println!("Question 6 - ZigZag Conversion");

        assert_eq!(
            q6_zigzag_conversion::Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
    }
}
