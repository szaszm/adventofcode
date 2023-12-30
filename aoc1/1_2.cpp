#include <algorithm>
#include <array>
#include <iostream>
#include <limits>
#include <fstream>
#include <cstdint>
#include <string>
#include <string_view>
//#include "fmt/format.h"
//#include "fmt/ranges.h"

auto find_first_number(std::string_view str) {
	std::array<int, 20> positions;
	const int empty = std::numeric_limits<int>::max();
	std::fill(std::begin(positions), std::end(positions), empty);
	const auto find = [&](std::string_view substr) -> int {
		const auto idx = str.find(substr);
		if (idx == std::string_view::npos) return empty;
		return static_cast<int>(idx);
	};
	positions[0] = find("0");
	positions[1] = find("1");
	positions[2] = find("2");
	positions[3] = find("3");
	positions[4] = find("4");
	positions[5] = find("5");
	positions[6] = find("6");
	positions[7] = find("7");
	positions[8] = find("8");
	positions[9] = find("9");
	positions[10] = find("zero");
	positions[11] = find("one");
	positions[12] = find("two");
	positions[13] = find("three");
	positions[14] = find("four");
	positions[15] = find("five");
	positions[16] = find("six");
	positions[17] = find("seven");
	positions[18] = find("eight");
	positions[19] = find("nine");
	//std::cerr << fmt::format("{}", positions) << '\n';
	auto mini = -1;
	auto minv = std::numeric_limits<int>::max();
	for(size_t i = 1; i < 20; ++i) {
		const int64_t iv = positions[i];
		if (iv != empty && iv < minv) {
			mini = i;
			minv = iv;
		}
	}
	return mini % 10;
}
auto find_last_number(std::string_view str) {
	std::array<int, 20> positions;
	const int empty = -1;
	std::fill(std::begin(positions), std::end(positions), std::string_view::npos);
	const auto find = [&](std::string_view substr) -> int {
		const auto idx = str.rfind(substr);
		if (idx == std::string_view::npos) return empty;
		return static_cast<int>(idx);
	};
	positions[0] = find("0");
	positions[1] = find("1");
	positions[2] = find("2");
	positions[3] = find("3");
	positions[4] = find("4");
	positions[5] = find("5");
	positions[6] = find("6");
	positions[7] = find("7");
	positions[8] = find("8");
	positions[9] = find("9");
	positions[10] = find("zero");
	positions[11] = find("one");
	positions[12] = find("two");
	positions[13] = find("three");
	positions[14] = find("four");
	positions[15] = find("five");
	positions[16] = find("six");
	positions[17] = find("seven");
	positions[18] = find("eight");
	positions[19] = find("nine");
	auto maxi = -1;
	auto maxv = std::numeric_limits<int>::min();
	for(size_t i = 1; i < 20; ++i) {
		const int iv = positions[i] == std::string_view::npos ? empty : positions[i];
		if (iv > maxv && iv != empty) {
			maxi = i;
			maxv = iv;
		}
	}
	return maxi % 10;
}

int main() {
	std::string line;
	//std::ifstream ifs{"1_2.txt"};
	while (std::getline(std::cin, line)) {
		if (line.empty()) continue;
		std::cout << find_first_number(line) << find_last_number(line) << '\n';
	}
}
