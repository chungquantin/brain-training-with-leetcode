#include <bits/stdc++.h>
#include <iostream>

#define loop(i, start, end) for (int i = start; i <= end; i++)

using namespace std;

int main(int argc, char const *argv[])
{
 // Get multiple columns input
 int a, b, x;
 cin >> a >> b >> x;

 // Input a vector
 int l = 20;
 vector<int> v(l, 0);
 loop(i, 0, l) cin >> v[i];

 // Input a matrix
 int r = 5;
 int c = 10;
 vector< vector<int> > matrix(r, vector<int>(c, 0));
 loop(i, 0, r)
 {
  loop(j, 0, c)
  {
   cin >> matrix[i][j];
  };
 };

 // Using definition to replace for loop
 loop(i, 0, 10) cout << i;

 // Sort a vector
 sort(v.begin(), v.end());

 // Sort a slice
 int n = 7; // array size
 int k[] = {4, 2, 5, 3, 5, 8, 3};
 sort(k, k + n);
}
