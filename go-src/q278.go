package leetcode

func isBadVersion(n int) bool {
			return n % 2 == 0 
}
func firstBadVersion(n int) int {
			s, e := 1, n
			for {
				if (s < e){
     p := (e+s) / 2
					if (isBadVersion(p)){
						e = p
					} else {
						s = p;
					}
				} else {
					break
				}
			}
			return s 
}