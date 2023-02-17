use crate::structs::Node;

#[derive(Clone, Copy)]
struct QueueEntry{
    self_node: usize,
    previous_node: usize,
    min_distance: i32
}

struct FibHeap{
    min: usize,
    map: Vec<FibBranch>,
    num_nodes: u32
}

impl FibHeap{
    fn find_min(&self) -> QueueEntry{
        let entry = self.map[self.min].entry;
        return entry;
    }
    
    fn merge(&mut self){
        let max_degree = (self.num_nodes as f32).log2() as usize;
        let mut degree_trees: Vec<usize> = vec![usize::MAX; max_degree + 1];

        // Iterate through branches and merge if necessary
        let mut index = self.map[self.min].next;
        let mut start = self.min;
        
        while index != start{
            if degree_trees[self.map[index].degree] == usize::MAX || degree_trees[self.map[index].degree] == index{
                degree_trees[self.map[index].degree] = index;
                index = self.map[index].next;
                continue;
            }
            
            // Merge
            let mut min = index;
            let mut max = degree_trees[self.map[index].degree];
            if min > max{
                let temp = min;
                min = max;
                max = temp;
            }
            
            // Remove bigger node form root list
            let root_next = self.map[max].next;
            let root_prev = self.map[max].prev; 
            self.map[root_next].prev = root_prev;
            self.map[root_prev].next = root_next;

            // Integrate bigger node as child of smaller node
            let mut prev = max;
            let mut next = max;

            if self.map[min].child.is_some(){
                let child = self.map[min].child.unwrap();
                prev = child;
                next = self.map[child].next;
            }

            self.map[max].next = next;
            self.map[max].prev = prev;
            self.map[max].parent = Some(min);

            self.map[next].prev = max;
            self.map[prev].next = max;

            self.map[min].degree += 1;
            self.map[min].child = Some(max);

            start = min;
            index = self.map[start].next;
            degree_trees[self.map[min].degree] = min;
            degree_trees[self.map[min].degree - 1] = usize::MAX;
        }
        // Reorder root list
        let mut root_list: Vec<usize> = Vec::new();
        for i in 0..degree_trees.len(){
            if degree_trees[i] == usize::MAX{
                continue;
            }
            root_list.push(degree_trees[i]);
            if self.map[degree_trees[i]].entry.min_distance < self.map[self.min].entry.min_distance{
                self.min = degree_trees[i];
                println!("{}", self.min);
            }
        }
        for i in 0..root_list.len(){
            let index = root_list[i];
            let next = root_list[(i + 1) % root_list.len()];
            self.map[index].next = next;
            self.map[next].prev = index;
        }
    }
    
    fn delete_min(&mut self){
        self.num_nodes -= 1;
        let index = self.map[self.min].child;
        
        // Delete min from root list
        let next = self.map[self.min].next;
        let prev = self.map[self.min].prev;
        self.map[next].prev = prev;
        self.map[prev].next = next;

        if index.is_some(){
            let mut index = index.unwrap();
            // Delete child-parent link
            while self.map[index].parent != None {
                self.map[index].parent = None;
                index = self.map[index].next;
            }
            

            // Integrate first and last child with root list
            let end = self.map[index].prev;

            self.map[end].next = self.map[self.min].next;
            let next = self.map[end].next;
            self.map[next].prev = end;

            self.map[index].prev = self.map[self.min].prev;
            let prev = self.map[index].prev;
            self.map[prev].next = index;
        }

        
        // Merge and get new min
        self.min = next;
        self.merge();

    }

    fn insert(&mut self, entry: QueueEntry){
        self.map.push(FibBranch{
            entry: entry,
            prev: self.min,
            next: self.map[self.min].next,
            child: None,
            parent: None,
            degree: 0,
            marked: false
        });
        let index = self.num_nodes as usize;
        self.map[self.min].next = index;
        let next = self.map[index].next;
        self.map[next].prev = index;
        self.num_nodes += 1;

        if self.map[index].entry.min_distance < self.map[self.min].entry.min_distance{
            self.min = index;
        }
    }

