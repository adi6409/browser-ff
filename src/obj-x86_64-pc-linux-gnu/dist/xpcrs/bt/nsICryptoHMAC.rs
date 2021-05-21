//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICryptoHMAC.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICryptoHMAC",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void init (in unsigned long aAlgorithm, in nsIKeyObject aKeyObject); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aAlgorithm", ty: "u32" }, Param { name: "aKeyObject", ty: "*const nsIKeyObject" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
                    Method {
                        name: "Update",
                        params: &[Param { name: "aData", ty: "*const u8" }, Param { name: "aLen", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void updateFromStream (in nsIInputStream aStream, in unsigned long aLen); */
                    Method {
                        name: "UpdateFromStream",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aLen", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] ACString finish (in boolean aASCII); */
                    Method {
                        name: "Finish",
                        params: &[Param { name: "aASCII", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void reset (); */
                    Method {
                        name: "Reset",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

