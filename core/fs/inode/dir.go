package inode

import (
	"blaze/file"
	"blaze/proto/fs/inode"
	"fmt"
)

// DirInode is a structure of a versioned inode that represents a directory.
// Since this inode supports versioning it maintains a directory entry mapping that
// is keyed by version (as well as name) to get the corresponding versioned inode.
type DirInode struct {
	inode *inode.DirInode
	dirty bool
}

func NewDirInode(ino int64) *DirInode {
	return &DirInode{inode: &inode.DirInode{Ino: ino, VCounter: 1, Entries: map[string]*inode.DirEntry{}}, dirty: true}
}

func (ip *DirInode) ToDisk() {
	if !ip.dirty {
		return
	}

	path := fmt.Sprintf(".blaze/inode/%d", ip.inode.Ino)
	protoInode := ip.inode
	file.Serialise(protoInode, path)
}

func (ip *DirInode) Lookup(entryKey *EntryKey) *inode.DirEntry {
	key := entryKey.Key()
	return ip.inode.Entries[key]
}
