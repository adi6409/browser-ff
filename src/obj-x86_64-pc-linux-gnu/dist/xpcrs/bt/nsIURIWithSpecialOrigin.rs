//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURIWithSpecialOrigin.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIWithSpecialOrigin",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIURI origin; */
                    Method {
                        name: "GetOrigin",
                        params: &[Param { name: "aOrigin", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

