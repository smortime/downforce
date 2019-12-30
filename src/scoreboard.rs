
enum Status {
    Available,
    Busy,
    Failed,
}

struct Scoreboard {
    id: i16,
    worker_status: Vec<WorkerStatus>,
}

struct WorkerStatus {
    port: i16,
    status: Status,
}

impl Scoreboard {
    fn new(id: i16, ports: &[i16]) -> Self {
        Scoreboard {
            id,
            worker_status: Scoreboard::new_worker_status_vec(ports),
        }
    }

    fn new_worker_status_vec(ports: &[i16]) -> Vec<WorkerStatus> {
        ports.iter().map(|&port|WorkerStatus{port, status: Status::Available}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_scoreboard_new() {
        let ports = vec![5001, 5002, 5003];
        let scoreboard = Scoreboard::new(1, &ports);
        assert_eq!(scoreboard.id, 1);
        assert_eq!(scoreboard.worker_status.len(), ports.len());
    }
}