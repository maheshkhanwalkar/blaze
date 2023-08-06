package core

import "blaze/core/fs"

// Commit is a integral "unit" of change within the version control system.
type Commit struct {
	id          int64        // commit id
	author      Author       // author of the commit
	message     string       // commit message
	rootEntry   *fs.DirEntry // root inode (in-memory)
	rootVersion int          // version of root inode that this commit points to
}
