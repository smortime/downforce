
#[derive(PartialEq, Debug)]
enum Status {
    Available,
    Busy,
    Failed,
}

struct WorkerStatus {
    port: i16,
    status: Status,
}

impl WorkerStatus {
    fn fail(&mut self) {
        self.status = Status::Failed
    }

    fn busy(&mut self) {
        self.status = Status::Busy
    }

    fn available(&mut self) {
        self.status = Status::Available
    }
}

struct Scoreboard {
    id: i16,
    workers: Vec<WorkerStatus>,
}

impl Scoreboard {
    fn new(id: i16, ports: &[i16]) -> Self {
        Scoreboard {
            id,
            workers: Scoreboard::new_worker_status_vec(ports),
        }
    }

    fn new_worker_status_vec(ports: &[i16]) -> Vec<WorkerStatus> {
        ports.iter().map(|&port| WorkerStatus{port, status: Status::Available}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::SeekFrom::Start;

    #[test]
    fn validate_scoreboard_new() {
        let ports = vec![5001, 5002, 5003];
        let mut scoreboard = Scoreboard::new(1, &ports);
        assert_eq!(scoreboard.id, 1);
        assert_eq!(scoreboard.workers.len(), ports.len());
    }

    #[test]
    fn validate_worker_status() {
        let mut worker = WorkerStatus {port: 5001, status: Status::Available};
        assert_eq!(worker.status, Status::Available);
        worker.fail();
        assert_eq!(worker.status, Status::Failed);
        worker.busy();
        assert_eq!(worker.status, Status::Busy);
    }
}