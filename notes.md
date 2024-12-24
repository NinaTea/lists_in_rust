# Self, &mut self and &self

- self: actual value. <b>true ownership.</b>
- &mut self: mutable reference. <b>Can modify the value but has to return it in a valid state.</b> Temporary <i>exclusive access.</i>
- &self: shared reference. <b>Can not mutate the variable</b>. Temporary <i>shared access.</i>

``

# Option<T>

enum T may or may not exist.
