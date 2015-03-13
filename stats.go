package influxdb

import (
	"sync"
)

type Int struct {
	mu sync.RWMutex
	i  int64
}

func NewInt(v int64) *Int {
	return &Int{i: v}
}

// Add atomically adds the given delta to the Int.
func (i *Int) Add(delta int64) {
	i.mu.Lock()
	defer i.mu.Unlock()
	i.i += delta

}

type Stats struct {
	m  map[string]*Int
	mu sync.RWMutex
}

func NewStats() *Stats {
	return &Stats{
		m: make(map[string]*Int),
	}
}

// Add adds delta to the stat indiciated by key.
func (s *Stats) Add(key string, delta int64) {
	s.mu.RLock()
	i, ok := s.m[key]
	s.mu.RUnlock()
	if !ok {
		// check again under the write lock
		s.mu.Lock()
		i, ok = s.m[key]
		if !ok {
			i = new(Int)
			s.m[key] = i
		}
		s.mu.Unlock()
	}

	i.Add(delta)
}

// Inc simply increments the given key by 1.
func (s *Stats) Inc(key string) {
	s.Add(key, 1)
}

func (s *Stats) Get(key string) int64 {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.m[key].i
}

func (s *Stats) Set(key string, v int64) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.m[key] = NewInt(v)
}

// Walk calls f for each entry in the stats. The stats are locked
// during the walk but existing entries may be concurrently updated.
func (s *Stats) Walk(f func(string, int64)) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	for k, v := range s.m {
		f(k, v.i)
	}
}

// Diff returns the difference between two sets of stats. The result is undefined
// if the two Stats objects do not contain the same keys.
func (s *Stats) Diff(other *Stats) *Stats {
	diff := NewStats()
	s.Walk(func(k string, v int64) {
		diff.Set(k, v-other.Get(k))
	})
	return diff
}
