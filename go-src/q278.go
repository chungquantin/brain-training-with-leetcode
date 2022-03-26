package leetcode

func IsBadVersion(n int) bool {
			return n % 2 == 0 
}
func FirstBadVersion(n int) int {
			s, e := 1, n
			for {
				if (s < e){
     p := (e+s) / 2
					if (IsBadVersion(p)){
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