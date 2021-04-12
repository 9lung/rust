/*
rust의 변수들은 owner라는 변수를 각각 가지고 있는데,
한번에 하나씩만 들고 있을 수 있다.
또한 owner가 scope를 벗어났을 때, value 또한 사라지게 된다.(drop)
이때, linkedlist에서 어떤 변수값을 쓰기 위해서는 직접 접근을 하게 되고
이에 따라서 owner가 변하는데, caller를 벗어나는 순간 값이 사라지게 된다.
따라서 그 값을 더이상 사용할 수 없게 되는데, safe로 구현함에 있어서
값의 호출은 빈번하게 발생하게 되는데, ownership으로 인해 direct로 구현하게
되면 불가능해지게 된다.  
*/
mod linked_list;

struct Cbuffer<T> {
    list : linked_list::LinkedList<T>,
    top : usize,
    bottom : usize,
    size : usize,
}

impl<T> Cbuffer<T> {
    fn new(size : usize) -> Self {
        Cbuffer {
            list : linked_list::LinkedList::new(),
            top : 0,
            bottom : 0,
            size : size,
        }
    }

    fn length(&self) -> usize {
        if self.bottom > self.top {
            self.top + self.size + 1 - self.bottom 
        }
        else {
            self.top - self.bottom
        }
    }

    fn enque(&mut self, data : T, forced : bool) {
        if self.length() == self.size {
            if forced {
                self.list.pop_back_value();
                self.list.push_front(data);
                self.bottom = (self.bottom + 1) % (self.size + 1);
                self.top = (self.top + 1) % (self.size + 1);
            }
        }
        else {
            self.list.push_front(data);
            self.top = (self.top + 1) % (self.size + 1);

            if self.length() == self.size {
                println!("now the buffer is full.");
            }
        }
    }

    fn deque(&mut self) -> T {
        if self.length() == 0 {
            panic!("try to deque from empty list");
        }
        else {
            self.bottom = (self.bottom + 1) % (self.size + 1);
            self.list.pop_back_value()
        }
    }

    fn peek(&mut self) -> &T {
        self.list.peek()
    }

    fn iter(&mut self) -> linked_list::Iter<T> {
        self.list.iter()
    }

}

fn main() {
    let mut my_buffer : Cbuffer<i32> = Cbuffer::new(10);
    let mut num = 0;
    // let mut iter = my_buffer.iter();
    loop {
        my_buffer.enque(num, true);
        println!("{}", *my_buffer.peek());
        num += 1;

        if num == 15 {
            break;
        }
    }
    loop {
        println!("{}", my_buffer.deque());
    }
}