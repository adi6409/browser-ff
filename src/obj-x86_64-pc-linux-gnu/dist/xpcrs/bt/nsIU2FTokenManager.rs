//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webauthn/nsIU2FTokenManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIU2FTokenManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resumeRegister (in uint64_t aTransactionID, in bool aForceNoneAttestation); */
                    Method {
                        name: "ResumeRegister",
                        params: &[Param { name: "aTransactionID", ty: "uint64_t" }, Param { name: "aForceNoneAttestation", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancel (in uint64_t aTransactionID); */
                    Method {
                        name: "Cancel",
                        params: &[Param { name: "aTransactionID", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

