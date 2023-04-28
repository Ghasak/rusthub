# About Number System Explanation

## Hexadecimal

- A byte is a unit of digital information that consists of 8 bits. In
  hexadecimal notation, each digit represents 4 bits. Therefore, a single byte
  can hold a maximum value of 2^8 - 1, which is equal to 255 in decimal or FF
  in hexadecimal. In other words, FF is the largest number that a single byte
  (8 bits) can hold in hexadecimal notation.

- In progrmming langues generally speaking any piece of data gets represented
  by a `hexadecimal` number system. Usually it get represents by a `two-cell`
  in memeroy these two cells are considered a `1-byte`. `1-byte` is `8-bits` in
  binary.

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-biti | 32     | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- `Memory` represents the `1-byte` usually in `hexadecimal` and its `two-cells`
  as `[[0][0]]` each can take value state {1,2,3,4,5,6,7,8,9,10,a,b,c,d,e,f}.

- The binary representation is not used for the memeory which consists of two
  states {1,0} as you will need to represent `1-byte` a `8` cells. instead of
  `2` cells in hexadecimal.

```rust
let my_value_08_bits   : i8 = 3;
let my_value_32_bits_1 : i32 = 15;
let my_value_32_bits_2 : i32 = 10;
```

| variable_name      | value | type | binary                              | hexadecimal | In Memeory                | num of bytes |
| ------------------ | ----- | ---- | ----------------------------------- | ----------- | ------------------------- | ------------ |
| my_value_08_bits   | 3     | i8   | 00000011                            | 03          | 0x03 or 03                | 1            |
| my_value_32_bits_1 | 15    | i32  | 00001111-00000000-00000000-00000000 | 0F          | 0x0f000000 or 0f 00 00 00 | 4            |
| my_value_32_bits_2 | 10    | i32  | 00001010-00000000-00000000-00000000 | 0A          | 0x0a000000 or 0a 00 00 00 | 4            |

- I used the `memory-veiwer` to see the values and how they are represented in
  `hexadecimal` numerial systems. Using the `dap-nvim` with `meme read &variable_name`

```shell
dap>

my_value_32_bits_2 ----------------------+
my_value_32_bits_1 ---------+            |
my_value_08_bits --------+  |            |
                         |  |            |
                         |  |            |
                         -- -- -- -- -- -- -- -- --
            0x16fdfe267: 03 0f 00 00 00 0a 00 00 00 00 00 00 00 00 00 00  ................
            0x16fdfe277: 00 67 e2 df 6f 01 00 00 00 48 02 04 00 01 00 00  .g..o....H......
            dap>

```

- The representation from deciaml -> binary or decimal -> hexa always involves with division.
- The representation from hexa -> decimal, binary -> decimal always involves with multiplication.

## To converate from Hex to Decimal

Use the `multiplication`. `Example` Lets converate `FF`

- We Know the mapping of hexadecimal number system as: {1-> 1, 2-> 2 , 3->3 ,
  4-> 4 , 5-> 5, 6->6, 7->7, 8->8, 9->9, 10->A, 11->B, 12->C, 13->D, 14->E,
  15->F} so, F-> means 15. Reading from left to right, first decimal assign to
  it 16^0, then second decimal 16^2, and so on and so forth.

```yaml
N ....       16^3 16^2  16^1 16^0
+------+----+----+----+----+----+
| ...  |  _ | _  | _  | F  |  F |
+------+----+----+----+----+----+
+------+----+----+----+----+----+
| ...  |  _ | _  | _  | 15 |  15 |
+------+----+----+----+----+----+
X (multiply)
+------+----+----+----+----+----+
| ...  |  _ | _  | _  |16^1|16^0|
+------+----+----+----+----+----+
                      <----+---->
                      1-byte size

FF = 15 * 16^0 + 15 * 16^1 = 255
```

- Notice, we care only about the first two cell, thus, maximum number for `1`
  byte can hold as `int` is `FF` or `255` this is maximum number that `1-byte`
  in memeory can hold, which is represented by `two` cells, which is `FF` in
  hexadecimal, or `255` in decimal.

- If you want to converate `255` to `hexadecimal` you will need to do the
  following steps. To convert 255 decimal to hexadecimal, we can use the
  following steps:

1. Divide the decimal number by `16` (since hexadecimal is base 16) to get a quotient and a remainder.

```yaml
255 ÷ 16 = 15 remainder 15
```

2. Convert the remainder to a hexadecimal digit using the following table:

```yaml
Decimal   Hexadecimal
-------   -----------
0           0
1           1
2           2
3           3
4           4
5           5
6           6
7           7
8           8
9           9
10          A
11          B
12          C
13          D
14          E
15          F
```

- In this case, the remainder is 15, which corresponds to the hexadecimal digit F.
3. Write down the hexadecimal digit as the least significant digit of the hexadecimal number.
- Hexadecimal number so far: F
4. Divide the quotient from step 1 by 16 to get a new quotient and remainder.
```yaml
15 ÷ 16 = 0 remainder 15
```
- Convert the remainder to a hexadecimal digit using the table from step 2.
```yaml
Remainder: 15 => Hexadecimal digit: F
```
5. Write down the hexadecimal digit as the second least significant digit of the hexadecimal number.
- Hexadecimal number so far: FF
6. Continue these steps until the quotient is 0.

```yaml
0 ÷ 16 = 0 remainder 0
```
7. The final hexadecimal number is the sequence of hexadecimal digits written
   in reverse order from the order in which they were calculated.
- Final hexadecimal number: FF
- Therefore, the hexadecimal representation of 255 decimal is FF.
