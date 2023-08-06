package fs

import (
	"encoding/json"
	"os"
)

const SuperBlockPath = ".blaze/superblock"

// SuperBlock is the top-level filesystem structure, which contains inode metadata
// related to allocation and freeing of inodes.
type SuperBlock struct {
	Version      int       `json:"version"`    // super-block version
	InodeCounter int       `json:"iCounter"`   // new inode counter
	FreeInodes   []int     `json:"freeInodes"` // list of free inodes
	root         *DirInode // (in-memory) root inode
}

func (sb *SuperBlock) ToDisk() {
	f, err := os.Create(SuperBlockPath)
	check(err)
	defer close(f)

	encoder := json.NewEncoder(f)
	err = encoder.Encode(*sb)
	check(err)

	sb.root.ToDisk()
}

func NewSuperBlock() *SuperBlock {
	root := CreateRootInode()
	return &SuperBlock{Version: 1, InodeCounter: 2, FreeInodes: []int{}, root: root}
}

func CreateRootInode() *DirInode {
	return &DirInode{Inode: Inode{Ino: 1, VersionCounter: 1}, Entries: map[EntryKey]DirEntry{}, dirty: true}
}

func LoadSuperBlock() *SuperBlock {
	f, err := os.Open(SuperBlockPath)
	check(err)
	defer close(f)

	var sb SuperBlock

	decoder := json.NewDecoder(f)
	err = decoder.Decode(&sb)
	check(err)

	// TODO load root inode from disk
	return &sb
}
