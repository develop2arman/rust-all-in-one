
Omitting lifetime annotations is formally referred to as lifetime elision

> Unfortunately, it’s not straightforward to disable the automatic drop functionality. Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically. Occasionally, however, you might want to clean up a value early. 

> Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function

---

> `tags` [[as_ptr]] [[from_utf8]] [[from_raw_parts]]
