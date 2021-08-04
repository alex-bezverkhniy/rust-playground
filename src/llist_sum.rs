use core::ptr::NonNull;

pub struct Solution;

struct LinkedList {
  head: Option<NonNull<Node>>,
  size: usize
}

struct Node {
  data: u8,
  next: Option<NonNull<Node>>
}

impl Node {
  
  fn new(data: u8) -> Self {
    Node {data: data, next: None}
  }
  
}

impl LinkedList {
  fn push(&self, node: Node) {
  /* TODO: learn Option first!
    let mut lastNode = self.head;
    // Check if node is not empty(None)
    match lastNode {
      Some(lastNode) => {
        // move to last node
        while lastNode.next != None {
          lastNode = lastNode.next;
        }
      }
    }
  
    lastNode.next = node;
    */
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn test1() {
  }
}