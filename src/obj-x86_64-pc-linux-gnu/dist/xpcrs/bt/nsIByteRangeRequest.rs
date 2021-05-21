//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIByteRangeRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIByteRangeRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean isByteRangeRequest; */
                    Method {
                        name: "GetIsByteRangeRequest",
                        params: &[Param { name: "aIsByteRangeRequest", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long startRange; */
                    Method {
                        name: "GetStartRange",
                        params: &[Param { name: "aStartRange", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long endRange; */
                    Method {
                        name: "GetEndRange",
                        params: &[Param { name: "aEndRange", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

