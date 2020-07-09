// Queue data structure

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyQueue
}

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T>
{
    pub fn new() -> Queue<T> {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyQueue)
        }
        Ok(self.items.swap_remove(0))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_empty_vector() {
        let queue: Queue<i32> = Queue::new();
        assert_eq!(queue.items, Vec::new());
    }

    #[test]
    fn should_add_item_to_queue() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(2);
        assert_eq!(queue.items.len(), 1);
        assert_eq!(queue.items[0], 2);
    }

    #[test]
    fn should_remove_first_item_from_queue() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.items.len(), 3);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.items.len(), 2);
    }

    #[test]
    fn should_not_explode_when_removing_empty_queue() {
        let mut queue: Queue<i32> = Queue::new();
        match queue.dequeue() {
            Err(err) => assert_eq!(err, Error::EmptyQueue),
            Ok(_) => assert!(true),
        }
    }

    #[test]
    fn should_return_if_queue_is_empty() {
        let mut queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        assert!(!queue.is_empty());

        match queue.dequeue() {
            Ok(_) | Err(_) => assert!(queue.is_empty())
        };
    }
}
