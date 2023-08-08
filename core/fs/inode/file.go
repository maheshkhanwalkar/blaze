package inode

import "blaze/proto/fs/inode"

// FileInode is a structure of a versioned inode that represents a file.
// Since this inode supports versioning, it maintains a mapping of a version to
// the underlying blob in the version-control filesystem.
type FileInode struct {
	inode *inode.FileInode
}

func (ip *FileInode) Update(blob string) int32 {
	version := getAndIncrement(&ip.inode.VCounter)
	ip.inode.VersionMap[version] = blob
	return version
}
