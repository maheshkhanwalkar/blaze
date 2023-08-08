package core

import (
	"blaze/core/fs/inode"
	"blaze/proto/vcs"
)

// Commit is a integral "unit" of change within the version control system.
type Commit struct {
	commit    *vcs.Commit
	rootEntry *inode.DirInode // root inode (in-memory)
}
