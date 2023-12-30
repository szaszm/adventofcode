#include <iostream>
#include <cstdint>
#include <charconv>
#include <string>
#include <string_view>
#include <vector>
#include <algorithm>
#include <fstream>
#include <sstream>
#include "fmt/format.h"
#include "fmt/ranges.h"
#include "range/v3/all.hpp"

const auto parseint = []<typename T = int64_t>(const auto& rng) {
  T num{};
  const char* const first = &*std::begin(rng);
  const char* const last = first + ranges::distance(rng);
  const auto [ptr, ec] = std::from_chars(first, last, num);
  if (ec != std::errc{}) {
    fmt::print("from_chars: errc={}, string=\'{}\'\n", static_cast<int>(ec), std::string_view{first, static_cast<size_t>(ranges::distance(rng))});
  }
  assert(ec == std::errc{});
  return num;
};

auto numbers_except_first_word(const auto& line) {
  std::vector<int64_t> res;
  std::string word;
  std::stringstream ss{line};
  ss >> word;
  while(ss >> word) {
    res.push_back(parseint(word));
  }
  return res;
}

// more mathy solution would be: count of ints where x*(distance-x)>time
// solve for the x*(distance-x)-time=0, and rount to integer inwards, and subtract
uint64_t possible_win_count(int64_t time, int64_t distance) {
  uint64_t cnt = 0;
  for(int64_t i = 1; i < time; ++i) {
    if (i * (time - i) > distance) {
      //fmt::println("{} in {}/{}", i, time, distance);
      ++cnt;
    }
  }
  return cnt;
}


int main()
{
  const auto [times_line, distances_line] = []{
    std::ifstream ifs{"in.txt"};
    std::string times, distances;
    std::getline(ifs, times);
    std::getline(ifs, distances);
    return std::make_tuple(std::move(times), std::move(distances));
  }();
  fmt::print("{}\n{}\n", times_line, distances_line);
  const auto times = numbers_except_first_word(times_line);
  const auto distances = numbers_except_first_word(distances_line);
  const auto zipped = ranges::view::zip(times, distances);
  fmt::println("{}", zipped);
  uint64_t mul = 1;
  for(const auto& elem: zipped) {
    const auto time = std::get<0>(elem);
    const auto distance = std::get<1>(elem);
    mul *= possible_win_count(time, distance);
  }
  fmt::println("mul: {}", mul);

  const auto time_p2 = parseint(times_line | ranges::view::filter([](char c) { return std::isdigit(c); }) | ranges::to<std::string>());
  const auto distance_p2 = parseint(distances_line | ranges::view::filter([](char c) { return std::isdigit(c); }) | ranges::to<std::string>());
  fmt::println("part2 time: {}  distance: {}", time_p2, distance_p2);
  fmt::println("part2 count: {}", possible_win_count(time_p2, distance_p2));
  return 0;
}
