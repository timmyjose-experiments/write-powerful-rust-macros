#include <array>
#include <iostream>
#include <string>

int main() {
  std::string s;
  std::array<char, 5> state = {'h', 'e', 'l', 'l', 'o'};
  int pos = 0;

  std::getline(std::cin, s);

  for (auto &c : s) {
    if (c == state[pos]) {
      pos++;
    }
  }

  std::cout << (pos == state.size() ? "YES\n" : "NO\n");

  return 0;
}