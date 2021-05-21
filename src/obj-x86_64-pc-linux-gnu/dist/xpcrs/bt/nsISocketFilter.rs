//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISocketFilter",
            base: Some("nsISupports"),
            methods: Err("native type mozilla::net::NetAddr unsupported"),
        },

        Interface {
            name: "nsISocketFilterHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISocketFilter newFilter (); */
                    Method {
                        name: "NewFilter",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISocketFilter" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

