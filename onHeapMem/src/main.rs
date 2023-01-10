
// Linked Lists with Boxes

#[derive(Debug)]
struct LinkedList<T> {
    next: Option<Box<LinkedList<T>>>,
    data: T
}

impl <T:std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList{
        data: 1,
        next: Some(Box::new(LinkedList{
            data: 2,
            next: None
        }))
    };

    if let Some(ref mut v) = ll.next {
        v.add_up(40)
    }

    let mut vector_arr = Vec::new();

    vector_arr.push("hello");
    vector_arr.push("test");

    println!("Length:: {}, Capacity {}",vector_arr.len(),  vector_arr.capacity());

    dbg!(ll);

}
