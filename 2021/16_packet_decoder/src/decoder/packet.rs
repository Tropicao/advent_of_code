pub struct Packet {
    version: usize,
    packet_type: PacketType,
    childs: Vec<Self>,
    value: usize,
}

#[derive(PartialEq, Debug)]
pub enum PacketType {
    LITERAL,
    SUM,
    PRODUCT,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

impl Packet {
    pub fn new(version: usize, packet_type: PacketType) -> Self {
        Packet {
            version,
            packet_type,
            childs: vec![],
            value: 0,
        }
    }

    pub fn set_type(&mut self, new_packet_type: PacketType) {
        self.packet_type = new_packet_type;
    }

    pub fn append_child(&mut self, child: Self) {
        self.childs.push(child);
    }

    pub fn set_value(mut self, value: usize) -> Self {
        self.update_value(value);
        self
    }

    pub fn update_value(&mut self, value: usize) {
        if self.packet_type != PacketType::LITERAL {
            panic!("Tried to set value on a non-litteral packet");
        }
        self.value = value;
    }

    pub fn value(&self) -> usize {
        match self.packet_type {
            PacketType::LITERAL => self.value,
            PacketType::SUM => self.childs.iter().map(|c| c.value()).sum(),
            PacketType::PRODUCT => self.childs.iter().map(|c| c.value()).product(),
            PacketType::MIN => self.childs.iter().map(|c| c.value()).min().unwrap(),
            PacketType::MAX => self.childs.iter().map(|c| c.value()).max().unwrap(),
            PacketType::LT => if self.childs[0].value() < self.childs[1].value() {1} else {0},
            PacketType::GT => if self.childs[0].value() > self.childs[1].value() {1} else {0},
            PacketType::EQ => if self.childs[0].value() == self.childs[1].value() {1} else {0}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::packet::PacketType;

    use super::Packet;

    #[test]
    fn test_packet_version() {
        let p = Packet::new(6, PacketType::LITERAL);
        assert_eq!(p.version, 6);
    }

    #[test]
    fn test_packet_type() {
        let p = Packet::new(6, PacketType::LITERAL);
        assert_eq!(p.packet_type, PacketType::LITERAL);
    }

    #[test]
    fn test_packet_type_bis() {
        let p = Packet::new(6, PacketType::SUM);
        assert_eq!(p.packet_type, PacketType::SUM);
    }

    #[test]
    fn test_append_child() {
        let mut p = Packet::new(6, PacketType::SUM);
        p.append_child(Packet::new(5, PacketType::LITERAL));
        p.append_child(Packet::new(4, PacketType::LITERAL));
        assert_eq!(p.childs.len(), 2);
        assert_eq!(p.childs[0].childs.len(), 0);
        assert_eq!(p.childs[1].childs.len(), 0);
    }

    #[test]
    fn test_sum() {
        let mut p = Packet::new(6, PacketType::SUM);
        p.append_child(Packet::new(5, PacketType::LITERAL).set_value(1));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(2));
        assert_eq!(p.value(), 3);
    }

    #[test]
    fn test_product() {
        let mut p = Packet::new(6, PacketType::PRODUCT);
        p.append_child(Packet::new(5, PacketType::LITERAL).set_value(6));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(9));
        assert_eq!(p.value(), 54);
    }

    #[test]
    fn test_min() {
        let mut p = Packet::new(6, PacketType::MIN);
        p.append_child(Packet::new(5, PacketType::LITERAL).set_value(7));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(8));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(9));
        assert_eq!(p.value(), 7);
    }

    #[test]
    fn test_max() {
        let mut p = Packet::new(6, PacketType::MAX);
        p.append_child(Packet::new(5, PacketType::LITERAL).set_value(7));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(8));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(9));
        assert_eq!(p.value(), 9);
    }

    #[test]
    fn test_lt() {
        let mut p = Packet::new(6, PacketType::LT);
        p.append_child(Packet::new(5, PacketType::LITERAL).set_value(5));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(15));
        assert_eq!(p.value(), 1);
    }

    #[test]
    fn test_gt() {
        let mut p = Packet::new(6, PacketType::GT);
        p.append_child(Packet::new(5, PacketType::LITERAL).set_value(5));
        p.append_child(Packet::new(4, PacketType::LITERAL).set_value(15));
        assert_eq!(p.value(), 0);
    }

    #[test]
    fn test_eq() {
        let mut p = Packet::new(6, PacketType::EQ);
        let mut p1 = Packet::new(6, PacketType::SUM);
        p1.append_child(Packet::new(5, PacketType::LITERAL).set_value(1));
        p1.append_child(Packet::new(4, PacketType::LITERAL).set_value(3));
        let mut p2 = Packet::new(6, PacketType::PRODUCT);
        p2.append_child(Packet::new(5, PacketType::LITERAL).set_value(2));
        p2.append_child(Packet::new(4, PacketType::LITERAL).set_value(2));
        p.append_child(p1);
        p.append_child(p2);
        assert_eq!(p.value(), 1);
    }
}
