//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISDBRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute nsIVariant result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsresult resultCode; */
                    Method {
                        name: "GetResultCode",
                        params: &[Param { name: "aResultCode", ty: "*mut ::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISDBCallback callback; */
                    Method {
                        name: "GetCallback",
                        params: &[Param { name: "aCallback", ty: "*mut*const nsISDBCallback" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCallback",
                        params: &[Param { name: "aCallback", ty: "*const nsISDBCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

