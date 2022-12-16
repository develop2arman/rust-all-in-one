
## Conditional compilation

[[conditional_compilation]]


### Example

```
Enables use statement only when serde is enabled.
 #[cfg(feature = "serde")]
 use serde::{Deserialize, Serialize};

 use zeroize::Zeroize;


 // Enables the derive() statement only when serde is enabled.
 #[cfg_attr(
     feature = "serde",
     derive(Serialize, Deserialize, Zeroize, Debug, PartialEq)
 )] // B
 //Enables the derive() statement only when serde is disabled.
 #[cfg_attr(not(feature = "serde"), derive(Zeroize, Debug, PartialEq))]
 #[zeroize(drop)]
 //Message container, for use with unencrypted messages
 pub struct Message(pub Box<InputBase>);
 ````
 
tags  #cfg_attr #cfg
