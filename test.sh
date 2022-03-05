#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo build 
  ./target/debug/nineccr "$input" > tmp.s
  gcc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 '0;'
assert 42 '42;'
assert 14 '7+12-5;'
assert 41 ' 12    + 34 -     5   ;'
assert 47 '5+6*7;'
assert 39 '5*  9- 6;'
assert 17 ' 3*5 + 4 / 2;'
assert 30 '5 *(9-6 ) *2;'
assert 4 '(3+5)/2;'
assert 11 '-44+55;'
assert 10 '- (-10);'
assert 10 '- (- (+10));'

assert 0 '0==1;'
assert 1 '42==42;'
assert 1 '0!=1;'
assert 0 '42!=42;'

assert 1 '0<1;'
assert 0 '1<1;'
assert 0 '2<1;'
assert 1 '0<=1;' 
assert 1 '1<=1;'
assert 0 '2<=1;'

assert 1 '1>0;'
assert 0 '1>1;'
assert 0 '1  >2;'
assert 1 '1>=(0);'
assert 1 '1>=  1 ;'
assert 0 '1>=2;'

assert 3 '1; 2; 3;'

assert 3 'a=3;'
assert 16 'a=7; i=9; a+i;'
assert 8 'z=3; h=5; z+h;'
assert 7 'b=3; h=5; p=1; i = 2; b+h+p-i;'
assert 6 'b=3; h=5; p=2; i = b+h; i-p;'

assert 3 'foo=3; foo;'
assert 8 'foo123=3; bar=5; foo123+bar;'
assert 2 'foo123=3; bar=5; bar_sub_foo = bar-foo123;'


echo OK