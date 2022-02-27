#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo build 
  ./target/debug/nineccr "$input" > tmp.s
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
assert 14 '7+12-5'
assert 41 ' 12    + 34 -     5   '
assert 47 '5+6*7'
assert 39 '5*  9- 6'
assert 17 ' 3*5 + 4 / 2'
assert 30 '5 *(9-6 ) *2'
assert 4 '(3+5)/2'
assert 11 '-44+55'
assert 10 '- (-10)'
assert 10 '- (- (+10))'

assert 0 '0==1'
assert 1 '42==42'
assert 1 '0!=1'
assert 0 '42!=42'

assert 1 '0<1'
assert 0 '1<1'
assert 0 '2<1'
assert 1 '0<=1'
assert 1 '1<=1'
assert 0 '2<=1'

assert 1 '1>0'
assert 0 '1>1'
assert 0 '1  >2'
assert 1 '1>=(0)'
assert 1 '1>=  1 '
assert 0 '1>=2'

echo OK