package fs

import (
	"encoding/json"
	"os"
)

const SuperBlockPath = ".blaze/fs/superblock"

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
}

func NewSuperBlock() *SuperBlock {
	return &SuperBlock{Version: 1, InodeCounter: 1, root: nil}
}

func LoadSuperBlock() *SuperBlock {
	f, err := os.Open(SuperBlockPath)
	check(err)
	defer close(f)

	var sb SuperBlock

	decoder := json.NewDecoder(f)
	err = decoder.Decode(&sb)
	check(err)

	return &sb
}
