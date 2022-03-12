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

assert 1 '0<13;'
assert 0 '1<1;'
assert 0 '2<1;'
assert 1 '0<=1;' 
assert 1 '1<=1;'
assert 0 '12<=1;'

assert 1 '1>0;'
assert 0 '1>1;'
assert 0 '1  >2;'
assert 1 '1>=(0);'
assert 1 '1>=  1 ;'
assert 0 '1>=2;'

assert 3 '1; 2; 3;'

assert 3 'a=3;'
assert 16 'a=7; i=9; a+i;'
assert 7 'b=3; h=5; p=1; i = 2; b+h+p-i;'
assert 6 'b=3; h=5; p=2; i = b+h; i-p;'

assert 3 'foo=3; foo;'
assert 8 'foo123=3; bar=5; foo123+bar;'
assert 2 'foo123=3; bar=5; bar_sub_foo = bar-foo123;'

assert 3 'a=3; return a;'
assert 8 'a=3; z=5; return a+z;'

assert 1 'return 1; 2; 3;'
assert 2 '1; return 2; 3;'
assert 3 '1; 2; return 3;'

assert 3 'foo=3; return foo;'
assert 8 'foo123=3; bar=5; return foo123+bar;'
assert 8 'foo_123=3; returnbar=5; return foo_123+returnbar;'
assert 16 'ret=7; els3=9; ret+els3;'
assert 16 'return5=7; ifa=9; return5+ifa;'

assert 3 'if (0) return 2; return 3;'
assert 3 'if (1-1) return 2; return 3;'
assert 2 'if (1) return 2; return 3;'
assert 2 'if (2-1) return 2; else return 3;'

assert 2 'if (2-1) 2; else 7;'
assert 7 'if (1-1) return 2; else 7;'

assert 10 'i=0; while(i<10) i=i+1; return i;'

assert 3 'for (;;) return 3; return 5;'
assert 10 'for (i=0; i<10; i=i+1) 3; return i;'
assert 10 'i=0; for (; i<10; i=i+1) 3; return i;'
assert 10 'i=0; for (; i<10;) i=i+1; return i;'

assert 55 'i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j;'

echo OK