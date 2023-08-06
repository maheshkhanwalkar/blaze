package inode

// FileInode is a structure of a versioned inode that represents a file.
// Since this inode supports versioning, it maintains a mapping of a version to
// the underlying blob in the version-control filesystem.
type FileInode struct {
	Inode                     // common inode data
	VersionMap map[int]string `json:"versionMap"` // mapping of version number to blob
}

func (ip *FileInode) Update(blob string) int {
	version := getAndIncrement(&ip.VersionCounter)
	ip.VersionMap[version] = blob
	return version
}
