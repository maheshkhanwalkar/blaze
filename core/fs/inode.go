package fs

type InodeType int

const (
	File InodeType = iota
	Directory
)

// FileInode is a structure of a versioned inode that represents a file.
// Since this inode supports versioning, it maintains a mapping of a version to
// the underlying blob in the version-control filesystem.
type FileInode struct {
	ino        int64             // inode number
	versionMap map[string]string // mapping of version number to blob
}

// DirInode is a structure of a versioned inode that represents a directory.
// Since this inode supports versioning it maintains a directory entry mapping that
// is keyed by version (as well as name) to get the corresponding versioned inode.
type DirInode struct {
	ino     int64                 // inode number
	entries map[EntryKey]DirEntry // directory entries
}

// EntryKey is the key structure for looking up a directory entry within a directory inode.
type EntryKey struct {
	name    string // directory entry name
	version int    // version number
}

// DirEntry is a versioned, directory entry which points to a single version of an inode
type DirEntry struct {
	ino     int64     // corresponding inode number
	itype   InodeType // corresponding inode type
	version int       // corresponding inode version
}
