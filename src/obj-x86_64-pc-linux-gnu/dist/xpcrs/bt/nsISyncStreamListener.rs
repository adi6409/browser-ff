//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISyncStreamListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISyncStreamListener",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* readonly attribute nsIInputStream inputStream; */
                    Method {
                        name: "GetInputStream",
                        params: &[Param { name: "aInputStream", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

