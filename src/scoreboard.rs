use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum WorkerStatus {
    Available,
    Busy,
    Failed,
}

struct Scoreboard {
    id: i16,
    workers: HashMap<i32, WorkerStatus>,
}

impl Scoreboard {
    fn new(id: i16, ports: &[i32]) -> Self {
        Scoreboard {
            id,
            workers: Scoreboard::new_worker_status_vec(ports),
        }
    }

    fn new_worker_status_vec(ports: &[i32]) -> HashMap<i32, WorkerStatus> {
        let mut scoreboard_map = HashMap::new();

        for port in ports {
            scoreboard_map.insert(*port, WorkerStatus::Available);
        }

        scoreboard_map
    }

    // TODO: make legit scheduling... someday
    fn get_available_worker(&mut self) -> Option<i32> {
        for (k, v) in &mut self.workers {
            if *v == WorkerStatus::Available {
                *v = WorkerStatus::Busy;
                return Option::Some(*k);
            }
        }
        None
    }

    fn mark_worker_available(&mut self, port: i32) {
        if let Some(x) = self.workers.get_mut(&port) {
            *x = WorkerStatus::Available;
        }
    }

    fn mark_worker_failed(&mut self, port: i32) {
        if let Some(x) = self.workers.get_mut(&port) {
            *x = WorkerStatus::Failed;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::SeekFrom::Start;

    #[test]
    fn validate_scoreboard_new() {
        let ports = vec![5001, 5002, 5003];
        let scoreboard = Scoreboard::new(1, &ports);
        assert_eq!(scoreboard.id, 1);
        assert_eq!(scoreboard.workers.len(), ports.len());

        for (k, v) in scoreboard.workers.iter() {
            assert_eq!(*v, WorkerStatus::Available);
        }
    }

    #[test]
    fn validate_get_available_worker() {
        let ports = vec![5001, 5002, 5003];
        let mut scoreboard = Scoreboard::new(1, &ports);

        let worker_1 = scoreboard.get_available_worker().unwrap();
        let worker_2 = scoreboard.get_available_worker().unwrap();
        let worker_3 = scoreboard.get_available_worker().unwrap();

        assert!(worker_1 != worker_2, "Workers 1 and 2 should not be equal");
        assert!(worker_1 != worker_3, "Workers 1 and 3 should not be equal");
        assert!(worker_3 != worker_2, "Workers 2 and 3 should not be equal");

        let none_worker = scoreboard.get_available_worker();
        assert!(none_worker.is_none(), "Should return None");

        scoreboard.mark_worker_available(worker_1);
        assert_eq!(*scoreboard.workers.get_mut(&worker_1).unwrap(), WorkerStatus::Available);
        scoreboard.mark_worker_failed(worker_2);
        assert_eq!(*scoreboard.workers.get_mut(&worker_2).unwrap(), WorkerStatus::Failed);
    }
}