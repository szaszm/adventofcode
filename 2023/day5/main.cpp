#include <iostream>
#include <fstream>
#include <vector>
#include <charconv>
#include <system_error>
#include <cassert>
#include <functional>
#include <unordered_map>
#include "fmt/format.h"
#include "fmt/ranges.h"
#include "range/v3/all.hpp"

struct num_mapped_type {
  uint64_t dest_number{};
  uint64_t range_length{};
};
using num_mapping_type = std::map<uint64_t, num_mapped_type, std::greater<>>;

struct type_mapping {
  std::string dest_type;
  num_mapping_type num_mapping;

  [[nodiscard]] uint64_t lookup(uint64_t num) const {
    const auto it = num_mapping.lower_bound(num);
    if (it == std::end(num_mapping)) return num;
    const auto diff = num - it->first;
    if (diff >= it->second.range_length) return num;
    return it->second.dest_number + diff;
  }
};

const auto parseint = [](const auto& rng) {
  uint64_t num{};
  const char* const first = &*std::begin(rng);
  const char* const last = first + ranges::distance(rng);
  const auto [ptr, ec] = std::from_chars(first, last, num);
  if (ec != std::errc{}) {
    fmt::print("from_chars: errc={}, string=\'{}\'\n", static_cast<int>(ec), std::string_view{first, static_cast<size_t>(ranges::distance(rng))});
  }
  assert(ec == std::errc{});
  return num;
};

struct lookup_result {
  std::string type;
  uint64_t num;

  void print() const {
    //fmt::print("{}[{}] ", type, num);
  }
};

struct oracle {
  std::unordered_map<std::string, type_mapping> mapping_by_src_type;

  [[nodiscard]] lookup_result lookup(const std::string& src_type, uint64_t num) const {
    const auto& mapping = mapping_by_src_type.at(src_type);
    return lookup_result{
      .type = mapping.dest_type,
      .num = mapping.lookup(num)
    };
  }

  [[nodiscard]] uint64_t chain_lookup(const std::string& src_type, uint64_t src_num, const std::string& dst_type) const {
    //static num_mapped_type cache{-1, -1};
    lookup_result current{
      .type = src_type,
      .num = src_num
    };
    current.print();
    while(current.type != dst_type) {
      current = lookup(current.type, current.num);
      current.print();
    }
    return current.num;
  }
};

int main()
{
  std::ifstream ifs{"in.txt"};
  const auto seeds = [&] {
    std::string first_line;
    std::getline( ifs, first_line );
    first_line = first_line.substr( first_line.find( ' ' ) + 1 );
    return first_line | ranges::view::split(' ') | ranges::view::filter(std::not_fn(ranges::empty))
        | ranges::view::transform(parseint) | ranges::to<std::vector>();
  }();
  std::cout << fmt::format("seeds: {}\n", seeds);
  const std::vector<std::string> lines = [&] {
    std::vector<std::string> result;
    std::string line;
    while(std::getline(ifs, line)) {
      result.push_back(line);
    }
    return result;
  }();
  auto maps_strs = lines | ranges::view::split("") | ranges::view::filter(std::not_fn(ranges::empty));
  oracle oracle;
  for(const auto& map_line: maps_strs) {
    std::string type = ranges::front(map_line);
    type = type.substr(0, type.size() - 5);
    const auto type_split = type | ranges::view::split('-') | ranges::to<std::vector<std::string>>();
    const std::string& from_type = ranges::front(type_split);
    const std::string& to_type = ranges::back(type_split);
    num_mapping_type num_mapping;
    auto elements = map_line | ranges::view::drop(1);
    for(const auto& elem: elements) {
      const auto numbers = elem | ranges::view::split(' ') | ranges::view::transform(parseint) | ranges::to<std::vector>();
      assert(numbers.size() == 3);
      const auto dest_range_start = numbers[0];
      const auto src_range_start = numbers[1];
      const auto range_length = numbers[2];
      std::cout << fmt::format("mapping {}[{}] -> {}[{}] length {}\n", from_type, src_range_start, to_type, dest_range_start, range_length);
      num_mapping.insert_or_assign(src_range_start, num_mapped_type{
        .dest_number = dest_range_start,
        .range_length = range_length
      });
    }
    oracle.mapping_by_src_type.insert_or_assign(from_type, type_mapping{
      .dest_type = to_type,
      .num_mapping = std::move(num_mapping)
    });
  }
  for (const auto& [from_type, mapping] : oracle.mapping_by_src_type) {
    fmt::print("{}-to-{} map:\n", from_type, mapping.dest_type);
    for(const auto& [start, nm]: mapping.num_mapping) {
      fmt::print("{} -> {} [{}]\n", start, nm.dest_number, nm.range_length);
    }
    fmt::print("\n");
  }

  const auto lookup_test = [&](const std::string& src_type, uint64_t num) {
    const auto res = oracle.lookup(src_type, num);
    fmt::print( "{} {} -> {} {}\n", src_type, num, res.type, res.num);
  };
  lookup_test("seed", 79);
  lookup_test("seed", 14);
  lookup_test("seed", 55);
  lookup_test("seed", 13);
  lookup_test("humidity", 82);

  {
    std::vector<uint64_t> locations;
    for (uint64_t seednum: seeds) {
      const auto locnum = oracle.chain_lookup("seed", seednum, "location");
      fmt::print("\nresult: seed[{}] -> location[{}]\n", seednum, locnum);
      locations.push_back(locnum);
    }
    fmt::print("part1 lowest location number: {}\n", ranges::min(locations));
  }

  {
    const std::string src_type = "seed";
    const std::string dst_type = "location";
    fmt::println("part2 seed ranges:");
    std::vector<uint64_t> locations;
    for (const auto seed_ranges: seeds | ranges::view::chunk(2)) {
      const auto start = seed_ranges[0];
      const auto length = seed_ranges[1];
      const auto end = start + length;
      fmt::println("{} - {} (len: {})", start, end, length);
      for(auto i = start; i < end; ++i) {
        locations.push_back(oracle.chain_lookup(src_type, i, dst_type));
        //fmt::print("result: seed[{}] -> location[{}]\n", i, locations.back());
      }
    }
    fmt::print("part2 lowest location number: {}\n", ranges::min(locations));
  }
  return 0;
}
