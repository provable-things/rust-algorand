pub(crate) mod compact_certificate_state;

use std::{collections::HashMap, str::FromStr};

use crate::algorand_compact_certificates::compact_certificate_state::CompactCertificateState;

pub type CompactCertificates = HashMap<u64, CompactCertificateState>;
