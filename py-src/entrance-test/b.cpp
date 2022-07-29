#include <bits/stdc++.h>

using namespace std;

int main() {
    // freopen("input.txt", "r", stdin);
    int T;
    cin >> T;
    while (T--) {
        int n, k;
        cin >> n >> k;
        vector<int> A(k);
        for (int i = 0; i < k; i++) {
            cin >> A[i];
        }

        sort(A.begin(), A.end());

        int ans = 0;
        int sum = 0;
        for (int i = k - 1; i >= 0; i--) {
            if (sum + n - A[i] < n) {
                sum += n - A[i];
                ans++;
            }
        }
        cout << ans << endl;
    }
    return 0;
}