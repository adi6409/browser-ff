//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITimedChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServerTiming",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute ACString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute double duration; */
                    Method {
                        name: "GetDuration",
                        params: &[Param { name: "aDuration", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString description; */
                    Method {
                        name: "GetDescription",
                        params: &[Param { name: "aDescription", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITimedChannel",
            base: Some("nsISupports"),
            methods: Err("Rust only supports [ref] / [ptr] native types"),
        },

        ]; D}

