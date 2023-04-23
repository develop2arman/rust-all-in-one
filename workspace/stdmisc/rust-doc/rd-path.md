

The Path struct represents file paths in the underlying filesystem. There are two flavors of Path: posix::Path, for UNIX-like systems, and windows::Path, for Windows. The prelude exports the appropriate platform-specific Path variant.

A Path can be created from an OsStr, and provides several methods to get information from the file/directory the path points to.

A Path is immutable. The owned version of Path is PathBuf. The relation between Path and PathBuf is similar to that of str and String: a PathBuf can be mutated in-place, and can be dereferenced to a Path.

Note that a Path is not internally represented as an UTF-8 string, but instead is stored as an OsString. Therefore, converting a Path to a &str is not free and may fail (an Option is returned). However, a Path can be freely converted to an OsString or &OsStr using into_os_string and as_os_str, respectively.

Be sure to check at other Path methods (posix::Path or windows::Path) and the Metadata struct.



> `tags` [[path]] [[metadata]] [[OSString]] [[OsStr]]