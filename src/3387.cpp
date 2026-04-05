#include <cassert>
#include <map>
#include <vector>
#include <string>
using namespace std;
class Solution {
public:
    double maxAmount(string initialCurrency,
                     vector<vector<string>>& pairs1, vector<double>& rates1,
                     vector<vector<string>>& pairs2, vector<double>& rates2) {
        map<string, double> d;
        d[initialCurrency] = 1;

        fordBellman(d, pairs1, rates1);
        fordBellman(d, pairs2, rates2);

        return d[initialCurrency];
    }
private:
    map<string, double> fordBellman(map<string, double>& d, vector<vector<string>>& pairs, vector<double>& rates) {
        for (int _ = 0; _ < 10; ++_){
            for (int j = 0; j < pairs.size(); ++j) {
                d[pairs[j][1]] = max(d[pairs[j][1]], d[pairs[j][0]] * rates[j]);
                d[pairs[j][0]] = max(d[pairs[j][0]], d[pairs[j][1]] / rates[j]);
            }
        }
        return d;
    }
};

auto sol = new Solution();
const string USD = "USD";
const string JPY = "JPY";
const string CHF = "CHF";
const string EUR = "EUR";
void test1() {
    string initialCurrency = "EUR";
    vector<vector<string>> pairs1 = {{EUR, USD}, {USD, JPY}};
    vector<vector<string>> pairs2 = {{JPY, USD}, {USD, CHF}, {CHF, EUR}};
    vector<double> rates1 = {2, 3};
    vector<double> rates2 = {4, 5, 6};
    double res = sol->maxAmount(initialCurrency, pairs1, rates1, pairs2, rates2);
    assert(res == 720.0);
}
void test2() {
    string initialCurrency = "S";
    vector<vector<string>> pairs1 = {{"S", "Z"}, {"Z", "FYN"}, {"FYN", "TW"}, {"TW", "V"}, {"V", "OO"}};
    vector<vector<string>> pairs2 = {{"OO", "CTV"}, {"CTV", "X"}, {"X", "CI"}, {"CI", "R"}, {"R", "S"}};
    vector<double> rates1 = {10, 10, 10, 10, 10};
    vector<double> rates2 = {10, 10, 10, 10, 10};
    double res = sol->maxAmount(initialCurrency, pairs1, rates1, pairs2, rates2);
    assert(res == 10000000000);
}
int main() {
    test1();
    test2();
}
