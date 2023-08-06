package core

// Serializable interface for objects that need to be written to disk
type Serializable interface {
	ToDisk()
}
