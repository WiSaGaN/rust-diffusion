initSidebarItems({"enum":[["Error","represents errors that can be encountered during the usage of of reader and writer."]],"struct":[["FileReader","is a reader that reads from a file. This file needs to be generated by a corresponding writer. Currently it does not support a growing file, e.g. files cannot be modified during read."],["FileWriter","is a writer for file. It can only start to write a new file but not append to an existing file."],["MulticastReader","is reader for multicast. Reads the UDP packet multicasted from writer. Each packet is a message."],["MulticastWriter","is writer for multicast. `MulticastWriter` uses the natual UDP packet as message boundary."]],"trait":[["Reader","is the general trait for all readers."],["Writer","is the general trait for all writers."]],"type":[["Result","is an alias for crate level result derived from the crate level `Error`."]]});