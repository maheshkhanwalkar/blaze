package fs

import (
	"blaze/common"
	"blaze/core/fs/inode"
	"blaze/file"
	"blaze/proto/fs"
	"os"
)

const SuperBlockPath = ".blaze/superblock"
const RootIno = 1

// SuperBlock is the top-level filesystem structure, which contains inode metadata
// related to allocation and freeing of inodes.
type SuperBlock struct {
	sb    *fs.SuperBlock
	root  *inode.DirInode // (in-memory) root inode
	dirty bool
}

func (sb *SuperBlock) ToDisk() {
	if !sb.dirty {
		return
	}

	f, err := os.Create(SuperBlockPath)
	common.Check(err)
	defer common.CloseFile(f)

	file.Serialise(sb.sb, SuperBlockPath)
	sb.root.ToDisk()
}

func CreateSuperBlock() *SuperBlock {
	root := CreateRootInode()
	return &SuperBlock{sb: &fs.SuperBlock{Version: 1, InodeCounter: 2, FreeInodes: []int64{}}, root: root, dirty: true}
}

func CreateRootInode() *inode.DirInode {
	return inode.NewDirInode(RootIno)
}

func LoadSuperBlock() *SuperBlock {
	// TODO
	return nil
}
