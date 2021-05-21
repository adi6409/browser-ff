//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertStorage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertStorageCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void done (in nsresult rv, in nsIVariant result); */
                    Method {
                        name: "Done",
                        params: &[Param { name: "rv", ty: "::nserror::nsresult" }, Param { name: "result", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIRevocationState",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute short state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIIssuerAndSerialRevocationState",
            base: Some("nsIRevocationState"),
            methods: Ok(&[
                    /* readonly attribute ACString issuer; */
                    Method {
                        name: "GetIssuer",
                        params: &[Param { name: "aIssuer", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString serial; */
                    Method {
                        name: "GetSerial",
                        params: &[Param { name: "aSerial", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISubjectAndPubKeyRevocationState",
            base: Some("nsIRevocationState"),
            methods: Ok(&[
                    /* readonly attribute ACString subject; */
                    Method {
                        name: "GetSubject",
                        params: &[Param { name: "aSubject", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString pubKey; */
                    Method {
                        name: "GetPubKey",
                        params: &[Param { name: "aPubKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICRLiteState",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString subject; */
                    Method {
                        name: "GetSubject",
                        params: &[Param { name: "aSubject", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString spkiHash; */
                    Method {
                        name: "GetSpkiHash",
                        params: &[Param { name: "aSpkiHash", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute short state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICertInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString cert; */
                    Method {
                        name: "GetCert",
                        params: &[Param { name: "aCert", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString subject; */
                    Method {
                        name: "GetSubject",
                        params: &[Param { name: "aSubject", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute short trust; */
                    Method {
                        name: "GetTrust",
                        params: &[Param { name: "aTrust", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICertStorage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void hasPriorData (in octet type, in nsICertStorageCallback callback); */
                    Method {
                        name: "HasPriorData",
                        params: &[Param { name: "type_", ty: "u8" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setRevocations (in Array<nsIRevocationState> revocations, in nsICertStorageCallback callback); */
                    Method {
                        name: "SetRevocations",
                        params: &[Param { name: "revocations", ty: "*const thin_vec::ThinVec<RefPtr<nsIRevocationState>>" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] short getRevocationState (in Array<octet> issuer, in Array<octet> serial, in Array<octet> subject, in Array<octet> pubkey); */
                    Method {
                        name: "GetRevocationState",
                        params: &[Param { name: "issuer", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "serial", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "subject", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "pubkey", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean isBlocklistFresh (); */
                    Method {
                        name: "IsBlocklistFresh",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setCRLiteState (in Array<nsICRLiteState> crliteState, in nsICertStorageCallback callback); */
                    Method {
                        name: "SetCRLiteState",
                        params: &[Param { name: "crliteState", ty: "*const thin_vec::ThinVec<RefPtr<nsICRLiteState>>" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] short getCRLiteState (in Array<octet> subject, in Array<octet> spki); */
                    Method {
                        name: "GetCRLiteState",
                        params: &[Param { name: "subject", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "spki", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setFullCRLiteFilter (in Array<octet> filter, in uint64_t timestamp, in nsICertStorageCallback callback); */
                    Method {
                        name: "SetFullCRLiteFilter",
                        params: &[Param { name: "filter", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "timestamp", ty: "uint64_t" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] short getCRLiteRevocationState (in Array<octet> issuer, in Array<octet> issuerSPKI, in Array<octet> serialNumber, out uint64_t validBefore); */
                    Method {
                        name: "GetCRLiteRevocationState",
                        params: &[Param { name: "issuer", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "issuerSPKI", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "serialNumber", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "validBefore", ty: "*mut uint64_t" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void addCRLiteStash (in Array<octet> stash, in nsICertStorageCallback callback); */
                    Method {
                        name: "AddCRLiteStash",
                        params: &[Param { name: "stash", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] bool isCertRevokedByStash (in Array<octet> issuerSPKI, in Array<octet> serialNumber); */
                    Method {
                        name: "IsCertRevokedByStash",
                        params: &[Param { name: "issuerSPKI", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "serialNumber", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void addCerts (in Array<nsICertInfo> certs, in nsICertStorageCallback callback); */
                    Method {
                        name: "AddCerts",
                        params: &[Param { name: "certs", ty: "*const thin_vec::ThinVec<RefPtr<nsICertInfo>>" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void removeCertsByHashes (in Array<ACString> hashes, in nsICertStorageCallback callback); */
                    Method {
                        name: "RemoveCertsByHashes",
                        params: &[Param { name: "hashes", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "callback", ty: "*const nsICertStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] Array<Array<octet>> findCertsBySubject (in Array<octet> subject); */
                    Method {
                        name: "FindCertsBySubject",
                        params: &[Param { name: "subject", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<thin_vec::ThinVec<u8>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] int32_t GetRemainingOperationCount (); */
                    Method {
                        name: "GetRemainingOperationCount",
                        params: &[Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

