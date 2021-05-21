//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIWellKnownOpportunisticUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWellKnownOpportunisticUtils",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void verify (in ACString aJSON, in ACString aOrigin); */
                    Method {
                        name: "Verify",
                        params: &[Param { name: "aJSON", ty: "*const ::nsstring::nsACString" }, Param { name: "aOrigin", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute bool valid; */
                    Method {
                        name: "GetValid",
                        params: &[Param { name: "aValid", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