    fn decrease_key(&mut self, index: usize, distance: i32){
        self.map[index].entry.min_distance = distance;
        if distance < self.map[self.min].entry.min_distance{
            self.min = index;
        }
        if self.map[index].parent.is_none(){
            return;
        }
        if self.map[self.map[index].parent.unwrap()].entry.min_distance <= self.map[index].entry.min_distance{
            return;
        }
        self.prune_branch(index);
    }

    fn prune_branch(&mut self, index: usize){
        let parent = self.map[index].parent.unwrap();

        // Change child to none or next if the pruned branch is the child
        if self.map[parent].child.unwrap() == index{
            if self.map[index].next == index{
                self.map[parent].child = None;
            }else{
                self.map[parent].child = Some(self.map[index].next);
            }
        }
        // Delete pruned branch from the child doubly linked list
        let next = self.map[index].next;
        let prev = self.map[index].prev;   
        self.map[next].prev = prev;
        self.map[prev].next = next;

        // Integrate pruned branch into the root list
        self.map[index].next = self.map[self.min].next;
        self.map[index].prev = self.min;

        self.map[self.min].next = index;
        let next = self.map[index].next;
        self.map[next].prev = index;

        // Mark parent and prune it if it is already marked and has parent
        if self.map[parent].marked{
            self.map[parent].marked = false;
            if self.map[parent].parent.is_some(){
                self.prune_branch(parent);
            }
            return;
        }
        self.map[parent].marked = true;
    }

    fn debug(&self, start: usize, pre: String){
        let mut index = start;

        let pre_slice = &pre[..];
        let mut i = 0;
        loop{
            println!("{}Node #{}, distance: {}, next: {}, prev: {}"
            , pre_slice, index, self.map[index].entry.min_distance, self.map[index].next, self.map[index].prev);
            if self.map[index].child.is_some(){
                self.debug(self.map[index].child.unwrap(), (pre_slice.to_owned() + "\t").to_owned());
            }
            index = self.map[index].next;
            i += 1;
            if start == index || i >= 10{
                break;
            }
        }
    }
}


struct FibBranch{
    entry: QueueEntry,
    prev: usize,
    next: usize,
    child: Option<usize>,
    parent: Option<usize>,
    degree: usize,
    marked: bool
}

pub fn solve(nodes: &Vec<Node>, start_node: usize, end_node: usize) -> (Vec<usize>, i32){
    let mut priority_queue = FibHeap{
        min: 0,
        map: vec![
        FibBranch{
            entry: QueueEntry{
                self_node: 0,
                previous_node: usize::MAX,
                min_distance: 0
            },
            prev: 0,
            next: 0,
            child: None,
            parent: None,
            degree: 0,
            marked: false
        }],
        num_nodes: 1
    };

    // Initialize priority queue
    for i in 1..nodes.len(){
        let temp = QueueEntry{
            self_node: i,
            previous_node: start_node,
            min_distance: i32::MAX
        };
        priority_queue.insert(temp);
    }

    return dijkstra(nodes, priority_queue, end_node);
}

fn dijkstra(nodes: &Vec<Node>, mut priority_queue: FibHeap, end_node: usize) -> (Vec<usize>, i32){
    let main_node;
    let mut main_entry: QueueEntry;

    // Get first non-visited entry
    main_entry = priority_queue.find_min();
    main_node = main_entry.self_node;
    priority_queue.delete_min();
    priority_queue.debug(priority_queue.min, "".to_owned());
    println!("----------------------------------");
    

    // Return base case
    if main_node == end_node{
        // Initialize result
        let mut result: (Vec<usize>, i32);
        result = (
            vec![main_node],
            main_entry.min_distance
        );

        // Traceback path
        while main_entry.previous_node != usize::MAX{
            main_entry = priority_queue.map[main_entry.previous_node].entry;
            result.0.push(main_entry.self_node);
        }

        return result;
    }
    // Update min distance for main node's neighbours
    for i in 0..nodes[main_node].neighbours.len(){
        let mut neighbour_entry = priority_queue.map[nodes[main_node].neighbours[i]].entry;
        let distance = nodes[main_node].lengths[i] + main_entry.min_distance;
        if neighbour_entry.min_distance > distance {
            priority_queue.decrease_key(nodes[main_node].neighbours[i], distance);
            neighbour_entry.previous_node = nodes[main_node].neighbours[i];
        }
    }

    // Call function for next node
    return dijkstra(nodes, priority_queue, end_node);
}