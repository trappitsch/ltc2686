# Notes to myself for development

## ToDos

This is not a real to-do list in the sense
that it's all the things that need to be done.
It rather represents a list of very next steps
and a list of down-the-road tasks to figure out.
Driver completeness is completely ignored from this list for now!

### Very next steps

### Notes to pass on to user

- Offset adjustment for one channel seems to be wrong for 12bit version,
  see comment in `ll`.

### Down the road

- How to support 12 bit version of driver (see below).
- CRC checking: Driver implements it but there's no way to send it yet.

## Notes

### Register RW vs. Commands

- Command #20 (Read offset and gain adjusted code of DAC Channel x)
  actually reads the register that we set and NOT the outputted value,
  i.e., the value that is loaded into the DAC.
  Thus, I believe that commands #0-#3 (write)/#20-23 (read)
  should be implemented as register access.

### CRC

The CRC is completely optional in write and read.
It can be left out and only 3 bytes sent if it is not activated.
In this case, it can also be sent and is just ignored.
This is the case for read and write.

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
