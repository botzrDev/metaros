struct MemoryBlock {
    start: usize,
    size: usize,
    is_free: bool,
}

pub struct MemoryManager {
    memory: Vec<u8>,
    blocks: Vec<MemoryBlock>,
}

impl MemoryManager {
    pub fn new(size: usize) -> Self {
        MemoryManager {
            memory: vec![0; size],
            blocks: vec![MemoryBlock { start: 0, size, is_free: true }],
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if let Some(index) = self.blocks.iter().position(|block| block.is_free && block.size >= size) {
            let block = &mut self.blocks[index];
            let alloc_start = block.start;
            
            if block.size > size {
                let new_block = MemoryBlock {
                    start: alloc_start + size,
                    size: block.size - size,
                    is_free: true,
                };
                self.blocks.insert(index + 1, new_block);
                block.size = size;
            }
            
            block.is_free = false;
            Some(alloc_start)
        } else {
            None
        }
    }

    pub fn deallocate(&mut self, start: usize) {
        if let Some(index) = self.blocks.iter().position(|block| block.start == start) {
            self.blocks[index].is_free = true;
            self.merge_free_blocks();
        }
    }

    fn merge_free_blocks(&mut self) {
        self.blocks.sort_by_key(|block| block.start);
        let mut i = 0;
        while i < self.blocks.len() - 1 {
            if self.blocks[i].is_free && self.blocks[i + 1].is_free {
                self.blocks[i].size += self.blocks[i + 1].size;
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager() {
        let mut mm = MemoryManager::new(1024);

        let addr1 = mm.allocate(100).unwrap();
        let addr2 = mm.allocate(200).unwrap();
        let addr3 = mm.allocate(300).unwrap();

        assert_eq!(addr1, 0);
        assert_eq!(addr2, 100);
        assert_eq!(addr3, 300);

        mm.deallocate(addr2);
        let addr4 = mm.allocate(150).unwrap();
        assert_eq!(addr4, 100);

        mm.deallocate(addr1);
        mm.deallocate(addr3);
        mm.deallocate(addr4);

        let addr5 = mm.allocate(1000).unwrap();
        assert_eq!(addr5, 0);
    }
}