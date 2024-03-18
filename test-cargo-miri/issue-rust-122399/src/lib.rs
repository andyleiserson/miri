const OID_RSA_ENCRYPTION: &[u64] = &[1, 2, 840, 113549, 1, 1, 1];

pub struct SignatureAlgorithm {
    pub oids: &'static [&'static [u64]],
}

pub static PKCS_RSA_SHA256: SignatureAlgorithm = SignatureAlgorithm {
    // Does not repro with this constant inlined.
    oids: &[&OID_RSA_ENCRYPTION],
};
