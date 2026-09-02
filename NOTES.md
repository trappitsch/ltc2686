# Notes to myself for development

## ToDos

This is not a real to-do list in the sense
that it's all the things that need to be done.
It rather represents a list of very next steps
and a list of down-the-road tasks to figure out.
Driver completeness is completely ignored from this list for now!

### Very next steps

- Work through naming of the low-level driver
  - remove some of the `channel_` prefixes in channel ops
- Is there a better way to set the registers for power down,
  etc.?
- Extend intro to docs of ll driver

### Notes to pass on to user

- Offset adjustment for one channel seems to be wrong for 12bit version,
  see comment in `ll`.

### Down the road

- How to support 12 bit version of driver (see below).
- Test performance and size of firmware using
  my CRC calculation or `crc` crate.

## Notes

### Register RW vs. Commands

- Command #20 (Read offset and gain adjusted code of DAC Channel x)
  actually reads the register that we set and NOT the outputted value,
  i.e., the value that is loaded into the DAC.
  Thus, I believe that commands #0-#3 (write)/#20-23 (read)
  should be implemented as register access.

### CRC

The CRC is a standard 6 bit wide, polynomial `0x03` CRC-6.
There no flipping (in or out), and no `xor_in` or `xor_out`

**Important:** For the CRC-6 calculation, use the 24 bits
from the first two three bytes
and add the two "do not care" bits from the fourth byte.
So the value to compute the CRC-6 for has a total length of 26 bit.

**Note:** The value returned from the LTC2686
will always contain all zeros in the last bit,
and thus returned values cannot be CRC checked.

#### CRC determination with Pico de Gallo

Since my first attempts of calculating the CRC
were never accepted by the LTC2686,
I used Pico de Gallo to empirically determine CRC-6.
A 6 bit CRC can run from `0x00` to `0x3f`.
If the command is not accepted, the fault register
will contain an adequate fault.
Using Pico de Gallo, it is very straightforward
to write a routine that will send a command with a CRC
and then check if it was accepted or not.
If it was accepted: great, we found the correct CRC
and are done.
If it is not accepted, reset the fault register,
increment the CRC, and try again.
With Pico de Gallo, this is very fast and easy to accomplish.

Here are some CRCs that I found:

| Data (3 bytes)   | Accepted CRC (6 bits) | Remarks    |
| ---------------- | --------------------- | ---------- |
| 0x00, 0x00, 0x00 | 0x00                  |            |
| 0x00, 0x00, 0x43 | 0x00                  | Polynomial |
| 0x40, 0x00, 0x00 | 0x33                  |            |
| 0x40, 0x12, 0x34 | 0x16                  |            |
| 0x40, 0x7F, 0x7F | 0x39                  |            |
| 0x40, 0xAB, 0xCD | 0x29                  |            |
| 0x40, 0xFF, 0xFF | 0x38                  |            |

### 12 bit implementation

DD does not error if a user sends a `u16` for a field that is only 12-bit long.
The higher bits just get truncated (makes a lot of sense).

All below answers from Matrix chat on 2026-08-29:

Dion on how to solve this:

> You could have two fields, one for the 12-bit version
> and one for the 16-bit version.
> The 12-bit field will do the calculations for you
> If you do that, you must allow bit overlap:
> <https://device-driver.com/book/v2/language-fieldset.html#bit-overlap>

Dion on range checking to prevent the user from sending a 16-bit value
to a 12-bit field:

> In device-driver everything is truncated. So the upper bits would be ignored
> I should add a strict mode or something... It'd panic if the value doesn't fit
> The alternative would be fallible setter, but not sure if I like that

Conclusion:

- Adding two fields would make me ready if a strict mode gets added.
- Range checking for now
  and 12-bit vs 16-bit needs to be implemented in the higher level driver.
- If I go with one field,
  the higher level driver will need to bit shift
  when in 12-bit mode.
  This seems slightly ugly.
- Having two fields and two methods for each would allow feature gating
  and features being additive…
