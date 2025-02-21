struct Stack<T> {
    elements: Vec<T>,
    max_size: usize,
}

impl<T: std::fmt::Display> Stack<T>{
    fn push(&mut self, element: T) -> Result<(), &str> {
        if self.elements.len() >= self.max_size{
            Err("Din stack er fuld!")
        } else {
            self.elements.push(element);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<T>{
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T>{
        self.elements.last()
    }

    fn is_empty(&self) -> bool{
        self.elements.is_empty()
    }

    fn read_elements(&self) {
        for element in self.elements.iter(){
            println!("{}", element);
        }
    }
}

fn main() {
    // Den her string er til tal
    let mut stack = Stack{elements: Vec::new(), max_size: 5};

    println!("Er din stack tom? {}", stack.is_empty());

    stack.push(10);
    stack.push(20);
    stack.push(30);
    stack.push(40);
    stack.push(50);
    stack.push(60);

    stack.read_elements();
    println!("Er din stack tom? {}", stack.is_empty());
    println!("Top element: {:?}", stack.peek());
    println!("Pop element: {:?}", stack.pop());
    println!("Pop element: {:?}", stack.pop());
    println!("Pop element: {:?}", stack.pop());
    println!("Pop element: {:?}", stack.pop());
    println!("Pop element: {:?}", stack.pop());
    println!("Pop element: {:?}", stack.pop());
    println!("Top element: {:?}", stack.peek());
    println!("Er din stack tom? {}", stack.is_empty());

    println!("\n");
    // Den her stack er til string elementer
    let mut stack_string = Stack{elements: Vec::new(), max_size: 5};
    println!("Er din stack tom? {}", stack_string.is_empty());

    stack_string.push("First element");
    stack_string.push("Second element");
    stack_string.read_elements();
    println!("Er din stack tom? {}", stack_string.is_empty());
    println!("Top element: {:?}", stack_string.peek());
    println!("Pop element: {:?}", stack_string.pop());
    println!("Top element: {:?}", stack_string.peek());
    println!("Er din stack tom? {}", stack_string.is_empty());
}