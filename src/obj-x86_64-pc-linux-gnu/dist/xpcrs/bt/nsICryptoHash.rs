//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICryptoHash.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICryptoHash",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in unsigned long aAlgorithm); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aAlgorithm", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void initWithString (in ACString aAlgorithm); */
                    Method {
                        name: "InitWithString",
                        params: &[Param { name: "aAlgorithm", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
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

                    /* ACString finish (in boolean aASCII); */
                    Method {
                        name: "Finish",
                        params: &[Param { name: "aASCII", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

