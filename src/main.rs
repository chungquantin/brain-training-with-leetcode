// use std::borrow::Borrow;
use std::{collections::HashMap, vec};

#[path = "./ds/linked_list.rs"]
mod linked_list;
mod others;
mod q1;
mod q1143;
mod q118;
mod q121;
mod q139;
mod q141;
mod q15;
mod q150;
mod q189;
mod q1963;
mod q208;
mod q217;
mod q219;
mod q220;
mod q232;
mod q283;
mod q3;
mod q303;
mod q322;
mod q344;
mod q35;
mod q350;
mod q383;
mod q53;
mod q542;
mod q56;
mod q566;
mod q567;
mod q57;
mod q58;
mod q695;
mod q70;
mod q733;
mod q88;
mod q921;
mod q973;
mod q977;
mod q994;

fn main() {
    let mut condition = HashMap::new();
    condition.insert("q1", false);
    condition.insert("q3", false);
    condition.insert("q15", false);
    condition.insert("q35", false);
    condition.insert("q53", false);
    condition.insert("q56", false);
    condition.insert("q57", false);
    condition.insert("q58", false);
    condition.insert("q70", false);
    condition.insert("q88", false);
    condition.insert("q118", false);
    condition.insert("q121", false);
    condition.insert("q139", false);
    condition.insert("q141", false);
    condition.insert("q150", false);
    condition.insert("q217", false);
    condition.insert("q219", false);
    condition.insert("q220", false);
    condition.insert("q283", false);
    condition.insert("q303", false);
    condition.insert("q322", false);
    condition.insert("q350", false);
    condition.insert("q383", false);
    condition.insert("q542", false);
    condition.insert("q566", false);
    condition.insert("q567", false);
    condition.insert("q695", false);
    condition.insert("q733", false);
    condition.insert("q921", false);
    condition.insert("q994", false);
    condition.insert("q973", false);
    condition.insert("q977", false);

    if *condition.get("q1").unwrap_or(&false) {
        assert_eq!(q1::Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(q1::Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

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
        let linked_list = &mut q141::ListNode::default();
        linked_list.init(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(q141::Solution::has_cycle(&linked_list), false)
    }

    if *condition.get("q203").unwrap_or(&false) {
        println!("Question 203 - Remove elements from linked list");
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

    if *condition.get("q3").unwrap_or(&false) {
        println!("Question 3 - Longest Substring Without Repeating Characters");
        assert_eq!(
            q3::Solution::length_of_longest_substring(String::from("dvdf")),
            3
        );
        assert_eq!(
            q3::Solution::length_of_longest_substring(String::from("pwfwge")),
            4
        );
        assert_eq!(
            q3::Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(
            q3::Solution::length_of_longest_substring(String::from("abcdabcde")),
            5
        );
        assert_eq!(
            q3::Solution::length_of_longest_substring(String::from("abfklameqsadonxzvjbasd")),
            13
        );
    }

    if *condition.get("q58").unwrap_or(&false) {
        println!("Question 58 - Length of last word");
        assert_eq!(
            q58::Solution::length_of_last_word(String::from("hello world")),
            "world".len() as i32
        );

        assert_eq!(
            q58::Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            "moon".len() as i32
        )
    }

    if *condition.get("q88").unwrap_or(&false) {
        println!("Question 88 - Merged Sorted Array");
        assert_eq!(
            q88::Solution::merge(&mut vec![1, 2, 3], 3, &mut vec![2, 5, 6], 3),
            vec![1, 2, 2, 3, 5, 6]
        );
        assert_eq!(
            q88::Solution::merge(&mut vec![1], 1, &mut vec![], 0),
            vec![1]
        );
        assert_eq!(
            q88::Solution::merge(&mut vec![1], 1, &mut vec![1, 0], 1),
            vec![1, 1]
        );
        assert_eq!(
            q88::Solution::merge(
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
            q350::Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            q350::Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }

    if *condition.get("q35").unwrap_or(&false) {
        println!("Question 35 - Search Insert Position");
        assert_eq!(q35::Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(q35::Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(q35::Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(q35::Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }

    if *condition.get("q977").unwrap_or(&false) {
        println!("Question 977 - Squares of Sorted Array");
        assert_eq!(
            q977::Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        )
    }

    if *condition.get("q118").unwrap_or(&false) {
        println!("Question 118 - Pascal's Triangle");
        assert_eq!(
            q118::Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );

        assert_eq!(q118::Solution::generate(4), vec![vec![1]]);
    }

    if *condition.get("q283").unwrap_or(&false) {
        println!("Question 283 - Move Zeros");
        let mut arr1 = vec![0, 1, 0, 3, 12];
        q283::Solution::move_zeroes(&mut arr1);
        assert_eq!(arr1, vec![1, 3, 12, 0, 0]);
    }

    if *condition.get("q567").unwrap_or(&false) {
        println!("Question 567 - Permutation in String");
        assert_eq!(
            q567::Solution::check_inclusion(String::from("ab"), String::from("eidbaooo")),
            true
        );
        assert_eq!(
            q567::Solution::check_inclusion(String::from("ab"), String::from("eidboaoo")),
            false
        );
        assert_eq!(
            q567::Solution::check_inclusion(String::from("adc"), String::from("dcda")),
            true
        );
        assert_eq!(
            q567::Solution::check_inclusion(String::from("abc"), String::from("aaaacccccbbbb")),
            false
        );
        assert_eq!(
            q567::Solution::check_inclusion(String::from("ab"), String::from("a")),
            false
        );

        assert_eq!(
            q567::Solution::check_inclusion(String::from("park"), String::from("spake")),
            false
        );
        assert_eq!(
            q567::Solution::check_inclusion(String::from("r"), String::from("pilmtnzraxj")),
            true
        );

        assert_eq!(
            q567::Solution::check_inclusion(
                String::from("vxqakfyaqahufxfizupjrkkifjlbfqfeprqrfjvxnubntdtlvz"),
                String::from("oumgfmlrbivgnrvsfslnheghnbhhicbvaddqadwicekguhjairhpqtebqvzyxdfodntxmoqtffgmsomnhndbrffxmuyjvqazwfvugyvmshxignfenmkihorjkshwyuxxkxidpkalqmdnxxmnovhangcwggjwjrletxhelehcipflvqyueptgjxugyipegamjbweqdfeswrjepjjlviuhfikfaojbhrujdfpnenrvajrdplwaevpbzkcdkyhidbgizwofjoxfvnkzhmwvegnfipgmnikljmmcrleffceqsxrxgsjmjmaflnxtigfoeaafsrjaxaqumhatpdeueridyxfjajsjkcrfwkwguclqtrefwevwalmcvoiragprwwnxpwgqoyulsadfhstduevikkkwmofklomdvpdcnxxsotudmkjgakbzopzengbfrvhiikzrsmmurewnnquydhntpqprtyvouatjevvoerbnzunwtdiigohcctulnvpgfbtdjmjsvszphulflmolunohifxsycaiifkngpuycnlihebmhaapureljxlqozydijsbcitfrdjhbdppvhpfjfuxujwfkcezyvhbityajxcccbuyvoluzndgjsyvrbluvagqtuujpsxjcgdtkbhaxsatmellmotdeoxsxjsywuavjqpzjkcpsrowqgogovcouosjhblhdmbkeidafpimjaiypownlcilpahbszmmcwbqmztpdmbqeuizjwdifruojquahdtdukydxgehpourjytveajjvtpiphihkmgqpmnuukpqtjyyqhfsiezugszdbjsseaxxdxomhouvetgcvfeuvwszmhgpkkorubvahxoufpfcukgchdubddpijsolrkvgiizhqefdhvpzvpmuxnwpcaepnpuicgvoxdqfcaabhsxgnorbpkboueplrpztqehrufldndxuenmvynranzqontfihfwvtxmiadbogsawlpmyycaztkyrqxwoxocbtngrmaiyafqqornrknnsmulglzyjnydvwpfefbuwhcocvyczklwywgwergmboqagcsngavorldbpsefoaynkmfqrjzdtbylibwwhhuhfqkoorihjewpcpuxhjnbeiegpplsyrzzyxsnnslvbkslzhwqjzrijdyucnnoshkxbagqdzxwnarwbzivhjpzgvaqrtywqwdetcqtsxviebsnpeflvegyjgxaskwuesfaqdjfdvuklayyekvumbuciranyvymtwshowdxyxbtpuchskraonnwvvvbdehhjdqiluauesssredqoeypadjsahxjqrgxmguppwrydbsiuvhjxyvsseeisbymztpaxclvxsvypdnabnhcnlzelogrfcvvzpwrujkdvwvzlgdbdqgiruxygolkamiluvmfuoaslakmsaehubjxlcfvonmcnqenxwjgypqfedszrisegdtlwhxdsqeyhkynmtdyrjwwslwwtucbtuxxlkymelgqumhgyjnnjtqvaatmcfpbupfszdcuwgbyudffdpxszrqskmnpdqeauilbwnojibjfqwaalwnmevjbbzvptawuemsvlqfmonxycnfnzmgfmwpmjcyvhjfjpyhywjqbtjwhxudqhuclhyfbiuazzvnklkjfiupntduclftpddhfkioqkptklhynubibyjwllgexbjafcgubnelxzdapnxsqtmnqcdscrgpawigubbugtnxvebnrlujkxranwzbiemsvykjlmnyyfkrplutmpohohtqntxmkdfmlwvspedxlgylwbqfbhtolkfbfjfmvbhxafbwmlolagckjeffnwsiesgkhpyeupuyvcmfhlxygrmkkvuarjviqxfgkwturzkzwquefcyelduecwqrachowsxbdcgrvtkmlkyloqjjlyctuzsztafqearzuiogjrfqbseqibzddpabbfhbkeiorgslhecyicrvxvlzynxctnydzpgghjocuqimuxlvwymkhvbaqyljurieblfffxplmfwszrhvqiyzaqruihxvabytqvhmauzmrgcxtiotlkhtqcrugthgerkvixykexnlyxntpgagigdepzchsplujazcqabzvjcvutxriirzeydvffvzhhhihlfwewusmuzeecuytcwermdxpcffsewvyfearxirlmdwqfxsaljqnyvfiibqgxpfgkuoqadxcbdgcrhhrhvrdluzlefeerwhwinuqzuwuvairwqraajnifdihhzuqtciqmkrxafiuixbqacmgxfybmdghunmgwbidvanucokddovmznczycruztcayuwafcvjnqxhzarfvdqfounjciebxngwfwjknqmrappyuiewzdjshsbespzfcrwswtpayqaldwdvwiyoullwxshgzdluqbwecxkiyrqrguyepbcqzgafvjfxkzvglnghbmogenghozxncfenobraxqrhybxcoyovwreawnxiurnggbecnybohhicmvxlexizgvskafbxabzkzuvbrksbanikuewjgbkwcxoqibcpissudhljplgqifaebgxueiukmqxfcjwkysjjyuamlcatwqfhltyrwjdutbqoclvpcyhczybrmsatuwaswlziqqasyedtoxhmhktrjfwytfhxhncwiovjbpimtjpkwmxkalmxdjkwqhtakjraqiykrrmgncirghthsyfuhubmkhduibikgkorispskluycaepxdgxsodeqstbreyfuxtwksqqyqlvzvqtnoirjujqeltakrbiubuoqzyorkgcdkormvctirrowpemxtfgiqpzkadbpxaghodshuxorilbjxfigpcixuhcbaoulyxaoweiaojcbmbptwcfzgvylwcfyvmhoepvexcmkqvlrbknbhvctplytkqjjvmfadhksevedhfhmbhhqzgooudgxuazdskajgvuejhpvgngpfsayyyvupptpjvveuwrsrdwewhwivpmojlvvhnahpqwvrspjpradwkylbsketjriqqpvhhedbhkkvswcvwadunencyxtxmindabykaqoaoeifqdwihohbueikhixdfumrldazczdlorgtcwrrylwjxyxzgmdujwiapwpzrikupdeijmqrarvwhfcgzrkysnwsjeqhqtembrwkcjxtvjwwwkarolvbeczwinodvtgvbbqdwipckezgxwdizrnaxfexnjoafnrqxgywruytmfktkoygmbdwogjfjdvohglnxngnijysafyrlzofdovuslmcnhxfjjitvbdvwbluorpwtmtynrmzjrbiaqvzcqrxppojxnqfebtwpmfvuhigbahvjraidtmmceaxctnbtolhuiqcforksdbypcvjagrvsqguhgbyqgqiruvvqszezfmlntffbdrlcwyjcgriqycabijkokoqzntelifjxzyvytnbfflkoiipilzafyicgniewbuapmfzqcaximlfjhvuurfscozgpgghvsuvpqqtsojdphbumqrluvgnbhlymulcmqziytcampdpdrqvfwxygwbysofektetfrwedzklwgtjytibpdzakxjfvczcfxtdjpbngqzrmvyudslvlqsezwkqkkcbxoipifopzynllvbuujssspdsycqbyvdlgoauisrkvahwipcwukmvllgsaijkdrpuxlzyzsjabsatdgzmbebovnlydvqgjphwxqgfzaqyxadgfdmtbbqlpvciucjdrhlluumnqubughdaauyhkvdekpoamoliyttnyondnmphcesvowghyktattrxhvlbdlrsfjssflnvtyltawmuqoofhvvobkistzasjhmbkessjtditkmxpzlskojwokiatbwhdzjbzbsqkzmnzvdpyaxygrehqgfqqaxnnerizmarbooohufcdhwpvsulsstmcoivxapufjqtnaviffihkpyrbrzfjenqtxlxfqkfjvazubkrdqluffwaluvqilsfrqrdmnhdhrjkbpkgtkznoqnvpswoyndtqqckqqtcubtutrbxrforaitavuazzugvgczvsyfvqnuakykuwczsithogunmpejpmrxcjjvvzbxzyrltihlsirevhjohihonylwsekburnndcwxkybjifubalefhmiiitpqqzjjazsbfgtxopdjvbdgdmcfrfiebeopggbukfxxxcjotbiltihiddnuwrlzvrzsfmgkrulvcwbeqnkkhgobokkkspdzomfgvodqvzrvajlanwbyeioxtxbyfrapsjtgqvqzdoerwkfvzmsbzhumuotznpiozkvodscfaffpgiapftqlrthudvgvenfzyaxaummpmyhyuswvegysoppwnysdpqyzzrubdrenplyiqmmvthtfreynmltqbizpycniwwnunqynfyeigduomqqulemmdjqpndhuxvmizutfzxztrbavfopefglqyvrqwggjnrpnkcyghelmcnmtmtzsepopwzoirjcdjenpuqpvutueinapitkhsmmraytgftzpjlekyhcwdltjjxsycydcvfmzwgbcxcoboqcbxzkszwlhxgpnpynjjhaqucnwhxhoyhqrkeeqgggqliogjuqyxgzcfvhmtkevipopxmowjotswevzmsxngshovradrjxihwbnqqpuhobnmlfassydwmlcgirrhrlxhkchzwrokrllbibjecgqfpttvuerckuegvooxssxvdcemgbuuirzfckxnxwsszrpcuogfuusrplrielkgvhvygnjkfzfzxsqhifoxctfhuipqxctdlvgdkewprxfuxsgfvfqtclxgzsadtvhuvpqrxjjjjrrxqgovoyuipqnhosvoufbqvugpplwzrphrjrnrllxpoxtuwlwkzotpebuamennaapntnneyvhahgxdbhtbqplduxaodhsskkpqynrzonhagrwxcoeagawjvcucsvbotsaelkywjoyeehsbfdwgoqnyyaunvhqdjvtsbxbbbpgfxnngxmikrrgsdbsokwdpyevqoplxqudieqpisrsbfxugybymbjwzoyevpigikboxhdntktxmzvjzywtbvcjrhuhwosrijcqmkfjcqxbitntabzbroolnxjllqfnznkaystntwegkkofkxsyxuamsxpubavzwfoixtjlluujjwifajginhtcywidlmkkhysmulzvzldxwyopdodzryxpkqrdukecragplzfysuwqcqaoqqfjuyfqliwkmvwvfubcklvdhzutgtgumayojpvovumxemspycidbnnmcxyarkxttcvqqhomkilqaqoovnrqlvubeeofiwxjqyayzcycgoiircgxgzhnqrbcgkrofcotpkqouabcqbdrouxxageejbjlfuzmbmfckrqbbyyhzvjrrpzbefamvoufqogovzaokciuzmevfyuaaypyjqggwipkgzkbmlbafbzlljsfmhscsodcrbbfylkfyqxokfgolmosuxpdsjspfciammxqirioodfxngosyzlkltpmsxpafmabupevthayfxdbiovsqxkrenmfkoctidrdqinrqojnelptkuzrcebddkboyxkhrfwrgvikyrvjihcfitdhdtexfnxtcsgmxkntaimhvdrndqprsddtqctysmzounvmsvxxhwgoxegqjqtouwapoxuvbkmpriuanvqoxorjcpprofqdspoosgmmlnztpaubosqqknbbqnhpyyubonpkwmlowfjgpmtgzlqujxljdadtosbhnmngepkjlmvuescozvzclobocahnkyplhqstopknrhufneybwebebhgxvwyydolesgbojzvvndmsgbhxhoybgakgdesrsmvrzhmijyjytdmpdxehxrhgsyiedfrxlgvxpxwyjaxjowwnuujnabjqcylypgduzrwituqvwckkworwlkcluicfwgydzothrmcisvfrhgpnpecbhxisfzchndxftawqybqnfuxetoupeplgxatqxytnfmokteeqmxjemgwrufklkwaxazzgelzfgglhirzjffkoqxmbtwkhhbodamujvwljmlivdbbirtubccsgrvkylyfmppilsxezcdozjpbdfjtnuwgfsreodmpafkzclgtzrgvbewdbaoephgkmwwidfcvzjkvrcafkzfdxxxdpgbqrybzejaaezxrwpxbeziscdwfbfahiefzokbnhgfqxkbcuixenmmnvolepkzkpeasrygpyoummmsoticswegyqyxkaqscmrtsnoblraczloiqcznabchbatdicakdpulzkddhcagdeogibasyddubcezmztaukusexmsjymljapvllrqfhpgkfudyedlcbhwpfimlplclrcxpttnxamzucpfagisbajnyrzvxxlzjolxxisexrojzwcratimkamjliplucbdejtcgwryaweatwbkqardkbmrdbdseggojlslcjhmcumhxikgdzxjuyyjcyyvkvseyvqjewdlrrpnwxltnnedjupeclekzyvoormnowjkrpuwfavricrvjoqqbjqijpqlmsnwmonmbjxvbpokumfvdfnxbhxxsfzxcozwhzmgrwkdczrcigxbuvlnyyuvsraobwuznkxdizyqifjwgtwnmxunhukoeqpxafupdxwzfymwsblpvyyktdspctcxmwsxfqgsgedimmhcqjztpmrbavhogwuhzbngycbggsoakrvuceiditgcwxaopcirtivcibqpxbqdstcokmypyuzlaowongknpzitpdhjkoszoqvdgrtqvckyjmduketumefixobawbekqfrnwebsijgvihedwmegkssznkwgwqgavcakfhdwesqyjvotcevhgltzcknzqrindzbywgiibjsnvsttugwizaxayngnmfsmjopzqygphvxdhdwnrnksdqwnahecbzpxeetihclinvbxmrpoiynuxsyabdmhtaqnjtsiearotujspwgmnjoqgeqbcrbrmssjtbuqciflfjjfxkiapfttwymziapbxjenhwrzcdrjtjudhcztpwqungqidwzjhsgwlxlujrpeepubglthkbnhbdqwwlbjbnndfjupllxyluagyieeurajyhdqcinkmosympawgreeihdjcfggtomoeooqbklqfxoavtpcdhrlcsezabwufygrghnmtdvisfrzfjmmgnqwnxdyvidbxizsunzosikevlwzgrybbidzyytsaspbgbgzrmgimcmmulodqbqljxeusveynagjkphulpqnqkfrbjcvtntzmjwsfifmhsphicrtchdydlklhrdghmtacbjfktgvphhlhhkkcqkytcfgjuoblfdjpkuocherhufixafdbgotxrxrcuohgpxpogfggvkdrkixzfkahtnwhvbntqbrpqxvutbldhrfirzxfupybrvolteycfjkdcaubwtqomzfepcevmpecqpteevaubtbchlaakgqpzvjwqxzbcneektiwhvyoexdufbirqhukdmtlfqtjhboncqciumvxccncrjpecyuctxfdsekblmnpmjkotsfopoeakeetsvagoayijofejkjixevcvopuymjxdpgrjupgbpjpqacdbbcpzuqwaztmxfriypcfdybjamjxflzinuxcszriqnsokpegxzfzgidrjsbfftnzxgcxtbrordopldbrtxqeeeeiixdnedgoaohbadmrnstciefemzhepbfwccdulrgduxhebifzivhzgiueajetcrwqvmeailiyyjclgfeizotkkjiaedwsqsngsoqrpekysgzlmtijsxdrcpjchecchkxhjuyevtdlohohbgrkyfsmygplztmvrgeuqjuenepnsarcopkadsbvvpzmcacliqagsfvsfylfoinibat")
            ),
            true
        );
    }

    if *condition.get("q733").unwrap_or(&false) {
        println!("Question 733 - Flood Fill");
        assert_eq!(
            q733::Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1),
            vec![vec![], vec![]]
        )
    }

    if *condition.get("q695").unwrap_or(&false) {
        println!("Question 695 - Max Area of Island");
        assert_eq!(
            q695::Solution::max_area_of_island(vec![
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
            q566::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
        assert_eq!(
            q566::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
        assert_eq!(
            q566::Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 4, 1),
            vec![vec![1], vec![2], vec![3], vec![4]]
        );
        assert_eq!(
            q566::Solution::matrix_reshape(vec![vec![1, 2]], 1, 1),
            vec![vec![1, 2]]
        );
        assert_eq!(
            q566::Solution::matrix_reshape(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]], 2, 5),
            vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]]
        );
    }

    if *condition.get("q994").unwrap_or(&false) {
        println!("Question 994 - Rotting Oragnes");
        assert_eq!(
            q994::Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
        assert_eq!(
            q994::Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
        assert_eq!(q994::Solution::oranges_rotting(vec![vec![0, 2]]), 0);
        assert_eq!(
            q994::Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 1], vec![0, 1, 2]]),
            2
        );
    }

    if *condition.get("q542").unwrap_or(&false) {
        println!("Question 542 - Update Matrix");
        // Write test case here
        assert_eq!(
            q542::Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
        assert_eq!(
            q542::Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
        assert_eq!(
            q542::Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 2, 2]]
        );
        assert_eq!(
            q542::Solution::update_matrix(vec![vec![0, 1], vec![1, 1]]),
            vec![vec![0, 1], vec![1, 2]]
        );
        assert_eq!(
            q542::Solution::update_matrix(vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 1, 1]]),
            vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
        );

        assert_eq!(
            q542::Solution::update_matrix(vec![
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
            q139::Solution::word_break(
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")]
            ),
            true
        );
        assert_eq!(
            q139::Solution::word_break(
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
            q139::Solution::word_break(
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
            q139::Solution::word_break(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
            vec![String::from("a"),String::from("aa"),String::from("aaa"),String::from("aaaa"),String::from("aaaaa"),String::from("aaaaaa"),String::from("aaaaaaa"),String::from("aaaaaaaa"),String::from("aaaaaaaaa"),String::from("aaaaaaaaaa")]),
        false);
    }

    if *condition.get("q322").unwrap_or(&false) {
        println!("Question 322 - Coin Change");
        assert_eq!(q322::Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    if *condition.get("q921").unwrap_or(&false) {
        println!("Question 921 - Minimum Add to Make Parentheses Valid");
        assert_eq!(
            q921::Solution::min_add_to_make_valid(String::from("()))(()")),
            3
        );
        assert_eq!(
            q921::Solution::min_add_to_make_valid(String::from(")))")),
            3
        );
        assert_eq!(q921::Solution::min_add_to_make_valid(String::from("()")), 0);
    }

    if *condition.get("q56").unwrap_or(&false) {
        println!("Question 56 - Merge Intervals");
        assert_eq!(
            q56::Solution::merge(vec![vec![1, 4], vec![2, 5], vec![7, 9]]),
            vec![vec![1, 5], vec![7, 9]]
        );
        assert_eq!(
            q56::Solution::merge(vec![vec![1, 2], vec![2, 5], vec![5, 9]]),
            vec![vec![1, 9]]
        );
        assert_eq!(
            q56::Solution::merge(vec![vec![1, 1], vec![2, 5],]),
            vec![vec![1, 1], vec![2, 5]]
        );
        assert_eq!(
            q56::Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            q56::Solution::merge(vec![vec![1, 4], vec![0, 4]]),
            vec![vec![0, 4]]
        );
        assert_eq!(
            q56::Solution::merge(vec![vec![1, 4], vec![0, 0]]),
            vec![vec![0, 0], vec![1, 4]]
        );
        assert_eq!(
            q56::Solution::merge(vec![
                vec![2, 3],
                vec![4, 6],
                vec![6, 7],
                vec![8, 9],
                vec![1, 10]
            ]),
            vec![vec![1, 10]]
        );
        assert_eq!(
            q56::Solution::merge(vec![
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
            q57::Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            q57::Solution::insert(
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
        assert_eq!(q57::Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
        assert_eq!(
            q57::Solution::insert(vec![vec![1, 1]], vec![5, 7]),
            vec![vec![1, 1], vec![5, 7]]
        );
        assert_eq!(
            q57::Solution::insert(vec![vec![1, 1], vec![2, 6]], vec![5, 7]),
            vec![vec![1, 1], vec![2, 7]]
        );
    }

    if *condition.get("q973").unwrap_or(&false) {
        println!("Question 973 - K Closest Points to Origin");
        // assert_eq!(
        //     q973::Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
        //     vec![vec![-2, 2]]
        // );
        // assert_eq!(
        //     q973::Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
        //     vec![vec![3, 3], vec![-2, 4]]
        // );
        // assert_eq!(
        //     q973::Solution::k_closest(vec![vec![6, 10], vec![-3, 3], vec![-2, 5], vec![0, 2]], 3),
        //     vec![vec![0, 2], vec![-3, 3], vec![-2, 5]]
        // );
        assert_eq!(
            q973::Solution::k_closest(
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
            q383::Solution::can_construct(
                String::from("bg"),
                String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj")
            ),
            true
        )
    }

    if *condition.get("q15").unwrap_or(&false) {
        println!("Question 15 - 3Sum");
        assert_eq!(
            q15::Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(q15::Solution::three_sum(vec![1, 2, -2, 1]), vec![vec![]]);
    }

    if *condition.get("q150").unwrap_or(&false) {
        println!("Question 150 - Evaluate Reverse Polish Notation");
        assert_eq!(
            q150::Solution::eval_rpn(vec!["1".to_string(), "2".to_string(), "+".to_string()]),
            3
        );
        assert_eq!(
            q150::Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
        assert_eq!(
            q150::Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "/".to_string()
            ]),
            1
        );
        assert_eq!(
            q150::Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
        assert_eq!(
            q150::Solution::eval_rpn(vec![
                "-1".to_string(),
                "1".to_string(),
                "*".to_string(),
                "-1".to_string(),
                "+".to_string()
            ]),
            -2
        );
        assert_eq!(
            q150::Solution::eval_rpn(vec![
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
}
