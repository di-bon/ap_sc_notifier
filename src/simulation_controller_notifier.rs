use std::fmt::{Debug, Formatter};
use crossbeam_channel::Sender;
use messages::node_event::NodeEvent;

pub struct SimulationControllerNotifier {
    simulation_controller_tx: Sender<NodeEvent>,
}

impl Debug for SimulationControllerNotifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimulationControllerNotifier")
    }
}

impl SimulationControllerNotifier {
    /// Returns a new instance of `SimulationControllerNotifier`
    #[must_use]
    pub fn new(simulation_controller_tx: Sender<NodeEvent>) -> Self {
        Self {
            simulation_controller_tx,
        }
    }

    /// Sends a `NodeEvent` into the channel shared with the simulation controller
    /// # Panics
    /// Panics if the transmission fails
    pub fn send_event(&self, node_event: NodeEvent) {
        match self.simulation_controller_tx.send(node_event) {
            Ok(()) => log::info!("Node event sent"),
            Err(err) => {
                let error = format!("Cannot send events to simulation controller. Error: {err:?}");
                log::error!("{error}");
                panic!("{error}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;
    use wg_2024::packet::{Ack, Packet, PacketType};
    use super::*;

    #[test]
    fn initialize() {
        let (tx, rx) = unbounded();
        let notifier = SimulationControllerNotifier::new(tx);

        let notifier = format!("{notifier:?}");
        assert_eq!(notifier, "SimulationControllerNotifier");
    }

    #[test]
    fn test_send_event() {
        let (tx, rx) = unbounded();
        let notifier = SimulationControllerNotifier::new(tx);

        let event = NodeEvent::PacketSent(
            Packet {
                routing_header: Default::default(),
                session_id: 0,
                pack_type: PacketType::Ack(
                    Ack {
                        fragment_index: 0,
                    }
                ),
            }
        );
        notifier.send_event(event.clone());

        let received = rx.recv().unwrap();
        assert!(matches!(received, NodeEvent::PacketSent(_)));
    }

    #[test]
    #[should_panic]
    fn test_failure() {
        let (tx, rx) = unbounded();
        let notifier = SimulationControllerNotifier::new(tx);

        let event = NodeEvent::PacketSent(
            Packet {
                routing_header: Default::default(),
                session_id: 0,
                pack_type: PacketType::Ack(
                    Ack {
                        fragment_index: 0,
                    }
                ),
            }
        );

        drop(rx);
        notifier.send_event(event.clone());
    }
}