package inode

import (
	"blaze/common"
	"encoding/json"
	"fmt"
	"os"
)

// DirInode is a structure of a versioned inode that represents a directory.
// Since this inode supports versioning it maintains a directory entry mapping that
// is keyed by version (as well as name) to get the corresponding versioned inode.
type DirInode struct {
	Inode                         // common inode data
	Entries map[EntryKey]DirEntry `json:"entries"` // directory entries
	dirty   bool
}

func NewDirInode(ino int64) *DirInode {
	return &DirInode{Inode: Inode{Ino: ino, VersionCounter: 1}, Entries: map[EntryKey]DirEntry{}, dirty: true}
}

func (ip *DirInode) MarshalJSON() ([]byte, error) {
	rekeyed := make(map[string]DirEntry)

	for key, value := range ip.Entries {
		nKey := fmt.Sprintf("(%s,%d)", key.Name, key.Version)
		rekeyed[nKey] = value
	}

	return json.Marshal(&struct {
		Inode
		Entries map[string]DirEntry
	}{
		Inode:   Inode{Ino: ip.Ino, VersionCounter: ip.VersionCounter},
		Entries: rekeyed,
	})
}

func (ip *DirInode) ToDisk() {
	// Nothing to write, if inode isn't dirty
	if !ip.dirty {
		return
	}

	path := fmt.Sprintf(".blaze/inode/%d", ip.Ino)
	f, err := os.Create(path)
	common.Check(err)
	defer common.CloseFile(f)

	encoder := json.NewEncoder(f)
	err = encoder.Encode(ip)
	common.Check(err)
}
