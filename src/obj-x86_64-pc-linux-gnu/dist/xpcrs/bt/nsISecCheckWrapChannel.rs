//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISecCheckWrapChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecCheckWrapChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIChannel innerChannel; */
                    Method {
                        name: "GetInnerChannel",
                        params: &[Param { name: "aInnerChannel", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

