#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 14 "7+12-5"
assert 41 " 12    + 34 -     5   "
#失敗するはず
assert 21 "5+ 20 -  "

echo OK