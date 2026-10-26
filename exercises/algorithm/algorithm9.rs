/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::fmt::Alignment::Right;
use std::thread::sleep;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);

        self.count+=1;
        let cmp = self.comparator; // ③ 复制 fn 指针出来，避免借用打架
        let mut idx = self.count;  // ④ 游标 = 新元素位置
        while idx > 1 && cmp(&self.items[idx], &self.items[self.parent_idx(idx)]) {
          // ⑤ 我比父节点"优" → 和父交换
          let parent = self.parent_idx(idx);   // ① 先算：返回 usize（值），&self 租约立即结束
            self.items.swap(idx, parent);        // ② 后写：此刻场上没有任何其他借用，独占合法
          idx = self.parent_idx(idx);   // ⑥ 游标跟上去
    }
    }
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let left=self.left_child_idx( idx);
		let right=self.right_child_idx( idx);
        if right<=self.count  {
            if (self.comparator)(&self.items[left], &self.items[right]) {
              left                               // 左比右"优" → 选左
          } else {
              right                              // 否则选右
          }
        }
        else {
            return  left;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
      if self.count == 0 {              // ① 空堆：测试第一行就是 next() on 空堆，必须 None
          return None;
      }

      // ② 金蝉脱壳：末尾元素顶上 1 号位，原堆顶被挤到末尾
      self.items.swap(1, self.count);
      self.count -= 1;
      let result = self.items.pop();    // ③ 挤出来的就是原堆顶 → 返回值到手

      // ④ 下沉：顶上来的新 1 号通常压不住场子
      let cmp = self.comparator;        // 老朋友：先复制 fn 指针防借用冲突
      let mut idx = 1;
      while self.children_present(idx) {
          let child = self.smallest_child_idx(idx);     // 选最优孩子
          if cmp(&self.items[child], &self.items[idx]) {
              self.items.swap(idx, child);              // 孩子更优 → 换位
              idx = child;                              // 游标跟下去
          } else {
              break;                                    // 当前节点坐稳了，提前收工
          }
      }
      result
  }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}