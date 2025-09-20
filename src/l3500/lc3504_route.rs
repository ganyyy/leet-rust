use std::collections::{HashMap, HashSet, VecDeque};

struct Router {
    memory_limit: i32,
    packets: VecDeque<(i32, i32, i32)>, // (source, destination, timestamp)
    packet_set: HashSet<(i32, i32, i32)>,
    destination_map: HashMap<i32, Vec<i32>>, // destination -> timestamps
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memoryLimit: i32) -> Self {
        Router {
            memory_limit: memoryLimit,
            packets: VecDeque::new(),
            packet_set: HashSet::new(),
            destination_map: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = (source, destination, timestamp);
        if !self.packet_set.insert(packet) {
            return false;
        }
        if self.packets.len() as i32 >= self.memory_limit {
            self.forward_packet();
        }
        self.packets.push_back(packet);
        self.destination_map
            .entry(destination)
            .or_default()
            .push(timestamp);

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(packet) = self.packets.pop_front() {
            self.packet_set.remove(&packet);
            let (source, destination, timestamp) = packet;
            self.destination_map
                .get_mut(&destination)
                .map(|timestamps| timestamps.remove(0));
            vec![source, destination, timestamp]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(timestamps) = self.destination_map.get(&destination) {
            let left = timestamps.partition_point(|&x| x < start_time);
            let right = timestamps.partition_point(|&x| x <= end_time);
            (right - left) as i32
        } else {
            0
        }
    }
}
