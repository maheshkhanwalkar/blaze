package fs

import (
	"blaze/common"
	"blaze/core/fs/inode"
	"encoding/json"
	"os"
)

const SuperBlockPath = ".blaze/superblock"
const RootIno = 1

// SuperBlock is the top-level filesystem structure, which contains inode metadata
// related to allocation and freeing of inodes.
type SuperBlock struct {
	Version      int             `json:"version"`    // super-block version
	InodeCounter int             `json:"iCounter"`   // new inode counter
	FreeInodes   []int           `json:"freeInodes"` // list of free inodes
	root         *inode.DirInode // (in-memory) root inode
}

func (sb *SuperBlock) ToDisk() {
	f, err := os.Create(SuperBlockPath)
	common.Check(err)
	defer common.CloseFile(f)

	encoder := json.NewEncoder(f)
	err = encoder.Encode(*sb)
	common.Check(err)

	sb.root.ToDisk()
}

func NewSuperBlock() *SuperBlock {
	root := CreateRootInode()
	return &SuperBlock{Version: 1, InodeCounter: 2, FreeInodes: []int{}, root: root}
}

func CreateRootInode() *inode.DirInode {
	return inode.NewDirInode(RootIno)
}

func LoadSuperBlock() *SuperBlock {
	f, err := os.Open(SuperBlockPath)
	common.Check(err)
	defer common.CloseFile(f)

	var sb SuperBlock

	decoder := json.NewDecoder(f)
	err = decoder.Decode(&sb)
	common.Check(err)

	// TODO load root inode from disk
	return &sb
}
